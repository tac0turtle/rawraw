//! Defines a VM manager that can be used to register VMs and resolve handler IDs.
extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::btree_map::BTreeMap;
use alloc::string::String;
use allocator_api2::alloc::Allocator;
use core::borrow::Borrow;
use ixc_message_api::alloc_util;
use ixc_message_api::code::{ErrorCode, SystemCode};
use ixc_message_api::handler::RawHandler;
use ixc_vm_api::{ReadonlyStore, VM};

/// The native Rust virtual machine implementation.
#[derive(Default)]
pub struct NativeVMImpl {
    handlers: BTreeMap<String, Box<dyn RawHandler>>,
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
    fn resolve_handler_id<'a>(
        &self,
        _store: &dyn ReadonlyStore,
        handler_id: &str,
        allocator: &'a dyn Allocator,
    ) -> Result<Option<&'a str>, ErrorCode> {
        if self.handlers.contains_key(handler_id) {
            unsafe { Ok(Some(alloc_util::copy_str(allocator, handler_id)?)) }
        } else {
            Ok(None)
        }
    }

    fn resolve_handler<'b, 'a: 'b>(
        &'a self,
        _store: &dyn ReadonlyStore,
        handler_id: &str,
        _allocator: &'b dyn Allocator,
    ) -> Result<&'b dyn RawHandler, ErrorCode> {
        if let Some(handler) = self.handlers.get(handler_id) {
            Ok(handler.borrow())
        } else {
            Err(ErrorCode::SystemCode(SystemCode::HandlerNotFound))
        }
    }
}
