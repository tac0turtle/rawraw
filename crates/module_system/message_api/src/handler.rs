//! The raw handler and host backend interfaces.
use crate::code::{ErrorCode, SystemCode};
use crate::gas::Gas;
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
    ) -> Result<Response<'a>, ErrorCode> {
        Err(ErrorCode::System(SystemCode::MessageNotHandled))
    }

    /// Handle a query message.
    fn handle_query<'a>(
        &self,
        _message: &Message,
        _callbacks: &dyn HostBackend,
        _allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        Err(ErrorCode::System(SystemCode::MessageNotHandled))
    }

    /// Handle a system message.
    fn handle_system<'a>(
        &self,
        _forwarded_caller: &AccountID,
        _message_packet: &Message,
        _callbacks: &mut dyn HostBackend,
        _allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        Err(ErrorCode::System(SystemCode::MessageNotHandled))
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
}

/// Parameters common to all invoke methods on HostBackend.
#[non_exhaustive]
pub struct InvokeParams<'a> {
    /// The allocator used to allocate responses.
    pub allocator: &'a dyn Allocator,

    /// An optional gas limit to set for the message.
    ///
    /// The amount of gas consumed will be updated based on how
    /// much gas was used during message execution.
    /// The message will fail if the gas limit is exceeded.
    /// If the gas limit set is greater than the parent gas limit,
    /// then the parent gas limit will be used.
    /// If an unlimited gas meter is provided, then gas consumption can be monitored
    /// without setting a limit.
    pub gas: &'a Option<Gas>,
}

impl<'a> InvokeParams<'a> {
    /// Create a new InvokeParams.
    pub fn new(allocator: &'a dyn Allocator, gas: &'a Option<Gas>) -> Self {
        Self { allocator, gas }
    }
}
