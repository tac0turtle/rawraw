//! The raw handler and host backend interfaces.
use crate::code::{ErrorCode, SystemCode};
use crate::message::{QueryStateResponse, StateRequest, UpdateStateResponse};
use crate::packet::MessagePacket;
pub use allocator_api2::alloc::Allocator;

/// A handler for an account.
pub trait RawHandler {
    /// Handle a message.
    #[allow(unused_variables)]
    fn handle_msg(
        &self,
        _message_packet: &mut MessagePacket,
        _callbacks: &mut dyn HostBackend,
        _allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        Err(ErrorCode::SystemCode(SystemCode::MessageNotHandled))
    }

    /// Handle a query message.
    fn handle_query(
        &self,
        _message_packet: &mut MessagePacket,
        _callbacks: &dyn HostBackend,
        _allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        Err(ErrorCode::SystemCode(SystemCode::MessageNotHandled))
    }

    /// Handle a system message.
    fn handle_system(
        &self,
        _message_packet: &mut MessagePacket,
        _callbacks: &mut dyn HostBackend,
        _allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        Err(ErrorCode::SystemCode(SystemCode::MessageNotHandled))
    }
}

/// A host backend for the handler.
pub trait HostBackend {
    /// Invoke a message packet.
    fn invoke_msg(
        &mut self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode>;

    /// Invoke a query message packet.
    fn invoke_query(
        &self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode>;

    /// Update the state of the account.
    fn update_state<'a>(&mut self, req: &StateRequest, allocator: &'a dyn Allocator) -> Result<UpdateStateResponse<'a>, ErrorCode>;

    /// Query the state of the account.
    fn query_state<'a>(&self, req: &StateRequest, allocator: &'a dyn Allocator) -> Result<QueryStateResponse<'a>, ErrorCode>;

    /// Consume gas. Returns an out-of-gas error if there is not enough gas.
    fn consume_gas(&self, gas: u64) -> Result<(), ErrorCode>;
}
