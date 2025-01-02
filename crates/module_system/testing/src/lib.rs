#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

mod store;

use crate::default_account::{DefaultAccount, DefaultAccountCreate};
use crate::store::VersionedMultiStore;
use allocator_api2::alloc::Allocator;
use ixc_account_manager::gas::GasMeter;
use ixc_account_manager::id_generator::IncrementingIDGenerator;
use ixc_account_manager::native_vm::{NativeVM, NativeVMImpl};
use ixc_account_manager::state_handler::std::StdStateHandler;
use ixc_account_manager::state_handler::StateHandler;
use ixc_account_manager::AccountManager;
#[doc(hidden)]
pub use ixc_core::account_api::create_account;
use ixc_core::account_api::{create_account_raw, ROOT_ACCOUNT};
use ixc_core::handler::{Client, Handler, HandlerClient};
use ixc_core::resource::{InitializationError, ResourceScope, Resources};
use ixc_core::result::ClientResult;
use ixc_core::Context;
use ixc_message_api::code::SystemCode::FatalExecutionError;
use ixc_message_api::code::{ErrorCode, SystemCode};
use ixc_message_api::error::HandlerError;
use ixc_message_api::handler::{HostBackend, InvokeParams, RawHandler};
use ixc_message_api::message::{Message, Request, Response};
use ixc_message_api::AccountID;
use ixc_schema::binary::NativeBinaryCodec;
use ixc_schema::codec::decode_value;
use ixc_schema::mem::MemoryManager;
use ixc_schema::structs::StructSchema;
use ixc_schema::SchemaValue;
use std::cell::Cell;
use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::sync::Mutex;

/// Defines a test harness for running tests against account and module implementations.
pub struct TestApp<V = NativeVMImpl> {
    mem: MemoryManager,
    mock_id: Cell<u64>,
    backend: Rc<Mutex<Backend<V>>>,
}

impl Default for TestApp<NativeVMImpl> {
    fn default() -> Self {
        let test_app = Self {
            backend: Rc::new(Mutex::new(Default::default())),
            mem: Default::default(),
            mock_id: Cell::new(0),
        };
        test_app.register_handler::<DefaultAccount>().unwrap();
        test_app
    }
}

impl<V: NativeVM + 'static> TestApp<V> {
    /// Registers a handler with the test harness so that accounts backed by this handler can be created.
    pub fn register_handler<H: Handler>(&self) -> Result<(), InitializationError> {
        let scope = ResourceScope::default();
        let mut backend = self.backend.lock().unwrap();
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
    ) -> Result<(), InitializationError> {
        let mut scope = ResourceScope::default();
        let mut backend = self.backend.lock().unwrap();
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
        Ok(client.target_account())
    }

    /// Creates a new random client account that can be used in calls and wraps it in a context.
    pub fn new_client_context(&self) -> ClientResult<Context> {
        let account_id = self.new_client_account()?;
        Ok(self.client_context_for(account_id))
    }

    /// Creates a new client for the given account.
    pub fn client_context_for(&self, account_id: AccountID) -> Context {
        let backend = BackendWrapper {
            account: account_id,
            backend: self.backend.clone(),
        };
        let ctx = Context::new_boxed(&account_id, &account_id, Box::new(backend), &self.mem);
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
            let mut backend = self.backend.lock().unwrap();
            backend.vm.register_handler(&handler_id, Box::new(mock));
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
        let mut ctx = self.client_context_for(client.target_account());
        f(&h, &mut ctx)
    }

    /// Get the events emitted during the last message execution.
    pub fn last_message_events(&self) -> EventLog {
        let backend = self.backend.lock().unwrap();
        EventLog {
            mem: &self.mem,
            events: backend.last_events.clone(),
        }
    }
}

#[derive(Default)]
struct Backend<V> {
    vm: V,
    state: VersionedMultiStore,
    id_gen: IncrementingIDGenerator,
    last_events: imbl::Vector<EventData>,
}

struct BackendWrapper<V> {
    account: AccountID,
    backend: Rc<Mutex<Backend<V>>>,
}

impl<V: ixc_vm_api::VM> HostBackend for BackendWrapper<V> {
    fn invoke_msg<'a>(
        &mut self,
        message: &Message,
        invoke_params: &InvokeParams<'a, '_>,
    ) -> Result<Response<'a>, ErrorCode> {
        let mut backend = self.backend.lock().unwrap();
        let mut tx = backend.state.new_transaction();
        let mut state = StdStateHandler::new(&mut tx, Default::default());
        let account_manager: AccountManager<V> = AccountManager::new(&backend.vm);
        let res = account_manager.invoke_msg(
            &mut state,
            &backend.id_gen,
            self.account,
            message,
            invoke_params,
        )?;
        let events = backend
            .state
            .commit(tx)
            .map_err(|_| ErrorCode::SystemCode(FatalExecutionError))?;
        backend.last_events = events;
        Ok(res)
    }

    fn invoke_query<'a>(
        &self,
        message: &Message,
        invoke_params: &InvokeParams<'a, '_>,
    ) -> Result<Response<'a>, ErrorCode> {
        // TODO add a read only state handler impl for query
        let backend = self.backend.lock().unwrap();
        let mut tx = backend.state.new_transaction();
        let state = StdStateHandler::new(&mut tx, Default::default());
        let account_manager: AccountManager<V> = AccountManager::new(&backend.vm);
        account_manager.invoke_query(&state, message, invoke_params)
    }

    fn update_state<'a>(
        &mut self,
        req: &Request,
        invoke_params: &InvokeParams<'a, '_>,
    ) -> Result<Response<'a>, ErrorCode> {
        let mut backend = self.backend.lock().unwrap();
        let mut tx = backend.state.new_transaction();
        let mut state = StdStateHandler::new(&mut tx, Default::default());
        let res = state.handle_exec(
            self.account,
            req,
            &GasMeter::unlimited(),
            invoke_params.allocator,
        )?;
        backend
            .state
            .commit(tx)
            .map_err(|_| ErrorCode::SystemCode(FatalExecutionError))?;
        Ok(res)
    }

    fn query_state<'a>(
        &self,
        req: &Request,
        invoke_params: &InvokeParams<'a, '_>,
    ) -> Result<Response<'a>, ErrorCode> {
        let backend = self.backend.lock().unwrap();
        let mut tx = backend.state.new_transaction();
        let state = StdStateHandler::new(&mut tx, Default::default());
        state.handle_query(
            self.account,
            req,
            &GasMeter::unlimited(),
            invoke_params.allocator,
        )
    }

    fn consume_gas(&self, _gas: u64) -> Result<(), ErrorCode> {
        Ok(())
    }

    fn out_of_gas(&self) -> Result<bool, ErrorCode> {
        Ok(false)
    }
}

