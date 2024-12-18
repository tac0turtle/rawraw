use core::cell::RefCell;
use core::borrow::Borrow;
use ixc_message_api::code::{ErrorCode, SystemCode};
use ixc_message_api::handler::{HostBackend, InvokeParams};
use ixc_message_api::message::{Message, Request, Response};
use ixc_message_api::AccountID;
use ixc_schema::mem::MemoryManager;

/// Context wraps a single message request (and possibly response as well) along with
/// the router callbacks necessary for making nested message calls.
pub struct Context<'a> {
    backend: BackendHandle<'a>,
    pub(crate) mem: &'a MemoryManager,
}

enum BackendHandle<'a> {
    Mut(&'a mut dyn HostBackend),
    Immutable(&'a dyn HostBackend),
    RefCell(&'a RefCell<dyn HostBackend>),
}

impl<'a> Context<'a> {
    /// Create a new context from a message packet and host callbacks with a pre-allocated memory manager.
    pub fn new(
        host_callbacks: &'a dyn HostBackend,
        mem: &'a MemoryManager,
    ) -> Self {
        Self {
            mem,
            backend: BackendHandle::Immutable(host_callbacks),
        }
    }

    /// Creates a new context with mutable host backend and a pre-allocated memory manager.
    pub fn new_mut(
        host_callbacks: &'a mut dyn HostBackend,
        mem: &'a MemoryManager,
    ) -> Self {
        Self {
            mem,
            backend: BackendHandle::Mut(host_callbacks),
        }
    }

    /// Creates a new context with a RefCell host backend and a pre-allocated memory manager.
    /// This constructor is primarily intended for use in testing.
    pub fn new_ref_cell(
        host_callbacks: &'a RefCell<dyn HostBackend>,
        mem: &'a MemoryManager,
    ) -> Self {
        Self {
            mem,
            backend: BackendHandle::RefCell(host_callbacks),
        }
    }

    /// This is the address of the account that is getting called.
    /// In a receiving account, this is the account's own address.
    pub fn self_account_id(&self) -> AccountID {
        self.with_backend(|backend| backend.self_account_id())
    }

    /// This is the address of the account which is making the message call.
    pub fn caller(&self) -> AccountID {
        self.with_backend(|backend| backend.caller())
    }

    /// Consume gas. Returns an out of gas error if there is not enough gas.
    pub fn consume_gas(&mut self, gas: u64) -> Result<(), ErrorCode> {
        self.with_backend(|backend| backend.consume_gas(gas))
    }

    /// Get the memory manager.
    pub fn memory_manager(&self) -> &'a MemoryManager {
        self.mem
    }

    /// Execute a closure directly on an immutable reference to the host backend.
    pub fn with_backend<R>(&self, f: impl FnOnce(&dyn HostBackend) -> R) -> R {
        match self.backend {
            BackendHandle::Mut(ref backend) => f(*backend),
            BackendHandle::Immutable(ref backend) => f(*backend),
            BackendHandle::RefCell(backend) => f(&*backend.borrow()),
        }
    }

    /// Execute a closure directly on a mutable reference to the host backend.
    pub fn with_backend_mut<R>(&mut self, f: impl FnOnce(&mut dyn HostBackend) -> R) -> core::result::Result<R, ErrorCode> {
        match self.backend {
            BackendHandle::Mut(ref mut backend) => Ok(f(*backend)),
            BackendHandle::RefCell(ref mut backend) => {
                if let Ok(mut backend) = backend.try_borrow_mut() {
                    Ok(f(&mut *backend))
                } else {
                    Err(ErrorCode::SystemCode(SystemCode::VolatileAccessError))
                }
            }
            BackendHandle::Immutable(_) => {
                Err(ErrorCode::SystemCode(SystemCode::VolatileAccessError))
            }
        }
    }
}
