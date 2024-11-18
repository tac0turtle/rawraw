use std::cell::RefCell;
use allocator_api2::alloc::Allocator;
use ixc_core_macros::message_selector;
use ixc_hypervisor::{PopFrameError, PushFrameError, Transaction};
use ixc_message_api::AccountID;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::header::MessageSelector;
use ixc_message_api::packet::MessagePacket;

const HAS_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.has");
const GET_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.get");
const SET_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.set");
const DELETE_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.delete");

pub trait DiskKV {
    fn get(&self, account_id: &AccountID, key: &[u8]) -> Option<Vec<u8>>;
    fn apply_state_changes(&mut self, state_changes: Vec<StateChange>) -> Result<(), ()>;
}

pub struct KV<S>{
    store: S
}

pub enum StateChange {
    InitAccount {
        account_id: AccountID,
    },
    Set {
        account_id: AccountID,
        key: Vec<u8>,
        value: Vec<u8>,
    },
    Delete {
        account_id: AccountID,
        key: Vec<u8>,
    }
}

pub struct Tx<S> {
    changes: Vec<StateChange>,
    db: S
}

impl<S: DiskKV> Transaction for Tx<S> {
    fn init_account_storage(&mut self, account: AccountID) -> Result<(), PushFrameError> {
        self.changes.push(StateChange::InitAccount {
            account_id: account,
        });
        Ok(())
    }

    fn push_frame(&mut self, account: AccountID, volatile: bool) -> Result<(), PushFrameError> {
        todo!()
    }

    fn pop_frame(&mut self, commit: bool) -> Result<(), PopFrameError> {
        todo!()
    }

    fn active_account(&self) -> AccountID {
        todo!()
    }

    fn self_destruct_account(&mut self) -> Result<(), ()> {
        todo!()
    }

    fn raw_kv_get(&self, account_id: AccountID, key: &[u8]) -> Option<Vec<u8>> {
        todo!()
    }

    fn raw_kv_set(&self, account_id: AccountID, key: &[u8], value: &[u8]) {
        todo!()
    }

    fn raw_kv_delete(&self, account_id: AccountID, key: &[u8]) {
        todo!()
    }

    fn handle(&self, message_packet: &mut MessagePacket, allocator: &dyn Allocator) -> Result<(), ErrorCode> {
        todo!()
    }
}