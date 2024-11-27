use allocator_api2::alloc::Allocator;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::handler::HostBackend;
use ixc_message_api::packet::MessagePacket;
use ixc_vm_api::HandlerDescriptor;

/// A code manager is responsible for resolving handler IDs to code.
pub trait CodeManager {
    /// Resolves a handler ID provided by a caller to the handler ID which should be stored in state
    /// or return None if the handler ID is not valid.
    /// This allows for multiple ways of addressing a single handler in code and for ensuring that
    /// the handler actually exists.
    fn resolve_handler_id(&self, handler_id: &[u8]) -> Option<Vec<u8>>;
    /// Runs a handler with the provided message packet and host backend.
    fn run_message(
        &self,
        handler_id: &[u8],
        message_packet: &mut MessagePacket,
        backend: &mut dyn HostBackend,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode>;

    fn run_query(
        &self,
        handler_id: &[u8],
        message_packet: &mut MessagePacket,
        backend: &dyn HostBackend,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode>;

    fn run_system_message(
        &self,
        handler_id: &[u8],
        message_packet: &mut MessagePacket,
        backend: &mut dyn HostBackend,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode>;
}
