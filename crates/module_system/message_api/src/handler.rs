//! The raw handler and host backend interfaces.
use crate::code::{ErrorCode, SystemCode};
use crate::packet::MessagePacket;

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

pub use allocator_api2::alloc::Allocator;

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
}
