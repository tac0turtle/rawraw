//! The raw handler and host backend interfaces.
use crate::code::{ErrorCode, SystemCode};
use crate::error::HandlerError;
use crate::gas::GasTracker;
use crate::message::{Message, Request, Response};
use crate::AccountID;
pub use allocator_api2::alloc::Allocator;

/// A handler for an account.
pub trait RawHandler {
    /// Handle a message.
    fn handle_msg<'a>(
        &self,
        _caller: &AccountID,
        _message: &Message,
        _callbacks: &mut dyn HostBackend,
        _allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, HandlerError> {
        Err(SystemCode::MessageNotHandled.into())
    }

    /// Handle a query message.
    fn handle_query<'a>(
        &self,
        _message: &Message,
        _callbacks: &dyn HostBackend,
        _allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, HandlerError> {
        Err(SystemCode::MessageNotHandled.into())
    }

    /// Handle a system message.
    fn handle_system<'a>(
        &self,
        _forwarded_caller: &AccountID,
        _message_packet: &Message,
        _callbacks: &mut dyn HostBackend,
        _allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, HandlerError> {
        Err(SystemCode::MessageNotHandled.into())
    }
}

/// A host backend for the handler.
pub trait HostBackend {
    /// Invoke a message packet.
    fn invoke_msg<'a>(
        &mut self,
        message: &Message,
        invoke_params: &InvokeParams<'a, '_>,
    ) -> Result<Response<'a>, ErrorCode>;

    /// Invoke a query message packet.
    fn invoke_query<'a>(
        &self,
        message: &Message,
        invoke_params: &InvokeParams<'a, '_>,
    ) -> Result<Response<'a>, ErrorCode>;

    /// Update the state of the account.
    fn update_state<'a>(
        &mut self,
        req: &Request,
        invoke_params: &InvokeParams<'a, '_>,
    ) -> Result<Response<'a>, ErrorCode>;

    /// Query the state of the account.
    fn query_state<'a>(
        &self,
        req: &Request,
        invoke_params: &InvokeParams<'a, '_>,
    ) -> Result<Response<'a>, ErrorCode>;

    /// Consume gas. Returns an out-of-gas error if there is not enough gas.
    fn consume_gas(&self, gas: u64) -> Result<(), ErrorCode>;

    /// Returns true if there is not enough gas to continue execution.
    fn out_of_gas(&self) -> Result<bool, ErrorCode>;
}

/// Parameters common to all invoke methods on HostBackend.
#[non_exhaustive]
pub struct InvokeParams<'a, 'b> {
    /// The allocator used to allocate responses.
    pub allocator: &'a dyn Allocator,

    /// An optional gas limit and tracker to set for the message.
    pub gas_tracker: Option<&'b GasTracker>,
}

impl<'a, 'b> InvokeParams<'a, 'b> {
    /// Create a new InvokeParams.
    pub fn new(allocator: &'a dyn Allocator, gas_tracker: Option<&'b GasTracker>) -> Self {
        Self {
            allocator,
            gas_tracker,
        }
    }
}
