//! A state handler that can be used to store and retrieve state.
mod snapshot_state;

use crate::snapshot_state::{Snapshot, SnapshotState};
use allocator_api2::alloc::{Allocator, Global};
use allocator_api2::vec::Vec;
use ixc_account_manager::state_handler::std::{StdStateError, StdStateManager};
use ixc_message_api::AccountID;

/// A store that can be used to store and retrieve state.
pub trait Store {
    /// Get the value for the given key.
    fn get(&self, key: &Vec<u8>) -> Option<Vec<u8>>;
}

/// StateHandler is a cache-based state handler that can be used to store and retrieve state.
pub struct StateHandler<'a, S: Store> {
    snapshot_state: SnapshotState<'a, S>,
    /// Checkpoints are used to revert state changes.
    /// checkpoints are used to follow the call stack of a transaction
    /// and revert the state changes when the transaction is rolled back.
    checkpoints: Vec<Snapshot>,
}

impl<'a, S: Store> StateHandler<'a, S> {
    /// Create a new state handler with the given store.
    pub fn new(store: &'a S) -> Self {
        Self {
            snapshot_state: SnapshotState::new(store),
            checkpoints: Vec::with_capacity_in(200, Global),
        }
    }
}

impl<'a, S: Store> StdStateManager for StateHandler<'a, S> {
    fn kv_get<A: Allocator>(
        &self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        allocator: A,
    ) -> Result<Option<Vec<u8, A>>, StdStateError> {
        todo!()
    }

    fn kv_set(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        value: &[u8],
    ) -> Result<(), StdStateError> {
        todo!()
    }

    fn kv_delete(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
    ) -> Result<(), StdStateError> {
        todo!()
    }

    fn accumulator_get(
        &self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
    ) -> Result<u128, StdStateError> {
        todo!()
    }

    fn accumulator_add(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        value: u128,
    ) -> Result<(), StdStateError> {
        todo!()
    }
    fn accumulator_safe_sub(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        value: u128,
    ) -> Result<bool, StdStateError> {
        todo!()
    }

    /// Begins a new transaction.
    fn begin_tx(&mut self) -> Result<(), StdStateError> {
        todo!()
    }

    /// Commits the current transaction.
    fn commit_tx(&mut self) -> Result<(), StdStateError> {
        todo!()
    }

    /// Rolls back the current transaction.
    fn rollback_tx(&mut self) -> Result<(), StdStateError> {
        todo!()
    }

    /// Create storage for a new account.
    fn create_account_storage(&mut self, account: AccountID) -> Result<(), StdStateError> {
        todo!()
    }

    /// Delete all of an account's storage.
    fn delete_account_storage(&mut self, account: AccountID) -> Result<(), StdStateError> {
        todo!()
    }

    /// Emit an event.
    fn emit_event(&mut self, sender: AccountID, data: &[u8]) -> Result<(), StdStateError> {
        todo!()
    }
}
