#![allow(unused)]
use crate::EventData;
use allocator_api2::alloc::Allocator;
use imbl::{HashMap, OrdMap, Vector};
use ixc_account_manager::state_handler::std::StdStateManager;
use ixc_account_manager::state_handler::StateHandler;
use ixc_core_macros::message_selector;
use ixc_message_api::code::{ErrorCode, SystemCode};
use ixc_message_api::{alloc_util, AccountID};
use std::alloc::Layout;
use std::cell::RefCell;
use thiserror::Error;

#[derive(Default, Clone)]
pub struct VersionedMultiStore {
    versions: Vector<MultiStore>,
}

impl VersionedMultiStore {
    pub fn new_transaction(&self) -> Tx {
        let latest = self.versions.last().cloned().unwrap_or_default();
        Tx {
            call_stack: vec![Frame {
                store: latest,
                events: Default::default(),
            }],
        }
    }

    pub fn commit(&mut self, tx: Tx) -> Result<Vector<EventData>, ()> {
        if tx.call_stack.len() != 1 {
            return Err(());
        }
        let current_frame = tx.current_frame().map_err(|_| ())?;
        self.versions.push_back(current_frame.store.clone());
        Ok(current_frame.events.clone())
    }
}

#[derive(Default, Clone, Debug)]
pub struct MultiStore {
    stores: HashMap<AccountID, Store>,
    events: Vec<EventData>,
}

#[derive(Default, Clone, Debug)]
pub struct Store {
    kv_store: OrdMap<Vec<u8>, Vec<u8>>,
}

pub struct Tx {
    call_stack: Vec<Frame>,
}

impl Tx {
    fn current_frame(&self) -> Result<&Frame, ErrorCode> {
        self.call_stack
            .last()
            .ok_or(ErrorCode::SystemCode(SystemCode::FatalExecutionError))
    }

    fn current_frame_mut(&mut self) -> Result<&mut Frame, ErrorCode> {
        self.call_stack
            .last_mut()
            .ok_or(ErrorCode::SystemCode(SystemCode::FatalExecutionError))
    }
}

impl StdStateManager for Tx {
    fn kv_get<'a>(
        &self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        allocator: &'a dyn Allocator,
    ) -> Result<Option<&'a [u8]>, ErrorCode> {
        if scope.is_some() {
            todo!("scoped kv_get")
        }
        if let Some(store) = self.current_frame()?.store.stores.get(&account_id) {
            if let Some(value) = store.kv_store.get(key) {
                unsafe { Ok(Some(alloc_util::copy_bytes(allocator, value.as_slice())?)) }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn kv_set(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        value: &[u8],
    ) -> Result<(), ErrorCode> {
        if scope.is_some() {
            todo!("scoped kv_set")
        }
        let multistore = &mut self.current_frame_mut()?.store;
        if let Some(store) = multistore.stores.get_mut(&account_id) {
            store.kv_store.insert(key.to_vec(), value.to_vec());
        } else {
            let mut store = Store::default();
            store.kv_store.insert(key.to_vec(), value.to_vec());
            multistore.stores.insert(account_id, store);
        }
        Ok(())
    }

    fn kv_delete(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
    ) -> Result<(), ErrorCode> {
        if scope.is_some() {
            todo!("scoped kv_delete")
        }
        let multistore = &mut self.current_frame_mut()?.store;
        multistore.stores.remove(&account_id);
        Ok(())
    }

    fn accumulator_get(
        &self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
    ) -> Result<u128, ErrorCode> {
        todo!("accumulator_get")
    }

    fn accumulator_add(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        value: u128,
    ) -> Result<(), ErrorCode> {
        todo!("accumulator_add")
    }

    fn accumulator_safe_sub(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        value: u128,
    ) -> Result<bool, ErrorCode> {
        todo!("accumulator_safe_sub")
    }

    fn begin_tx(&mut self) -> Result<(), ErrorCode> {
        self.call_stack.push(Frame {
            store: self.current_frame()?.store.clone(),
            events: Default::default(),
        });
        Ok(())
    }

    fn commit_tx(&mut self) -> Result<(), ErrorCode> {
        // when we commit, we pop the current frame and set the store in the next frame to the current frame's store
        let current_frame = self.current_frame()?;
        let new_multi_store = current_frame.store.clone();
        let events = current_frame.events.clone();
        self.call_stack.pop();
        let next_frame = self.current_frame_mut()?;
        next_frame.store = new_multi_store;
        next_frame.events = events;
        Ok(())
    }

    fn rollback_tx(&mut self) -> Result<(), ErrorCode> {
        // when we rollback we simply pop the current frame
        self.call_stack.pop();
        Ok(())
    }

    fn create_account_storage(&mut self, account: AccountID) -> Result<(), ErrorCode> {
        let mut current_frame = self.current_frame_mut()?;
        current_frame.store.stores.insert(account, Store::default());
        Ok(())
    }

    fn delete_account_storage(&mut self, account: AccountID) -> Result<(), ErrorCode> {
        let mut current_frame = self.current_frame_mut()?;
        current_frame.store.stores.remove(&account);
        Ok(())
    }

    fn emit_event(
        &mut self,
        sender: AccountID,
        type_selector: u64,
        data: &[u8],
    ) -> Result<(), ErrorCode> {
        let mut current_frame = self.current_frame_mut()?;
        current_frame.events.push_back(EventData {
            sender,
            type_selector,
            data: data.to_vec(),
        });
        Ok(())
    }
}

#[derive(Debug, Error)]
enum Error {
    #[error("allocation error")]
    AllocError(#[from] allocator_api2::alloc::AllocError),
    #[error("access error")]
    AccessError(#[from] AccessError),
}

#[derive(Debug, Error)]
#[error("access error")]
struct AccessError;

#[derive(Clone)]
pub struct Frame {
    store: MultiStore,
    events: Vector<EventData>,
}
