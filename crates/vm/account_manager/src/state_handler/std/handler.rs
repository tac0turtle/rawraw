use allocator_api2::alloc::Allocator;
use ixc_message_api::AccountID;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::packet::MessagePacket;
use crate::state_handler::{Gas, StateHandler};
use crate::state_handler::std::manager::StdStateManager;

pub struct StdStateHandler<A: Allocator, S: StdStateManager<A>> {
    state: S,
    gas_config: GasConfig,
}

pub struct GasConfig {
    pub has_cost: u64,
    pub delete_cost: u64,
    pub read_cost_flat: u64,
    pub read_cost_per_byte: u64,
    pub write_cost_flat: u64,
    pub write_cost_per_byte: u64,
}

impl<A: Allocator, S: StdStateManager<A>> StdStateHandler<A, S> {
    pub fn new(state: S, gas_config: GasConfig) -> Self {
        Self { state, gas_config }
    }
}

impl<A: Allocator, S: StdStateManager<A>> StateHandler<A> for StdStateHandler<A, S> {
    fn kv_get(&self, account_id: AccountID, key: &[u8], gas: &mut Gas, allocator: A) -> Result<Option<allocator_api2::vec::Vec<u8, A>>, ErrorCode> {
        todo!()
    }

    fn kv_set(&mut self, account_id: AccountID, key: &[u8], value: &[u8], gas: &mut Gas) -> Result<(), ErrorCode> {
        todo!()
    }

    fn kv_delete(&mut self, account_id: AccountID, key: &[u8], gas: &mut Gas) -> Result<(), ErrorCode> {
        todo!()
    }

    fn begin_tx(&mut self) -> Result<(), ErrorCode> {
        todo!()
    }

    fn commit_tx(&mut self) -> Result<(), ErrorCode> {
        todo!()
    }

    fn rollback_tx(&mut self) -> Result<(), ErrorCode> {
        todo!()
    }

    fn handle_exec(&mut self, message_packet: &mut MessagePacket, allocator: &dyn Allocator) -> Result<(), ErrorCode> {
        todo!()
    }

    fn handle_query(&self, message_packet: &mut MessagePacket, allocator: &dyn Allocator) -> Result<(), ErrorCode> {
        todo!()
    }

    fn create_account_storage(&mut self, account: AccountID, gas: &mut Gas) -> Result<(), ErrorCode> {
        todo!()
    }

    fn delete_account_storage(&mut self, account: AccountID, gas: &mut Gas) -> Result<(), ErrorCode> {
        todo!()
    }
}