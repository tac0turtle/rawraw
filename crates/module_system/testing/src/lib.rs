#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

mod store;

use crate::default_account::{DefaultAccount, DefaultAccountCreate};
use crate::store::{Tx, VersionedMultiStore};
use allocator_api2::alloc::Allocator;
use ixc_account_manager::id_generator::IncrementingIDGenerator;
use ixc_account_manager::native_vm::{NativeVM, NativeVMImpl};
use ixc_account_manager::state_handler::std::StdStateHandler;
use ixc_account_manager::AccountManager;
#[doc(hidden)]
pub use ixc_core::account_api::create_account;
use ixc_core::account_api::{create_account_raw, ROOT_ACCOUNT};
use ixc_core::handler::{Client, Handler, HandlerClient};
use ixc_core::resource::{InitializationError, ResourceScope, Resources};
use ixc_core::result::ClientResult;
use ixc_core::Context;
use ixc_message_api::code::{ErrorCode, SystemCode};
use ixc_message_api::handler::{HostBackend, InvokeParams, RawHandler};
use ixc_message_api::message::{Message, Request, Response};
use ixc_message_api::AccountID;
use ixc_schema::mem::MemoryManager;
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;

/// Defines a test harness for running tests against account and module implementations.
pub struct TestApp<V = NativeVMImpl> {
    mem: MemoryManager,
    mock_id: Cell<u64>,
    backend: RefCell<Backend<V>>,
}

impl Default for TestApp<NativeVMImpl> {
    fn default() -> Self {
        let test_app = Self {
            backend: RefCell::new(Default::default()),
            mem: Default::default(),
            mock_id: Cell::new(0),
        };
        test_app.register_handler::<DefaultAccount>().unwrap();
        test_app
    }
}

