#[cfg(feature = "std")]
extern crate alloc;
use ixc_message_api::code::{ErrorCode, SystemCode};
use ixc_message_api::handler::HostBackend;
use ixc_message_api::AccountID;
use ixc_schema::mem::MemoryManager;
use crate::result::ClientResult;

/// Context wraps a single message request (and possibly response as well) along with
/// the router callbacks necessary for making nested message calls.
pub struct Context<'a> {
    account_id: AccountID,
    caller_id: AccountID,
    backend: BackendHandle<'a>,
    pub(crate) mem: &'a MemoryManager,
}

enum BackendHandle<'a> {
    Mut(&'a mut dyn HostBackend),
    Immutable(&'a dyn HostBackend),
    #[cfg(feature = "std")]
    Boxed(alloc::boxed::Box<dyn HostBackend>),
}

impl<'a> Context<'a> {
    /// Create a new context from a message packet and host callbacks with a pre-allocated memory manager.
    pub fn new(
        account_id: &AccountID,
        host_callbacks: &'a dyn HostBackend,
        mem: &'a MemoryManager,
    ) -> Self {
        Self {
            account_id: *account_id,
            caller_id: AccountID::EMPTY,
            mem,
            backend: BackendHandle::Immutable(host_callbacks),
        }
    }

    /// Creates a new context with mutable host backend and a pre-allocated memory manager.
    pub fn new_mut(
        account_id: &AccountID,
        caller_id: &AccountID,
        host_callbacks: &'a mut dyn HostBackend,
        mem: &'a MemoryManager,
    ) -> Self {
        Self {
            account_id: *account_id,
            caller_id: *caller_id,
            mem,
            backend: BackendHandle::Mut(host_callbacks),
        }
    }

    /// Creates a new context with a Box host backend and a pre-allocated memory manager.
    /// This constructor is primarily intended for use in testing.
    #[cfg(feature = "std")]
    pub fn new_boxed(
        account_id: &AccountID,
        caller_id: &AccountID,
        host_callbacks: alloc::boxed::Box<dyn HostBackend>,
        mem: &'a MemoryManager,
    ) -> Self {
        Self {
            account_id: *account_id,
            caller_id: *caller_id,
            mem,
            backend: BackendHandle::Boxed(host_callbacks),
        }
    }

    /// This is the address of the account that is getting called.
    /// In a receiving account, this is the account's own address.
    pub fn self_account_id(&self) -> AccountID {
        self.account_id
    }

    /// This is the address of the account which is making the message call.
    ///
    /// For queries this will always be [`AccountID::EMPTY`].
    /// For system messages, this is the forwarded caller that was responsible
    /// for triggering the system message, any.
    /// For example, when an account is created, this is the account that called create.
    pub fn caller(&self) -> AccountID {
        self.caller_id
    }

    /// Consume gas. Returns an out of gas error if there is not enough gas.
    pub fn consume_gas(&mut self, gas: u64) -> ClientResult<()> {
        self.with_backend(|backend| backend.consume_gas(gas))?;
        Ok(())
    }

    /// Get the memory manager.
    pub fn memory_manager(&self) -> &'a MemoryManager {
        self.mem
    }

    /// Execute a closure directly on an immutable reference to the host backend.
    pub fn with_backend<R>(&self, f: impl FnOnce(&dyn HostBackend) -> R) -> R {
        match &self.backend {
            BackendHandle::Mut(backend) => f(*backend),
            BackendHandle::Immutable(backend) => f(*backend),
            BackendHandle::Boxed(backend) => f(backend.as_ref()),
        }
    }

    /// Execute a closure directly on a mutable reference to the host backend.
    pub fn with_backend_mut<R>(
        &mut self,
        f: impl FnOnce(&mut dyn HostBackend) -> R,
    ) -> Result<R, ErrorCode> {
        match self.backend {
            BackendHandle::Mut(ref mut backend) => Ok(f(*backend)),
            BackendHandle::Boxed(ref mut backend) => Ok(f(&mut **backend)),
            BackendHandle::Immutable(_) => {
                Err(ErrorCode::SystemCode(SystemCode::VolatileAccessError))
            }
        }
    }
}
