//! Defines a VM manager that can be used to register VMs and resolve handler IDs.
use crate::CodeManager;
use allocator_api2::alloc::Allocator;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::handler::HostBackend;
use ixc_message_api::packet::MessagePacket;
use ixc_vm_api::{VM};
use std::collections::HashMap;
use crate::state_handler::Store;


/// A unique identifier for a handler implementation.
#[derive(Debug, Clone)]
pub struct HandlerID {
    // NOTE: encoding these as strings should be considered a temporary
    /// The unique identifier for the virtual machine that the handler is implemented in.
    pub vm: String,
    /// The unique identifier for the handler within the virtual machine.
    pub vm_handler_id: String,
}
/// Rust Cosmos SDK RFC 003 VM manager.
#[derive(Default)]
pub struct VMManager {
    vms: HashMap<String, Box<dyn VM>>,
    default_vm: Option<String>,
}

impl VMManager {
    /// Create a new hypervisor with the given state handler.
    pub fn new() -> Self {
        Self::default()
    }

    /// This is a hack until we figure out a better way to reference handler IDs.
    pub fn set_default_vm(&mut self, name: &str) -> Result<(), ()> {
        self.default_vm = Some(name.to_string());
        Ok(())
    }

    /// Register a VM with the hypervisor.
    pub fn register_vm(&mut self, name: &str, vm: Box<dyn VM>) -> Result<(), ()> {
        self.vms.insert(name.to_string(), vm);
        Ok(())
    }

    fn parse_handler_id(&self, value: &[u8]) -> Option<HandlerID> {
        parse_handler_id(value, &self.default_vm)
    }
}

impl VM for VMManager {
    fn resolve_handler_id<S: Store>(&self, store: &S, handler_id: &[u8]) -> Option<Vec<u8>> {
        todo!()
    }

    fn run_message<S: Store>(&self, store: &S, handler_id: &[u8], message_packet: &mut MessagePacket, backend: &mut dyn HostBackend, allocator: &dyn Allocator) -> Result<(), ErrorCode> {
        todo!()
    }

    fn run_query<S: Store>(&self, store: &S, handler_id: &[u8], message_packet: &mut MessagePacket, backend: &dyn HostBackend, allocator: &dyn Allocator) -> Result<(), ErrorCode> {
        todo!()
    }

    fn run_system_message<S: Store>(&self, store: &S, handler_id: &[u8], message_packet: &mut MessagePacket, backend: &mut dyn HostBackend, allocator: &dyn Allocator) -> Result<(), ErrorCode> {
        todo!()
    }
}

fn format_handler_id(HandlerID { vm, vm_handler_id }: &HandlerID) -> String {
    format!("{}:{}", vm, vm_handler_id)
}

fn parse_handler_id(value: &[u8], default_vm: &Option<String>) -> Option<HandlerID> {
    let str = String::from_utf8(value.to_vec()).ok()?;
    let mut parts = str.split(':');
    let mut vm = parts.next()?;
    let vm_handler_id = if let Some(handler_id) = parts.next() {
        handler_id
    } else {
        let handler_id = vm;
        if let Some(dvm) = default_vm {
            vm = dvm;
        } else {
            return None;
        }
        handler_id
    };
    Some(HandlerID {
        vm: vm.to_string(),
        vm_handler_id: vm_handler_id.to_string(),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_handler_id() {
        let value = b"vm1:handler1";
        let handler_id = super::parse_handler_id(value, &None).unwrap();
        assert_eq!(handler_id.vm, "vm1");
        assert_eq!(handler_id.vm_handler_id, "handler1");

        let value = b"handler1";
        let opt_handler_id = super::parse_handler_id(value, &None);
        assert!(opt_handler_id.is_none());

        let value = b"handler1";
        let handler_id = super::parse_handler_id(value, &Some("default".into())).unwrap();
        assert_eq!(handler_id.vm, "default");
        assert_eq!(handler_id.vm_handler_id, "handler1");
    }
}
