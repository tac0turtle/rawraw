use allocator_api2::alloc::Allocator;
use ixc_message_api::AccountID;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::packet::MessagePacket;
use crate::state_handler::{Gas, StateHandler};
use crate::state_handler::std::manager::StdStateManager;

/// The standard state handler.
pub struct StdStateHandler<'a, A: Allocator, S: StdStateManager<A>> {
    state: &'a mut S,
    gas_config: GasConfig,
    _phantom: core::marker::PhantomData<A>,
}

/// Gas configuration for the standard state handler.
#[derive(Default)]
pub struct GasConfig {
    /// The cost of deleting a value from storage.
    pub delete_cost: u64,
    /// The flat cost of reading a value from storage.
    pub read_cost_flat: u64,
    /// The cost per byte of reading a value from storage.
    pub read_cost_per_byte: u64,
    /// The flat cost of writing a value to storage.
    pub write_cost_flat: u64,
    /// The cost per byte of writing a value to storage.
    pub write_cost_per_byte: u64,
}

impl<'a, A: Allocator, S: StdStateManager<A>> StdStateHandler<'a, A, S> {
    /// Create a new standard state handler.
    pub fn new(state: &'a mut S, gas_config: GasConfig) -> Self {
        Self { state, gas_config, _phantom: Default::default() }
    }
}

impl<'a, A: Allocator, S: StdStateManager<A>> StateHandler<A> for StdStateHandler<'a, A, S> {
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