impl<V: NativeVM + 'static> TestApp<V> {
    /// Registers a handler with the test harness so that accounts backed by this handler can be created.
    pub fn register_handler<H: Handler>(&self) -> core::result::Result<(), InitializationError> {
        let scope = ResourceScope::default();
        let mut backend = self.backend.borrow_mut();
        unsafe {
            backend
                .vm
                .register_handler(H::NAME, Box::new(H::new(&scope)?));
        }
        Ok(())
    }

    /// Registers a handler with the test harness so that accounts backed by this handler can be created.
    /// This version of the function also registers the handler's client bindings.
    pub fn register_handler_with_bindings<H: Handler>(
        &self,
        client_bindings: &[(&'static str, AccountID)],
    ) -> core::result::Result<(), InitializationError> {
        let mut scope = ResourceScope::default();
        let mut backend = self.backend.borrow_mut();
        let binding_map = BTreeMap::<&str, AccountID>::from_iter(client_bindings.iter().cloned());
        scope.account_resolver = Some(&binding_map);
        unsafe {
            backend
                .vm
                .register_handler(H::NAME, Box::new(H::new(&scope)?));
        }
        Ok(())
    }

    /// Creates a new random client account that can be used in calls.
    pub fn new_client_account(&self) -> ClientResult<AccountID> {
        let mut ctx = self.client_context_for(ROOT_ACCOUNT);
        let client = create_account::<DefaultAccount>(&mut ctx, DefaultAccountCreate {})?;
        Ok(client.account_id())
    }

    /// Creates a new random client account that can be used in calls and wraps it in a context.
    pub fn new_client_context(&self) -> ClientResult<Context> {
        let account_id = self.new_client_account()?;
        Ok(self.client_context_for(account_id))
    }

    /// Creates a new client for the given account.
    pub fn client_context_for(&self, account_id: AccountID) -> Context {
        let ctx = Context::new_ref_cell(account_id, account_id, 0, &self.backend, &self.mem);
        ctx
    }

    /// Adds a mock account handler to the test harness, instantiates it as an account and returns the account ID.
    pub fn add_mock(&self, mock: MockHandler) -> ClientResult<AccountID> {
        let mut root = self.client_context_for(ROOT_ACCOUNT);
        let mock_id = self.mock_id.get();
        self.mock_id.set(mock_id + 1);
        let handler_id = format!("mock{}", mock_id);
        {
            // we need a scope here because we borrow the backend mutably
            // and we want to release the borrow before we call create_account_raw
            // because that will mutably borrow the backend again
            let mut backend = self.backend.borrow_mut();
            backend
                .vm
                .register_handler(&handler_id, std::boxed::Box::new(mock));
        }
        create_account_raw(&mut root, &handler_id, &[])
    }

    /// Executes a function in the context of a handler.
    /// This provides a way for tests to peek inside and manipulate a handler's state directly.
    /// This method will panic if we can't call into the handler, but panicking is acceptable in tests.
    pub fn exec_in<HC: HandlerClient, F, R>(&self, client: &HC, f: F) -> R
    where
        F: FnOnce(&HC::Handler, &mut Context) -> R,
    {
        // TODO lookup handler ID to make sure this is the correct handler
        let scope = ResourceScope::default();
        let h = unsafe { HC::Handler::new(&scope) }.unwrap();
        let mut ctx = self.client_context_for(client.account_id());
        f(&h, &mut ctx)
    }
}

#[derive(Default)]
struct Backend<V> {
    vm: V,
    account: AccountID,
    state: VersionedMultiStore,
    id_gen: IncrementingIDGenerator,
}

impl<V: ixc_vm_api::VM> Backend<V> {
    fn init_operation(&self) -> (AccountManager<V>, Tx, StdStateHandler<Tx>) {
        let account_manager: AccountManager<V> = AccountManager::new(&self.vm);
        let mut tx = self.state.new_transaction();
        let mut state_handler = StdStateHandler::new(&mut tx, Default::default());
        (account_manager, tx, state_handler)
    }

    fn exec_context(&mut self, caller: AccountID) -> (Tx, impl HostBackend) {
        let account_manager: AccountManager<V> = AccountManager::new(&self.vm);
        let mut tx = self.state.new_transaction();
        let mut state_handler = StdStateHandler::new(&mut tx, Default::default());
        let exec_context = account_manager.exec_context(caller, &mut state_handler, &mut self.id_gen);
        (tx, exec_context)
    }
}


impl<V: ixc_vm_api::VM> HostBackend for Backend<V> {
    fn invoke_msg<'a>(&mut self, message: &Message, invoke_params: &InvokeParams<'a>) -> Result<Response<'a>, ErrorCode> {
        todo!()
    }

    fn invoke_query<'a>(&self, message: &Message, invoke_params: &InvokeParams<'a>) -> Result<Response<'a>, ErrorCode> {
        todo!()
    }

    fn update_state<'a>(&mut self, req: &Request, invoke_params: &InvokeParams<'a>) -> Result<Response<'a>, ErrorCode> {
        todo!()
    }

    fn query_state<'a>(&self, req: &Request, invoke_params: &InvokeParams<'a>) -> Result<Response<'a>, ErrorCode> {
        todo!()
    }

    fn consume_gas(&self, gas: u64) -> Result<(), ErrorCode> {
        Ok(())
    }

    fn self_account_id(&self) -> AccountID {
        self.account
    }

    fn caller(&self) -> AccountID {
        AccountID::EMPTY
    }
    // fn invoke_msg(
    //     &mut self,
    //     message_packet: &mut MessagePacket,
    //     allocator: &dyn Allocator,
    // ) -> Result<(), ErrorCode> {
    //     let (tx, mut exec_context) = self.exec_context(message_packet.header().caller);
    //
    //     exec_context.invoke_msg(message_packet, allocator)?;
    //
    //     self.state
    //         .commit(tx)
    //         .map_err(|_| ErrorCode::SystemCode(FatalExecutionError))
    // }
    //
    // fn invoke_query(
    //     &self,
    //     message_packet: &mut MessagePacket,
    //     allocator: &dyn Allocator,
    // ) -> Result<(), ErrorCode> {
    //     let (account_manager, mut tx, mut state_handler) = self.init_operation();
    //
    //     account_manager.invoke_query(&state_handler, message_packet, allocator)
    // }
    //
    // fn update_state<'a>(&mut self, req: &StateRequest, allocator: &'a dyn Allocator) -> Result<UpdateStateResponse<'a>, ErrorCode> {
    //     let (tx, mut exec_context) = self.exec_context(req.header().caller);
    // }
    //
    // fn query_state<'a>(&self, req: &StateRequest, allocator: &'a dyn Allocator) -> Result<QueryStateResponse<'a>, ErrorCode> {
    //     todo!()
    // }
    //
    // fn consume_gas(&self, gas: u64) -> Result<(), ErrorCode> {
    //     todo!()
    // }
}

