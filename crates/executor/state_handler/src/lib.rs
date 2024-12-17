//! A state handler that can be used to store and retrieve state.
mod event;
mod snapshot_state;

use crate::event::EventState;
use crate::snapshot_state::{Snapshot, SnapshotState};
use allocator_api2::alloc::{Allocator, Global};
use allocator_api2::vec::Vec;
use ixc_account_manager::state_handler::std::{StdStateError, StdStateManager};
use ixc_message_api::{code::ErrorCode, AccountID};

/// A store that can be used to store and retrieve state.
pub trait Store {
    /// Get the value for the given key.
    fn get<A: Allocator>(&self, key: &Vec<u8>, allocator: A) -> Option<Vec<u8, A>>;
}

/// StateHandler is a cache-based state handler that can be used to store and retrieve state.
pub struct StateHandler<S: Store> {
    snapshot_state: SnapshotState<S>,
    /// Checkpoints are used to revert state changes.
    /// checkpoints are used to follow the call stack of a transaction
    /// and revert the state changes when the transaction is rolled back.
    checkpoints: Vec<Snapshot>,
    event_state: EventState,
}

impl<S: Store> StateHandler<S> {
    /// Create a new state handler with the given store.
    pub fn new(store: S) -> Self {
        Self {
            snapshot_state: SnapshotState::new(store),
            checkpoints: Vec::with_capacity_in(200, Global),
            event_state: EventState::new(),
        }
    }

    pub(crate) fn construct_key(
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        accumulator: bool,
    ) -> Vec<u8> {
        const KV_SEPARATOR: u8 = 0;
        const KV_SCOPED_SEPARATOR: u8 = 1;
        const ACC_SEPARATOR: u8 = 2;
        const SCOPED_ACC_SEPARATOR: u8 = 3;
        match scope {
            // account / 0 / key
            // account / 1 / scope / key
            // account / 2 (accumulator) / key
            // account / 3 (scoped accumulator) / scope / key
            Some(scope) => {
                let ac = account_id.to_le_bytes();
                let sc = scope.to_le_bytes();
                let mut new_key = Vec::new_in(Global);
                new_key.extend_from_slice(&ac);
                if accumulator {
                    new_key.push(SCOPED_ACC_SEPARATOR);
                } else {
                    new_key.push(KV_SCOPED_SEPARATOR);
                }
                new_key.extend_from_slice(&sc);
                new_key.extend_from_slice(key);
                new_key
            }
            None => {
                let mut new_key = Vec::new_in(Global);
                new_key.extend_from_slice(&account_id.to_le_bytes());
                if accumulator {
                    new_key.push(ACC_SEPARATOR);
                } else {
                    new_key.push(KV_SEPARATOR);
                }
                new_key.extend_from_slice(key);
                new_key
            }
        }
    }
}

impl<S: Store> StdStateManager for StateHandler<S> {
    fn kv_get<A: Allocator>(
        &self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        allocator: A,
    ) -> Result<Option<Vec<u8, A>>, StdStateError> {
        let constructed_key = Self::construct_key(account_id, scope, key, false);

        match self.snapshot_state.get(&constructed_key, allocator) {
            Some(value) => Ok(Some(value)),
            None => Ok(None),
        }
    }

    fn kv_set(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        value: &[u8],
    ) -> Result<(), StdStateError> {
        let constructed_key = Self::construct_key(account_id, scope, key, false);
        let mut vec = Vec::new(); //TODO allocations occur here
        vec.extend_from_slice(value);
        self.snapshot_state.set(constructed_key, &vec);
        Ok(())
    }

    fn kv_delete(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
    ) -> Result<(), StdStateError> {
        let constructed_key = Self::construct_key(account_id, scope, key, false);
        self.snapshot_state.delete(&constructed_key);
        Ok(())
    }

    fn accumulator_get(
        &self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
    ) -> Result<u128, StdStateError> {
        let constructed_key = Self::construct_key(account_id, scope, key, true);

        match self.snapshot_state.get(&constructed_key, Global) {
            Some(value) => {
                let mut data = [0u8; 16];
                data.copy_from_slice(&value);
                Ok(u128::from_le_bytes(data))
            }
            None => Ok(0),
        }
    }

    fn accumulator_add(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        value: u128,
    ) -> Result<(), StdStateError> {
        let constructed_key = Self::construct_key(account_id, scope, key, true);

        let bz = self.snapshot_state.get(&constructed_key, Global);
        let old_value: u128 = match bz {
            Some(value) => {
                let mut data = [0u8; 16];
                data.copy_from_slice(&value);
                let ov: u128 = u128::from_le_bytes(data);
                ov
            }
            None => 0,
        };

        let new_value = old_value
            .checked_add(value)
            .ok_or(StdStateError::from(ErrorCode::HandlerCode(0)))?;

        let mut vec = Vec::new();
        vec.extend_from_slice(&new_value.to_le_bytes());
        self.snapshot_state.set(constructed_key, &vec);
        Ok(())
    }

