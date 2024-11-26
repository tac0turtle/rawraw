use crate::error::ClientError;
use crate::low_level::create_packet;
use crate::message::Message;
use crate::result::ClientResult;
use alloc::string::String;
use core::cell::Cell;
use ixc_message_api::code::{ErrorCode, SystemCode};
use ixc_message_api::handler::HostBackend;
use ixc_message_api::AccountID;
use ixc_schema::codec::Codec;
use ixc_schema::mem::MemoryManager;
use ixc_schema::value::OptionalValue;

/// Context wraps a single message request (and possibly response as well) along with
/// the router callbacks necessary for making nested message calls.
pub struct Context<'a> {
    pub(self) mem: MemHandle<'a>,
    pub(crate) backend: &'a mut dyn HostBackend,
    pub(crate) account: AccountID, // 16 bytes
    pub(crate) caller: AccountID,  // 16 bytes
    #[allow(unused)]
    gas_left: Cell<u64>,
}

enum MemHandle<'a> {
    Borrowed(&'a MemoryManager),
    Owned(MemoryManager),
}
// enum BackendHandle<'a> {
//     Mut(&'a mut dyn HostBackend),
//     Immutable(&'a dyn HostBackend),
// }

impl<'a> Context<'a> {
    /// Create a new context from a message packet and host callbacks.
    pub fn new(
        account: AccountID,
        caller: AccountID,
        gas_left: u64,
        host_callbacks: &'a dyn HostBackend,
    ) -> Self {
        // Self {
        //     mem: MemHandle::Owned(MemoryManager::new()),
        //     backend: BackendHandle::Immutable(host_callbacks),
        //     account,
        //     caller,
        //     gas_left: Cell::new(gas_left),
        // }
        todo!()
    }

    /// Create a new context from a message packet and host callbacks with a pre-allocated memory manager.
    pub fn new_with_mem(
        account: AccountID,
        caller: AccountID,
        gas_left: u64,
        host_callbacks: &'a dyn HostBackend,
        mem: &'a MemoryManager,
    ) -> Self {
        // Self {
        //     mem: MemHandle::Borrowed(mem),
        //     backend: BackendHandle::Immutable(host_callbacks),
        //     account,
        //     caller,
        //     gas_left: Cell::new(gas_left),
        // }
        todo!()
    }

    pub fn new_mut(
        account: AccountID,
        caller: AccountID,
        gas_left: u64,
        host_callbacks: &'a mut dyn HostBackend,
    ) -> Self {
        Self {
            mem: MemHandle::Owned(MemoryManager::new()),
            backend: host_callbacks,
            account,
            caller,
            gas_left: Cell::new(gas_left),
        }
    }

    /// This is the address of the account that is getting called.
    /// In a receiving account, this is the account's own address.
    pub fn self_account_id(&self) -> AccountID {
        self.account
    }

    /// This is the address of the account which is making the message call.
    pub fn caller(&self) -> AccountID {
        self.caller
    }

    /// Get the host backend.
    pub unsafe fn host_backend(&self) -> Option<&dyn HostBackend> {
        // match self.backend {
        //     BackendHandle::Immutable(backend) => Some(backend),
        //     BackendHandle::Mut(_) => None,
        // }
        todo!()
    }

    pub unsafe fn host_backend_mut(
        &mut self,
        host_backend: &mut dyn HostBackend,
    ) -> Option<&mut dyn HostBackend> {
        // match self.backend {
        //     BackendHandle::Immutable(_) => None,
        //     BackendHandle::Mut(host_backend) => Some(host_backend),
        // }
        todo!()
    }

    /// Get the memory manager.
    pub fn memory_manager(&self) -> &MemoryManager {
        self.mem.get()
    }

    pub fn dynamic_invoke_msg<'b, M: Message<'b>>(
        &mut self,
        account: AccountID,
        message: M,
    ) -> ClientResult<<M::Response<'a> as OptionalValue<'a>>::Value, M::Error> {
        unsafe {
            // encode the message body
            let mem = &self.mem;
            let mem = mem.get();
            let cdc = M::Codec::default();
            let msg_body = cdc.encode_value(&message, mem)?;

            // create the message packet and fill in call details
            let mut packet = create_packet(self, account, M::SELECTOR)?;
            let header = packet.header_mut();
            header.in_pointer1.set_slice(msg_body);

            // invoke the message
            let res = self.backend.invoke_msg(&mut packet, mem);

            let out1 = header.out_pointer1.get(&packet);

            match res {
                Ok(_) => {
                    let res = M::Response::<'a>::decode_value(&cdc, out1, mem)?;
                    Ok(res)
                }
                Err(e) => {
                    let c: u16 = e.into();
                    let code = ErrorCode::<M::Error>::from(c);
                    let msg = String::from_utf8(out1.to_vec())
                        .map_err(|_| ErrorCode::SystemCode(SystemCode::EncodingError))?;
                    Err(ClientError { message: msg, code })
                }
            }
        }
    }
}

impl MemHandle<'_> {
    pub fn get(&self) -> &MemoryManager {
        match self {
            MemHandle::Borrowed(mem) => mem,
            MemHandle::Owned(mem) => mem,
        }
    }
}
