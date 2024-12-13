use core::cell::{Cell, RefCell};
use ixc_message_api::code::{ErrorCode, SystemCode};
use ixc_message_api::handler::{HostBackend, InvokeParams};
use ixc_message_api::AccountID;
use ixc_message_api::message::{Message, Request, Response};
use ixc_schema::mem::MemoryManager;

/// Context wraps a single message request (and possibly response as well) along with
/// the router callbacks necessary for making nested message calls.
pub struct Context<'a> {
    backend: BackendHandle<'a>,
    pub(crate) mem: &'a MemoryManager,
    pub(crate) account: AccountID, // 16 bytes
    pub(crate) caller: AccountID,  // 16 bytes
}

enum BackendHandle<'a> {
    Mut(&'a mut dyn HostBackend),
    Immutable(&'a dyn HostBackend),
    RefCell(&'a RefCell<dyn HostBackend>),
}

impl<'a> Context<'a> {
    /// Create a new context from a message packet and host callbacks with a pre-allocated memory manager.
    pub fn new(
        account: AccountID,
        caller: AccountID,
        _gas_left: u64, // TODO remove
        host_callbacks: &'a dyn HostBackend,
        mem: &'a MemoryManager,
    ) -> Self {
        Self {
            mem,
            backend: BackendHandle::Immutable(host_callbacks),
            account,
            caller,
        }
    }

    /// Creates a new context with mutable host backend and a pre-allocated memory manager.
    pub fn new_mut(
        account: AccountID,
        caller: AccountID,
        gas_left: u64,
        host_callbacks: &'a mut dyn HostBackend,
        mem: &'a MemoryManager,
    ) -> Self {
        Self {
            mem,
            backend: BackendHandle::Mut(host_callbacks),
            account,
            caller,
        }
    }

    /// Creates a new context with a RefCell host backend and a pre-allocated memory manager.
    /// This constructor is primarily intended for use in testing.
    pub fn new_ref_cell(
        account: AccountID,
        caller: AccountID,
        gas_left: u64,
        host_callbacks: &'a RefCell<dyn HostBackend>,
        mem: &'a MemoryManager,
    ) -> Self {
        Self {
            mem,
            backend: BackendHandle::RefCell(host_callbacks),
            account,
            caller,
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

    /// Get the memory manager.
    pub fn memory_manager(&self) -> &'a MemoryManager {
        self.mem
    }

    /// Dynamically invokes a message packet.
    /// This is marked unsafe because it should only be called by generated code or library functions.
    /// # Safety
    /// the function is marked as unsafe to detour users from calling it directly
    pub unsafe fn dynamic_invoke_msg(
        &mut self,
        msg: &Message,
    ) -> Result<Response<'a>, ErrorCode> {
        let invoke_params = InvokeParams::new(self.mem, None);
        match self.backend {
            BackendHandle::Mut(ref mut backend) => (*backend).invoke_msg(msg, &invoke_params),
            BackendHandle::RefCell(ref mut backend) => {
                if let Ok(mut backend) = backend.try_borrow_mut() {
                    (*backend).invoke_msg(msg, &invoke_params)
                } else {
                    Err(ErrorCode::SystemCode(SystemCode::VolatileAccessError))
                }
            }
            BackendHandle::Immutable(_) => {
                Err(ErrorCode::SystemCode(SystemCode::VolatileAccessError))
            }
        }
    }

    /// Dynamically invokes a query.
    /// # Safety
    /// This is marked unsafe because it should only be called by generated code or library functions.
    pub unsafe fn dynamic_invoke_query(&self, msg: &Message) -> Result<Response<'a>, ErrorCode> {
        let invoke_params = InvokeParams::new(self.mem, None);
        let backend = match self.backend {
            BackendHandle::Mut(ref backend) => *backend,
            BackendHandle::Immutable(backend) => backend,
            BackendHandle::RefCell(backend) => {
                return backend.borrow().invoke_query(msg, &invoke_params)
            }
        };
        backend.invoke_query(msg, &invoke_params)
    }

    /// Consume gas. Returns an out of gas error if there is not enough gas.
    pub fn consume_gas(&self, gas: u64) -> Result<(), ErrorCode> {
        todo!()
    }

    /// Update the state of the account.
    /// # Safety
    /// This is marked unsafe because it should only be called by library functions.
    pub unsafe fn dynamic_update_state(
        &mut self,
        req: &Request,
    ) -> Result<Response<'a>, ErrorCode> {
        let invoke_params = InvokeParams::new(self.mem, None);
        match self.backend {
            BackendHandle::Mut(ref mut backend) => (*backend).update_state(req, &invoke_params),
            BackendHandle::Immutable(_) => {
                Err(ErrorCode::SystemCode(SystemCode::VolatileAccessError))
            }
            BackendHandle::RefCell(ref mut backend) => {
                if let Ok(mut backend) = backend.try_borrow_mut() {
                    (*backend).update_state(req, &invoke_params)
                } else {
                    Err(ErrorCode::SystemCode(SystemCode::VolatileAccessError))
                }
            }
        }
    }

    /// Query the state of the account.
    /// # Safety
    /// This is marked unsafe because it should only be called by library functions.
    pub unsafe fn dynamic_query_state(
        &self,
        req: &Request,
    ) -> Result<Response<'a>, ErrorCode> {
        let invoke_params = InvokeParams::new(self.mem, None);
        let backend = match self.backend {
            BackendHandle::Mut(ref backend) => *backend,
            BackendHandle::Immutable(backend) => backend,
            BackendHandle::RefCell(backend) => {
                return backend.borrow().query_state(req, &invoke_params)
            }
        };
        backend.query_state(req, &invoke_params)
    }
}
