//! Rust Cosmos SDK RFC 003 hypervisor/state-handler function implementation.
#![no_std]
extern crate alloc;

mod call_stack;
mod exec_ctx;
pub mod gas;
mod gas_stack;
pub mod id_generator;
pub mod native_vm;
mod query_ctx;
mod scope_guard;
pub mod state_handler;
mod wrapper;

use crate::call_stack::CallStack;
use crate::exec_ctx::ExecContext;
use crate::gas::GasMeter;
use crate::gas_stack::GasStack;
use crate::id_generator::IDGenerator;
use crate::query_ctx::QueryContext;
use crate::state_handler::StateHandler;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::handler::{Allocator, HostBackend, InvokeParams};
use ixc_message_api::message::{Message, Response};
use ixc_message_api::AccountID;
use ixc_vm_api::{ReadonlyStore, VM};

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
    /// Returns a new host backend for the provided state handler.
    pub fn invoke_msg<'b, ST: StateHandler, IDG: IDGenerator>(
        &self,
        state_handler: &mut ST,
        id_generator: &IDG,
        caller: AccountID,
        message: &Message,
        invoke_params: &InvokeParams<'b, '_>,
    ) -> Result<Response<'b>, ErrorCode> {
        let mut exec_context = ExecContext::new(
            self,
            state_handler,
            id_generator,
            caller,
            invoke_params.gas_tracker,
        );
        exec_context.do_invoke_msg(message, invoke_params)
    }

    /// Invokes the query in the context of the provided state handler.
    pub fn invoke_query<'b, ST: StateHandler>(
        &self,
        state_handler: &ST,
        message_packet: &Message,
        invoke_params: &InvokeParams<'b, '_>,
    ) -> Result<Response<'b>, ErrorCode> {
        let call_stack = CallStack::new(AccountID::EMPTY);
        let gas_stack = GasStack::new(invoke_params.gas_tracker.map(|g| g.limit).flatten());
        let query_ctx = QueryContext::new(self, state_handler, &call_stack, &gas_stack);
        query_ctx.invoke_query(message_packet, invoke_params)
    }
}

struct ReadOnlyStoreWrapper<'a, S: StateHandler> {
    state_handler: &'a S,
    allocator: &'a dyn Allocator,
    gas: &'a GasMeter,
}

impl<'a, S: StateHandler> ReadOnlyStoreWrapper<'a, S> {
    fn wrap(state_handler: &'a S, gas: &'a GasMeter, allocator: &'a dyn Allocator) -> Self {
        Self {
            state_handler,
            gas,
            allocator,
        }
    }
}

impl<S: StateHandler> ReadonlyStore for ReadOnlyStoreWrapper<'_, S> {
    fn get(&self, account_id: AccountID, key: &[u8]) -> Result<Option<&[u8]>, ErrorCode> {
        self.state_handler
            .kv_get(account_id, key, self.gas, self.allocator)
    }
}