    fn accumulator_safe_sub(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        value: u128,
    ) -> Result<bool, StdStateError> {
        let constructed_key = Self::construct_key(account_id, scope, key, true);
        let bz = self.snapshot_state.get(&constructed_key, Global);

        let old_value: u128 = match bz {
            Some(value) => {
                let mut data = [0u8; 16];
                data.copy_from_slice(&value);
                let ov: u128 = u128::from_le_bytes(data);
                ov
            }
            None => 0,
        };

        let new_value = old_value
            .checked_sub(value)
            .ok_or(StdStateError::from(ErrorCode::HandlerCode(0)))?;

        let mut vec = Vec::new();
        vec.extend_from_slice(&new_value.to_le_bytes());
        self.snapshot_state.set(constructed_key, &vec);
        Ok(true)
    }

    /// Begins a new transaction.
    fn begin_tx(&mut self) -> Result<(), StdStateError> {
        self.checkpoints.push(self.snapshot_state.snapshot());
        self.event_state.snapshot();
        Ok(())
    }

    /// Commits the current transaction.
    fn commit_tx(&mut self) -> Result<(), StdStateError> {
        self.checkpoints.pop();
        self.event_state.commit();
        Ok(())
    }

    /// Rolls back the current transaction.
    fn rollback_tx(&mut self) -> Result<(), StdStateError> {
        let snapshot = self
            .checkpoints
            .pop()
            .ok_or(StdStateError::FatalExecutionError)?;
        let _ = self.snapshot_state.revert_to_snapshot(snapshot);
        self.event_state.revert_to_snapshot(self.checkpoints.len());
        Ok(())
    }

    /// Create storage for a new account.
    fn create_account_storage(&mut self, _account: AccountID) -> Result<(), StdStateError> {
        Ok(())
    }

    /// Delete all of an account's storage.
    fn delete_account_storage(&mut self, _account: AccountID) -> Result<(), StdStateError> {
        Ok(())
    }

    /// Emit an event.
    fn emit_event(&mut self, sender: AccountID, data: &[u8]) -> Result<(), StdStateError> {
        let data_vec = Vec::from(data);
        self.event_state.emit_event(sender, data_vec);
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_kv() {
        let store = HashMap::<Vec<u8>, Vec<u8>>::new();
        let mut state_handler = StateHandler::new(store);

        state_handler
            .kv_set(AccountID::new(1), None, b"key1", b"value1")
            .unwrap();
        state_handler
            .kv_set(AccountID::new(1), None, b"key2", b"value2")
            .unwrap();
        state_handler
            .kv_set(
                AccountID::new(1),
                Some(AccountID::new(2)),
                b"key3",
                b"value3",
            )
            .unwrap();
        state_handler
            .kv_set(
                AccountID::new(1),
                Some(AccountID::new(2)),
                b"key4",
                b"value4",
            )
            .unwrap();
        state_handler.commit_tx().unwrap();
        assert_eq!(
            state_handler
                .kv_get(AccountID::new(1), None, b"key1", Global)
                .unwrap()
                .unwrap(),
            b"value1"
        );
        assert_eq!(
            state_handler
                .kv_get(AccountID::new(1), None, b"key2", Global)
                .unwrap()
                .unwrap(),
            b"value2"
        );
    }

    #[test]
    fn test_accumulator() {
        let store = HashMap::<Vec<u8>, Vec<u8>>::new();
        let mut state_handler = StateHandler::new(store);

        assert_eq!(
            state_handler
                .accumulator_get(AccountID::new(1), None, b"key1")
                .unwrap(),
            0
        );
        // accumulator add
        state_handler
            .accumulator_add(AccountID::new(1), None, b"key1", 10)
            .unwrap();
        assert_eq!(
            state_handler
                .accumulator_get(AccountID::new(1), None, b"key1")
                .unwrap(),
            10
        );
        state_handler
            .accumulator_add(AccountID::new(1), None, b"key1", 20)
            .unwrap();
        assert_eq!(
            state_handler
                .accumulator_get(AccountID::new(1), None, b"key1")
                .unwrap(),
            30
        );
        state_handler
            .accumulator_add(AccountID::new(1), None, b"key1", 30)
            .unwrap();
        assert_eq!(
            state_handler
                .accumulator_get(AccountID::new(1), None, b"key1")
                .unwrap(),
            60
        );

        // accumulator safe sub
        assert!(state_handler
            .accumulator_safe_sub(AccountID::new(1), None, b"key1", 10)
            .unwrap());
        assert_eq!(
            state_handler
                .accumulator_get(AccountID::new(1), None, b"key1")
                .unwrap(),
            50
        );
        assert!(state_handler
            .accumulator_safe_sub(AccountID::new(1), None, b"key1", 10)
            .unwrap());
        assert_eq!(
            state_handler
                .accumulator_get(AccountID::new(1), None, b"key1")
                .unwrap(),
            40
        );
        assert_eq!(
            state_handler.accumulator_safe_sub(AccountID::new(1), None, b"key1", 50),
            Err(StdStateError::ExecutionErrorCode(ErrorCode::HandlerCode(0)))
        );
    }
}
