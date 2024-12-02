//! **WARNING: This is an API preview! Expect major bugs, glaring omissions, and breaking changes!**
//!
//! Virtual Machine API
use ixc_message_api::code::ErrorCode;
use ixc_message_api::handler::{Allocator, HostBackend};
use ixc_message_api::packet::MessagePacket;


/// A virtual machine that can run message handlers.
pub trait VM {
    /// Resolves a handler ID provided by a caller to the handler ID which should be stored in state
    /// or return None if the handler ID is not valid.
    /// This allows for multiple ways of addressing a single handler in code and for ensuring that
    /// the handler actually exists.
    fn resolve_handler_id<S: ReadonlyStore>(&self, store: &S, handler_id: &[u8]) -> Option<Vec<u8>>;
    /// Runs a handler with the provided message packet and host backend.
    fn run_message<S: ReadonlyStore>(
        &self,
        store: &S,
        handler_id: &[u8],
        message_packet: &mut MessagePacket,
        backend: &mut dyn HostBackend,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode>;

    fn run_query<S: ReadonlyStore>(
        &self,
        store: &S,
        handler_id: &[u8],
        message_packet: &mut MessagePacket,
        backend: &dyn HostBackend,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode>;

    fn run_system_message<S: ReadonlyStore>(
        &self,
        store: &S,
        handler_id: &[u8],
        message_packet: &mut MessagePacket,
        backend: &mut dyn HostBackend,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode>;
}

pub trait ReadonlyStore {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, ErrorCode>;
}


/// A descriptor for a handler.
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct HandlerDescriptor {}
