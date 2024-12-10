//! **WARNING: This is an API preview! Expect major bugs, glaring omissions, and breaking changes!**
//!
//! Virtual Machine API
#![no_std]

use ixc_message_api::AccountID;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::handler::{Allocator, HostBackend, RawHandler};
use ixc_message_api::packet::MessagePacket;
use allocator_api2::vec::Vec;

/// A virtual machine that can run message handlers.
pub trait VM {
    /// Resolves a handler ID provided by a caller to the handler ID which should be stored in state
    /// or return None if the handler ID is not valid.
    /// This allows for multiple ways of addressing a single handler in code and for ensuring that
    /// the handler actually exists.
    fn resolve_handler_id<'a>(&self, store: &dyn ReadonlyStore, handler_id: &[u8], allocator: &'a dyn Allocator) -> Result<Option<Vec<u8, &'a dyn Allocator>>, ErrorCode>;

    /// Resolves a handler ID to an executable handler or returns an error if the handler is not found.
    fn resolve_handler<'a>(&self, store: &dyn ReadonlyStore, handler_id: &[u8], allocator: &'a dyn Allocator) -> Result<&'a dyn RawHandler, ErrorCode>;

    // /// Runs a handler with the provided message packet and host backend.
    // fn run_message(
    //     &self,
    //     store: &dyn ReadonlyStore,
    //     handler_id: &[u8],
    //     message_packet: &mut MessagePacket,
    //     backend: &mut dyn HostBackend,
    //     allocator: &dyn Allocator,
    // ) -> Result<(), ErrorCode>;
    //
    // /// Runs a query handler with the provided message packet and host backend.
    // fn run_query(
    //     &self,
    //     store: &dyn ReadonlyStore,
    //     handler_id: &[u8],
    //     message_packet: &mut MessagePacket,
    //     backend: &dyn HostBackend,
    //     allocator: &dyn Allocator,
    // ) -> Result<(), ErrorCode>;
    //
    // /// Runs a system message handler with the provided message packet and host backend.
    // fn run_system_message(
    //     &self,
    //     store: &dyn ReadonlyStore,
    //     handler_id: &[u8],
    //     message_packet: &mut MessagePacket,
    //     backend: &mut dyn HostBackend,
    //     allocator: &dyn Allocator,
    // ) -> Result<(), ErrorCode>;
}

/// A store that can only be read from.
/// In the context of a VM,
/// this state should only be used to retrieve the code for a handler from the store.
pub trait ReadonlyStore {
    /// Gets the value for the given key for the given account.
    fn get<'a>(&self, account_id: AccountID, key: &[u8], allocator: &'a dyn Allocator) -> Result<Option<Vec<u8, &'a dyn Allocator>>, ErrorCode>;
}

/// A descriptor for a handler.
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct HandlerDescriptor {}