/// Defines a mock handler composed of mock handler API trait implementations.
pub struct MockHandler {
    mocks: Vec<Box<dyn RawHandler>>,
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
    pub fn add_handler<T: RawHandler + ?Sized + 'static>(&mut self, mock: Box<T>) {
        self.mocks.push(Box::new(MockWrapper::<T>(mock)));
    }

    /// Creates a mock handler for one mock handler API trait implementation.
    pub fn of<T: RawHandler + ?Sized + 'static>(mock: Box<T>) -> Self {
        let mut mocks = MockHandler::new();
        mocks.add_handler(Box::new(MockWrapper::<T>(mock)));
        mocks
    }
}

impl RawHandler for MockHandler {
    fn handle_msg<'a>(
        &self,
        caller: &AccountID,
        message: &Message,
        callbacks: &mut dyn HostBackend,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, HandlerError> {
        for mock in &self.mocks {
            let res = mock.handle_msg(caller, message, callbacks, allocator);
            match res {
                Err(HandlerError {
                    code: ErrorCode::SystemCode(SystemCode::MessageNotHandled),
                    ..
                }) => continue,
                _ => return res,
            }
        }
        Err(SystemCode::MessageNotHandled.into())
    }

    fn handle_query<'a>(
        &self,
        message: &Message,
        callbacks: &dyn HostBackend,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, HandlerError> {
        for mock in &self.mocks {
            let res = mock.handle_query(message, callbacks, allocator);
            match res {
                Err(HandlerError {
                    code: ErrorCode::SystemCode(SystemCode::MessageNotHandled),
                    ..
                }) => continue,
                _ => return res,
            }
        }
        Err(SystemCode::MessageNotHandled.into())
    }
}

struct MockWrapper<T: RawHandler + ?Sized>(Box<T>);
impl<T: RawHandler + ?Sized> RawHandler for MockWrapper<T> {
    fn handle_query<'a>(
        &self,
        message_packet: &Message,
        callbacks: &dyn HostBackend,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ixc_message_api::error::HandlerError> {
        self.0.handle_query(message_packet, callbacks, allocator)
    }

    fn handle_msg<'a>(
        &self,
        caller: &AccountID,
        message_packet: &Message,
        callbacks: &mut dyn HostBackend,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, HandlerError> {
        self.0
            .handle_msg(caller, message_packet, callbacks, allocator)
    }

    fn handle_system<'a>(
        &self,
        caller: &AccountID,
        message_packet: &Message,
        callbacks: &mut dyn HostBackend,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, HandlerError> {
        self.0
            .handle_system(caller, message_packet, callbacks, allocator)
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

#[derive(Clone)]
/// The events captured by the test harness.
pub struct EventLog<'a> {
    mem: &'a MemoryManager,
    events: imbl::Vector<EventData>,
}

impl Debug for EventLog<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.events.iter()).finish()
    }
}

impl Iterator for EventLog<'_> {
    type Item = EventData;

    fn next(&mut self) -> Option<Self::Item> {
        self.events.pop_front()
    }
}

impl<'a> EventLog<'a> {
    /// Get the number of events in the log.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Check if the log is empty.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Select all events of a specific type emitted by a specific account.
    pub fn select<T: SchemaValue<'a> + StructSchema>(&'a self, sender: AccountID) -> Vec<T> {
        let mut res = vec![];
        for event in self.events.iter() {
            if event.sender == sender {
                if let Some(data) = event.try_decode::<T>(self.mem) {
                    res.push(data);
                }
            }
        }
        res
    }
}

/// The event data captured by the test harness.
#[derive(Default, Clone, Debug)]
pub struct EventData {
    /// The account that emitted the event.
    pub sender: AccountID,
    /// The type selector of the event.
    pub type_selector: u64,
    /// The event data.
    pub data: Vec<u8>,
}

impl EventData {
    /// Try to decode the event data as a struct.
    pub fn try_decode<'a, E: StructSchema + SchemaValue<'a>>(
        &'a self,
        mem: &'a MemoryManager,
    ) -> Option<E> {
        if self.type_selector != E::TYPE_SELECTOR {
            return None;
        }
        let cdc = NativeBinaryCodec;
        decode_value(&cdc, self.data.as_slice(), mem).unwrap_or(None)
    }
}
