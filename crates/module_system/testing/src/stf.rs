use ixc_message_api::code::{ErrorCode, SystemCode};
use ixc_message_api::handler::{Allocator, HostBackend, RawHandler};
use ixc_message_api::packet::MessagePacket;
use ixc_vm_api::{HandlerDescriptor, ReadonlyStore, VM};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct NativeVM(Arc<RwLock<NativeVMImpl>>);

pub(crate) struct NativeVMImpl {
    handlers: HashMap<String, Box<dyn RawHandler>>,
}

impl NativeVM {
    pub fn new() -> NativeVM {
        NativeVM(Arc::new(RwLock::new(NativeVMImpl {
            handlers: HashMap::new(),
        })))
    }

    pub fn register_handler(&self, name: &str, handler: Box<dyn RawHandler>) {
        let mut vm = self.0.write().unwrap();
        vm.handlers.insert(name.to_string(), handler);
    }
}

impl VM for NativeVM {
    fn resolve_handler_id(&self, store: &dyn ReadonlyStore, handler_id: &[u8]) -> Option<allocator_api2::vec::Vec<u8>> {
        todo!()
    }

    fn run_message(&self, store: &dyn ReadonlyStore, handler_id: &[u8], message_packet: &mut MessagePacket, backend: &mut dyn HostBackend, allocator: &dyn Allocator) -> Result<(), ErrorCode> {
        todo!()
    }

    fn run_query(&self, store: &dyn ReadonlyStore, handler_id: &[u8], message_packet: &mut MessagePacket, backend: &dyn HostBackend, allocator: &dyn Allocator) -> Result<(), ErrorCode> {
        todo!()
    }

    fn run_system_message(&self, store: &dyn ReadonlyStore, handler_id: &[u8], message_packet: &mut MessagePacket, backend: &mut dyn HostBackend, allocator: &dyn Allocator) -> Result<(), ErrorCode> {
        todo!()
    }
    // fn describe_handler(&self, vm_handler_id: &str) -> Option<HandlerDescriptor> {
    //     let vm = self.0.read().unwrap();
    //     if vm.handlers.contains_key(vm_handler_id) {
    //         Some(HandlerDescriptor::default())
    //     } else {
    //         None
    //     }
    // }
    //
    // fn run_handler(
    //     &self,
    //     vm_handler_id: &str,
    //     message_packet: &mut MessagePacket,
    //     callbacks: &dyn HostBackend,
    //     allocator: &dyn Allocator,
    // ) -> Result<(), ErrorCode> {
    //     let vm = self.0.read().unwrap();
    //     if let Some(handler) = vm.handlers.get(vm_handler_id) {
    //         handler.handle(message_packet, callbacks, allocator)
    //     } else {
    //         Err(ErrorCode::SystemCode(SystemCode::HandlerNotFound))
    //     }
    // }
}