/// Defines a mock handler composed of mock handler API trait implementations.
pub struct MockHandler {
    mocks: Vec<std::boxed::Box<dyn RawHandler>>,
}

impl Default for MockHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl MockHandler {
    /// Creates a new mock handler.
    pub fn new() -> Self {
        MockHandler { mocks: Vec::new() }
    }

    /// Adds a mock handler API trait implementation to the mock handler.
    pub fn add_handler<T: RawHandler + ?Sized + 'static>(&mut self, mock: std::boxed::Box<T>) {
        self.mocks.push(Box::new(MockWrapper::<T>(mock)));
    }

    /// Creates a mock handler for one mock handler API trait implementation.
    pub fn of<T: RawHandler + ?Sized + 'static>(mock: std::boxed::Box<T>) -> Self {
        let mut mocks = MockHandler::new();
        mocks.add_handler(Box::new(MockWrapper::<T>(mock)));
        mocks
    }
}

impl RawHandler for MockHandler {
    fn handle_msg<'a>(
        &self,
        message: &Request,
        callbacks: &mut dyn HostBackend,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        for mock in &self.mocks {
            let res = mock.handle_msg(message, callbacks, allocator);
            match res {
                Err(ErrorCode::SystemCode(SystemCode::MessageNotHandled)) => continue,
                _ => return res,
            }
        }
        Err(ErrorCode::SystemCode(SystemCode::MessageNotHandled))
    }

    fn handle_query<'a>(
        &self,
        message: &Request,
        callbacks: &dyn HostBackend,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        for mock in &self.mocks {
            let res = mock.handle_query(message, callbacks, allocator);
            match res {
                Err(ErrorCode::SystemCode(SystemCode::MessageNotHandled)) => continue,
                _ => return res,
            }
        }
        Err(ErrorCode::SystemCode(SystemCode::MessageNotHandled))
    }
}

struct MockWrapper<T: RawHandler + ?Sized>(std::boxed::Box<T>);
impl<T: RawHandler + ?Sized> RawHandler for MockWrapper<T> {
    fn handle_query<'a>(
        &self,
        message_packet: &Request,
        callbacks: &dyn HostBackend,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        self.0.handle_query(message_packet, callbacks, allocator)
    }

    fn handle_msg<'a>(
        &self,
        message_packet: &Request,
        callbacks: &mut dyn HostBackend,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        self.0.handle_msg(message_packet, callbacks, allocator)
    }

    fn handle_system<'a>(
        &self,
        message_packet: &Request,
        callbacks: &mut dyn HostBackend,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        self.0.handle_system(message_packet, callbacks, allocator)
    }
}

#[ixc::handler(DefaultAccount)]
mod default_account {
    use ixc::*;

    #[derive(Resources)]
    pub struct DefaultAccount {}

    impl DefaultAccount {
        #[on_create]
        pub fn create(&self, _ctx: &mut Context) -> Result<()> {
            Ok(())
        }
    }
}
