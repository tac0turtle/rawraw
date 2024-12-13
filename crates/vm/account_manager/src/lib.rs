//! Rust Cosmos SDK RFC 003 hypervisor/state-handler function implementation.
#![no_std]
extern crate alloc;

mod exec_ctx;
pub mod id_generator;
pub mod native_vm;
pub mod state_handler;
mod call_stack;
mod query_ctx;
mod gas;

use crate::call_stack::CallStack;
use crate::exec_ctx::ExecContext;
use crate::id_generator::IDGenerator;
use crate::query_ctx::QueryContext;
use crate::state_handler::StateHandler;
use allocator_api2::vec::Vec;
use core::cell::RefCell;
use ixc_core_macros::message_selector;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::code::ErrorCode::SystemCode;
use ixc_message_api::code::SystemCode::FatalExecutionError;
use ixc_message_api::handler::{Allocator, HostBackend};
use ixc_message_api::packet::MessagePacket;
use ixc_message_api::AccountID;
use ixc_vm_api::{ReadonlyStore, VM};
use crate::gas::GasMeter;

/// The default stack size for the account manager.
pub const DEFAULT_STACK_SIZE: usize = 128;

/// The account manager manages the execution, creation, and destruction of accounts.
pub struct AccountManager<'a, CM: VM, const CALL_STACK_LIMIT: usize = DEFAULT_STACK_SIZE> {
    code_manager: &'a CM,
}

impl<'a, CM: VM, const CALL_STACK_LIMIT: usize> AccountManager<'a, CM, CALL_STACK_LIMIT> {
    /// Creates a new account manager.
    pub fn new(code_manager: &'a CM) -> Self {
        Self { code_manager }
    }
}

impl<CM: VM, const CALL_STACK_LIMIT: usize> AccountManager<'_, CM, CALL_STACK_LIMIT> {
    /// Invokes a message packet in the context of the provided state handler.
    pub fn invoke_msg<'b, ST: StateHandler, IDG: IDGenerator>(
        &'b self,
        state_handler: &'b mut ST,
        id_generator: &'b mut IDG,
        message_packet: &mut MessagePacket,
        allocator: &'b dyn Allocator,
    ) -> Result<(), ErrorCode> {
        let mut exec_context = ExecContext::new(self, state_handler, id_generator, message_packet.header().account);
        exec_context.invoke_msg(message_packet, allocator)
    }

    /// Invokes a message packet in the context of the provided state handler.
    pub fn invoke_query<ST: StateHandler>(
        &self,
        state_handler: &ST,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        let mut call_stack = CallStack::new(message_packet.header().caller);
        let query_ctx = QueryContext::new(self, state_handler, &call_stack);
        query_ctx.invoke_query(message_packet, allocator)
    }
}

struct ReadOnlyStoreWrapper<'a, S: StateHandler> {
    state_handler: &'a S,
    gas: RefCell<&'a mut GasMeter>,
}

impl<'a, S: StateHandler> ReadOnlyStoreWrapper<'a, S> {
    fn wrap(state_handler: &'a S, gas: &'a mut GasMeter) -> Self {
        Self {
            state_handler,
            gas: RefCell::new(gas),
        }
    }
}

impl<S: StateHandler> ReadonlyStore for ReadOnlyStoreWrapper<'_, S> {
    fn get<'b>(
        &self,
        account_id: AccountID,
        key: &[u8],
        allocator: &'b dyn Allocator,
    ) -> Result<Option<Vec<u8, &'b dyn Allocator>>, ErrorCode> {
        let mut gas = self
            .gas
            .try_borrow_mut()
            .map_err(|_| SystemCode(FatalExecutionError))?;
        self.state_handler.kv_get(account_id, key, *gas, allocator)
    }
}
