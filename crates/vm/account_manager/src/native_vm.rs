//! Defines a VM manager that can be used to register VMs and resolve handler IDs.
extern crate alloc;

use alloc::boxed::Box;
use allocator_api2::alloc::Allocator;
use allocator_api2::vec::Vec;
use ixc_message_api::code::{ErrorCode, SystemCode};
use ixc_message_api::handler::RawHandler;
use ixc_vm_api::{ReadonlyStore, VM};
use alloc::collections::btree_map::BTreeMap;
use core::borrow::Borrow;

/// The native Rust virtual machine implementation.
#[derive(Default)]
pub struct NativeVMImpl {
    handlers: BTreeMap<alloc::vec::Vec<u8>, Box<dyn RawHandler>>,
}

/// The trait that VMs which support native execution must implement to be used with the test harness.
pub trait NativeVM: VM {
    /// Registers a handler with the native VM.
    fn register_handler(&mut self, name: &str, handler: Box<dyn RawHandler>);
}

impl NativeVM for NativeVMImpl {
    fn register_handler(&mut self, name: &str, handler: Box<dyn RawHandler>) {
        self.handlers.insert(name.into(), handler);
    }
}

impl VM for NativeVMImpl {
    fn resolve_handler_id<'a>(&self, store: &dyn ReadonlyStore, handler_id: &[u8], allocator: &'a dyn Allocator) -> Result<Option<allocator_api2::vec::Vec<u8, &'a dyn Allocator>>, ErrorCode> {
        if self.handlers.contains_key(handler_id) {
            let mut res = Vec::new_in(allocator);
            res.extend_from_slice(handler_id);
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }

    fn resolve_handler<'b, 'a:'b>(&'a self, store: &dyn ReadonlyStore, handler_id: &[u8], allocator: &'b dyn Allocator) -> Result<&'b dyn RawHandler, ErrorCode> {
        if let Some(handler) = self.handlers.get(handler_id) {
            Ok(handler.borrow())
        } else {
            Err(ErrorCode::SystemCode(SystemCode::HandlerNotFound))
        }
    }
}
