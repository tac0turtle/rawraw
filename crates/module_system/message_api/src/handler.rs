//! The raw handler and host backend interfaces.
use crate::code::{ErrorCode, SystemCode};
use crate::message::{Message, Request, Response};
pub use allocator_api2::alloc::Allocator;
use crate::AccountID;

/// A handler for an account.
pub trait RawHandler {
    /// Handle a message.
    fn handle_msg<'a>(
        &self,
        _message: &Message,
        _callbacks: &mut dyn HostBackend,
        _allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        Err(ErrorCode::SystemCode(SystemCode::MessageNotHandled))
    }

    /// Handle a query message.
    fn handle_query<'a>(
        &self,
        _message: &Message,
        _callbacks: &dyn HostBackend,
        _allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        Err(ErrorCode::SystemCode(SystemCode::MessageNotHandled))
    }

    /// Handle a system message.
    fn handle_system<'a>(
        &self,
        _message_packet: &Message,
        _callbacks: &mut dyn HostBackend,
        _allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        Err(ErrorCode::SystemCode(SystemCode::MessageNotHandled))
    }
}

/// A host backend for the handler.
pub trait HostBackend {
    /// Invoke a message packet.
    fn invoke_msg<'a>(
        &mut self,
        message: &Message,
        invoke_params: &InvokeParams<'a>,
    ) -> Result<Response<'a>, ErrorCode>;

    /// Invoke a query message packet.
    fn invoke_query<'a>(
        &self,
        message: &Message,
        invoke_params: &InvokeParams<'a>,
    ) -> Result<Response<'a>, ErrorCode>;

    /// Update the state of the account.
    fn update_state<'a>(
        &mut self,
        req: &Request,
        invoke_params: &InvokeParams<'a>,
    ) -> Result<Response<'a>, ErrorCode>;

    /// Query the state of the account.
    fn query_state<'a>(
        &self,
        req: &Request,
        invoke_params: &InvokeParams<'a>,
    ) -> Result<Response<'a>, ErrorCode>;

    /// Consume gas. Returns an out-of-gas error if there is not enough gas.
    fn consume_gas(&self, gas: u64) -> Result<(), ErrorCode>;

    /// Get the account ID of the caller of the message.
    fn caller(&self) -> AccountID;

}

/// Parameters common to all invoke methods on HostBackend.
#[non_exhaustive]
pub struct InvokeParams<'a> {
    /// The allocator used to allocate responses.
    pub allocator: &'a dyn Allocator,
}

impl<'a> InvokeParams<'a> {
    /// Create a new InvokeParams.
    pub fn new(allocator: &'a dyn Allocator) -> Self {
        Self {
            allocator,
        }
    }
}