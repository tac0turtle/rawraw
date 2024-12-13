//! Rust Cosmos SDK RFC 003 hypervisor/state-handler function implementation.
#![no_std]
extern crate alloc;

mod authz;
pub mod id_generator;
pub mod native_vm;
pub mod state_handler;

use crate::authz::AuthorizationMiddleware;
use crate::id_generator::IDGenerator;
use crate::state_handler::gas::GasMeter;
use crate::state_handler::{
    destroy_account_data, get_account_handler_id, init_next_account, update_handler_id,
    StateHandler,
};
use allocator_api2::vec::Vec;
use arrayvec::ArrayVec;
use core::alloc::Layout;
use core::cell::RefCell;
use ixc_core_macros::message_selector;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::code::ErrorCode::SystemCode;
use ixc_message_api::code::SystemCode::{AccountNotFound, CallStackOverflow, EncodingError, FatalExecutionError, HandlerNotFound, InvalidHandler, MessageNotHandled, UnauthorizedCallerAccess};
use ixc_message_api::handler::{Allocator, HostBackend};
use ixc_message_api::packet::MessagePacket;
use ixc_message_api::AccountID;
use ixc_vm_api::{ReadonlyStore, VM};

/// The default stack size for the account manager.
pub const DEFAULT_STACK_SIZE: usize = 128;

/// The account manager manages the execution, creation, and destruction of accounts.
pub struct AccountManager<'a, CM: VM, const CALL_STACK_LIMIT: usize = DEFAULT_STACK_SIZE> {
    code_manager: &'a CM,
}

impl<'a, CM: VM, const CALL_STACK_LIMIT: usize> AccountManager<'a, CM, CALL_STACK_LIMIT> {
    /// Creates a new account manager.
    pub fn new(code_manager: &'a CM) -> Self {
        Self { code_manager }
    }
}

impl<CM: VM, const CALL_STACK_LIMIT: usize> AccountManager<'_, CM, CALL_STACK_LIMIT> {
    /// Invokes a message packet in the context of the provided state handler.
    pub fn invoke_msg<'b, ST: StateHandler, IDG: IDGenerator, AUTHZ: AuthorizationMiddleware>(
        &'b self,
        state_handler: &'b mut ST,
        id_generator: &'b mut IDG,
        authz: &'b AUTHZ,
        message_packet: &mut MessagePacket,
        allocator: &'b dyn Allocator,
    ) -> Result<(), ErrorCode> {
        let mut exec_context = ExecContext {
            account_manager: self,
            state_handler,
            id_generator,
            authz,
            call_stack: Default::default(),
        };
        exec_context
            .call_stack
            .borrow_mut()
            .try_push(Frame {
                active_account: message_packet.header().caller,
            })
            .map_err(|_| SystemCode(CallStackOverflow))?;
        exec_context.invoke_msg(message_packet, allocator)
    }

    /// Invokes a message packet in the context of the provided state handler.
    pub fn invoke_query<ST: StateHandler>(
        &self,
        state_handler: &ST,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        let mut call_stack = ArrayVec::<Frame, CALL_STACK_LIMIT>::new();
        call_stack
            .try_push(Frame {
                active_account: message_packet.header().caller,
            })
            .map_err(|_| SystemCode(CallStackOverflow))?;
        let ref_cell = RefCell::new(call_stack);
        let query_ctx = QueryContext {
            account_manager: self,
            state_handler,
            call_stack: &ref_cell,
        };
        query_ctx.invoke_query(message_packet, allocator)
    }
}

struct ExecContext<
    'a,
    CM: VM,
    ST: StateHandler,
    IDG: IDGenerator,
    AUTHZ: AuthorizationMiddleware,
    const CALL_STACK_LIMIT: usize,
> {
    account_manager: &'a AccountManager<'a, CM, CALL_STACK_LIMIT>,
    state_handler: &'a mut ST,
    id_generator: &'a mut IDG,
    authz: &'a AUTHZ,
    call_stack: RefCell<ArrayVec<Frame, CALL_STACK_LIMIT>>,
}

#[derive(Debug)]
struct Frame {
    active_account: AccountID,
}

struct QueryContext<'b, 'a: 'b, CM: VM, ST: StateHandler, const CALL_STACK_LIMIT: usize> {
    account_manager: &'a AccountManager<'a, CM, CALL_STACK_LIMIT>,
    state_handler: &'a ST,
    call_stack: &'b RefCell<ArrayVec<Frame, CALL_STACK_LIMIT>>,
}

const ROOT_ACCOUNT: AccountID = AccountID::new(1);
const STATE_ACCOUNT: AccountID = AccountID::new(2);

/// Invoke a message packet in the context of the provided state handler.
impl<
        CM: VM,
        ST: StateHandler,
        IDG: IDGenerator,
        AUTHZ: AuthorizationMiddleware,
        const CALL_STACK_LIMIT: usize,
    > HostBackend for ExecContext<'_, CM, ST, IDG, AUTHZ, CALL_STACK_LIMIT>
{
    fn invoke_msg(
        &mut self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        let caller = message_packet.header().caller;
        let target_account = message_packet.header().account;
        let active_account = self
            .call_stack
            .borrow()
            .last()
            .map(|f| f.active_account)
            .ok_or(SystemCode(FatalExecutionError))?;

        // check if the caller matches the active account
        if caller != active_account {
            if target_account == STATE_ACCOUNT {
                // when calling the state handler, we NEVER allow impersonation
                return Err(SystemCode(UnauthorizedCallerAccess));
            }
            // otherwise we check the authorization middleware to see if impersonation is allowed
            self.authz
                .authorize(message_packet.header().caller, message_packet)?;
        }

        // if the target account is the state account, we can just run the state handler
        if target_account == STATE_ACCOUNT {
            return self.state_handler.handle_exec(message_packet, allocator);
        }

        let mut gas = GasMeter::new(message_packet.header().gas_left);

        // begin a transaction
        self.state_handler
            .begin_tx(&mut gas)
            .map_err(|_| SystemCode(InvalidHandler))?;
        // push the current caller onto the call stack
        message_packet.header_mut().gas_left = gas.get();

        let res = if target_account == ROOT_ACCOUNT {
            // if the target account is the root account, we can just run the system message
            self.handle_system_message(message_packet, allocator)
        } else {
            // push onto the call stack when we're calling a non-system account
            self.call_stack
                .borrow_mut()
                .try_push(Frame {
                    active_account: target_account,
                })
                .map_err(|_| SystemCode(CallStackOverflow))?;

            // find the account's handler ID
            let handler_id =
                get_account_handler_id(self.state_handler, target_account, &mut gas, allocator)?
                    .ok_or(SystemCode(AccountNotFound))?;

            // run the handler
            let handler = self.account_manager.code_manager.resolve_handler(
                &ReadOnlyStoreWrapper::wrap(self.state_handler, &mut gas),
                &handler_id,
                allocator,
            )?;
            let res = handler.handle_msg(message_packet, self, allocator);

            // pop the call stack
            self.call_stack.borrow_mut().pop();

            res
        };

        // commit or rollback the transaction
        if res.is_ok() {
            self.state_handler
                .commit_tx(&mut gas)
                .map_err(|_| SystemCode(InvalidHandler))?;
        } else {
            self.state_handler
                .rollback_tx(&mut gas)
                .map_err(|_| SystemCode(InvalidHandler))?;
        }

        res
    }

    fn invoke_query(
        &self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        // create a nested execution frame for the target account
        let query_ctx = QueryContext {
            account_manager: self.account_manager,
            state_handler: self.state_handler,
            call_stack: &self.call_stack,
        };
        //
        // we never pass the caller to query handlers and any value set in the caller field is ignored
        message_packet.header_mut().caller = AccountID::EMPTY;

        let target_account = message_packet.header().account;
        if target_account == STATE_ACCOUNT {
            return self.state_handler.handle_query(message_packet, allocator);
        }

        let mut gas = GasMeter::new(message_packet.header().gas_left);

        // find the account's handler ID
        let handler_id =
            get_account_handler_id(self.state_handler, target_account, &mut gas, allocator)?
                .ok_or(SystemCode(AccountNotFound))?;

        // run the handler
        let handler = self.account_manager.code_manager.resolve_handler(
            &ReadOnlyStoreWrapper::wrap(self.state_handler, &mut gas),
            &handler_id,
            allocator,
        )?;
        handler.handle_query(message_packet, &query_ctx, allocator)
    }
}

impl<'b, 'a: 'b, CM: VM, ST: StateHandler, const CALL_STACK_LIMIT: usize> HostBackend
    for QueryContext<'b, 'a, CM, ST, CALL_STACK_LIMIT>
{
    fn invoke_msg(
        &mut self,
        _message_packet: &mut MessagePacket,
        _allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        Err(SystemCode(
            ixc_message_api::code::SystemCode::VolatileAccessError,
        ))
    }

    fn invoke_query(
        &self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        let target_account = message_packet.header().account;
        if target_account == STATE_ACCOUNT {
            // the state account receives packets with the actual caller set
            message_packet.header_mut().caller =
                self.call_stack.borrow().last().unwrap().active_account;
            return self.state_handler.handle_query(message_packet, allocator);
        }


        // for all other accounts, we just set the caller to the empty account
        // because queries should depend on the caller
        message_packet.header_mut().caller = AccountID::EMPTY;

        if target_account == ROOT_ACCOUNT {
            return self.handle_system_query(message_packet, allocator);
        }

        let mut gas = GasMeter::new(message_packet.header().gas_left);

        // find the account's handler ID
        let handler_id =
            get_account_handler_id(self.state_handler, target_account, &mut gas, allocator)?
                .ok_or(SystemCode(AccountNotFound))?;

        // create a nested execution frame for the target account
        (*self.call_stack.borrow_mut())
            .try_push(Frame {
                active_account: target_account,
            })
            .map_err(|_| SystemCode(CallStackOverflow))?;

        // run the handler
        let handler = self.account_manager.code_manager.resolve_handler(
            &ReadOnlyStoreWrapper::wrap(self.state_handler, &mut gas),
            &handler_id,
            allocator,
        )?;

        let res = handler.handle_query(message_packet, self, allocator);

        // pop the call stack
        (*self.call_stack.borrow_mut()).pop();

        res
    }
}

impl<
        CM: VM,
        ST: StateHandler,
        IDG: IDGenerator,
        AUTHZ: AuthorizationMiddleware,
        const CALL_STACK_LIMIT: usize,
    > ExecContext<'_, CM, ST, IDG, AUTHZ, CALL_STACK_LIMIT>
{
    fn handle_system_message(
        &mut self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        unsafe {
            match message_packet.header().message_selector {
                CREATE_SELECTOR => self.handle_create(message_packet, allocator),
                MIGRATE_SELECTOR => self.handle_migrate(message_packet, allocator),
                SELF_DESTRUCT_SELECTOR => self.handle_self_destruct(message_packet),
                _ => Err(SystemCode(MessageNotHandled)),
            }
        }
    }

    unsafe fn handle_create(
        &mut self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        // get the input data
        let create_header = message_packet.header_mut();
        let handler_id = create_header.in_pointer1.get(message_packet);
        let init_data = create_header.in_pointer2.get(message_packet);

        let mut gas = GasMeter::new(message_packet.header().gas_left);

        // resolve the handler ID and retrieve the VM
        let handler_id = self
            .account_manager
            .code_manager
            .resolve_handler_id(
                &ReadOnlyStoreWrapper::wrap(self.state_handler, &mut gas),
                handler_id,
                allocator,
            )?
            .ok_or(SystemCode(HandlerNotFound))?;

        // get the next account ID and initialize the account storage
        let id = init_next_account(
            self.id_generator,
            self.state_handler,
            &handler_id,
            allocator,
            &mut gas,
        )
        .map_err(|_| SystemCode(InvalidHandler))?;

        // create a packet for calling on_create
        let mut on_create_packet =
            MessagePacket::allocate(allocator, 0).map_err(|_| SystemCode(FatalExecutionError))?;
        let on_create_header = on_create_packet.header_mut();
        on_create_header.account = id;
        on_create_header.caller = create_header.caller;
        on_create_header.message_selector = ON_CREATE_SELECTOR;
        on_create_header.in_pointer1.set_slice(init_data);

        // run the on_create handler
        let handler = self.account_manager.code_manager.resolve_handler(
            &ReadOnlyStoreWrapper::wrap(self.state_handler, &mut gas),
            &handler_id,
            allocator,
        )?;

        // push a frame onto the call stack
        self.call_stack
            .borrow_mut()
            .try_push(Frame { active_account: id })
            .map_err(|_| SystemCode(CallStackOverflow))?;

        let res = handler.handle_system(&mut on_create_packet, self, allocator);

        // pop the frame
        self.call_stack.borrow_mut().pop();

        let is_ok = match res {
            Ok(_) => true,
            // we accept the case where the handler doesn't have an on_create method
            Err(SystemCode(MessageNotHandled)) => true,
            _ => false,
        };

        if is_ok {
            // the result is ID of the newly created account
            let mut res = allocator
                .allocate(Layout::from_size_align_unchecked(16, 16))
                .map_err(|_| SystemCode(FatalExecutionError))?;
            let id: u128 = id.into();
            res.as_mut().copy_from_slice(&id.to_le_bytes());
            create_header.in_pointer1.set_slice(res.as_ref());
            Ok(())
        } else {
            res
        }
    }

    unsafe fn handle_migrate(
        &mut self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        // get the input data
        let migrate_header = message_packet.header_mut();
        let caller = migrate_header.caller;
        let new_handler_id = migrate_header.in_pointer1.get(message_packet);

        let mut gas = GasMeter::new(message_packet.header().gas_left);

        // get the old handler id
        let old_handler_id =
            get_account_handler_id(self.state_handler, caller, &mut gas, allocator)?
                .ok_or(SystemCode(AccountNotFound))?;

        // resolve the handler ID and retrieve the VM
        let new_handler_id = self
            .account_manager
            .code_manager
            .resolve_handler_id(
                &ReadOnlyStoreWrapper::wrap(self.state_handler, &mut gas),
                new_handler_id,
                allocator,
            )?
            .ok_or(SystemCode(HandlerNotFound))?;

        // update the handler ID
        update_handler_id(self.state_handler, caller, &new_handler_id, &mut gas)
            .map_err(|_| SystemCode(InvalidHandler))?;

        // create a packet for calling on_create
        let mut on_migrate_packet =
            MessagePacket::allocate(allocator, 0).map_err(|_| SystemCode(FatalExecutionError))?;
        let on_migrate_header = on_migrate_packet.header_mut();
        on_migrate_header.account = caller;
        on_migrate_header.caller = caller;
        on_migrate_header.message_selector = ON_MIGRATE_SELECTOR;
        on_migrate_header
            .in_pointer1
            .set_slice(old_handler_id.as_slice());

        // retrieve the handler
        let handler = self.account_manager.code_manager.resolve_handler(
            &ReadOnlyStoreWrapper::wrap(self.state_handler, &mut gas),
            &new_handler_id,
            allocator,
        )?;

        // execute the on-migrate packet with the system message handler
        handler.handle_system(&mut on_migrate_packet, self, allocator)
    }

    unsafe fn handle_self_destruct(
        &mut self,
        message_packet: &mut MessagePacket,
    ) -> Result<(), ErrorCode> {
        let mut gas = GasMeter::new(message_packet.header().gas_left);
        destroy_account_data(self.state_handler, message_packet.header().caller, &mut gas)
            .map_err(|_| SystemCode(FatalExecutionError))
    }
}

impl<'b, 'a: 'b, CM: VM, ST: StateHandler, const CALL_STACK_LIMIT: usize> QueryContext<'b, 'a, CM, ST, CALL_STACK_LIMIT> {
    fn handle_system_query(
        &self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        unsafe {
            match message_packet.header().message_selector {
                GET_HANDLER_ID_SELECTOR => self.handle_get_handler_id(message_packet, allocator),
                _ => Err(SystemCode(MessageNotHandled)),
            }
        }
    }

    unsafe fn handle_get_handler_id(
        &self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {

        // get the account ID from the in pointer
        let account_id = message_packet.header().in_pointer1.get(message_packet);
        if account_id.len() != 16 {
            return Err(SystemCode(EncodingError));
        }
        let account_id = u128::from_le_bytes(account_id.try_into().unwrap());

        // look up the handler ID
        let mut gas = GasMeter::new(message_packet.header().gas_left);
        let handler_id = get_account_handler_id(self.state_handler, AccountID::from(account_id), &mut gas, allocator)?
            .ok_or(SystemCode(AccountNotFound))?;

        // copy the handler ID to the out pointer
        let mut vec = Vec::new_in(allocator);
        vec.extend_from_slice(handler_id.as_slice());
        message_packet.header_mut().out_pointer1.set_slice(vec.as_slice());

        Ok(())
    }
}


const CREATE_SELECTOR: u64 = message_selector!("ixc.account.v1.create");
const ON_CREATE_SELECTOR: u64 = message_selector!("ixc.account.v1.on_create");
const MIGRATE_SELECTOR: u64 = message_selector!("ixc.account.v1.migrate");
const ON_MIGRATE_SELECTOR: u64 = message_selector!("ixc.account.v1.on_migrate");
const SELF_DESTRUCT_SELECTOR: u64 = message_selector!("ixc.account.v1.self_destruct");
const GET_HANDLER_ID_SELECTOR: u64 = message_selector!("ixc.account.v1.get_handler_id");

struct ReadOnlyStoreWrapper<'a, S: StateHandler> {
    state_handler: &'a S,
    gas: RefCell<&'a mut GasMeter>,
}

impl<'a, S: StateHandler> ReadOnlyStoreWrapper<'a, S> {
    fn wrap(state_handler: &'a S, gas: &'a mut GasMeter) -> Self {
        Self {
            state_handler,
            gas: RefCell::new(gas),
        }
    }
}

impl<S: StateHandler> ReadonlyStore for ReadOnlyStoreWrapper<'_, S> {
    fn get<'b>(
        &self,
        account_id: AccountID,
        key: &[u8],
        allocator: &'b dyn Allocator,
    ) -> Result<Option<Vec<u8, &'b dyn Allocator>>, ErrorCode> {
        let mut gas = self
            .gas
            .try_borrow_mut()
            .map_err(|_| SystemCode(FatalExecutionError))?;
        self.state_handler.kv_get(account_id, key, *gas, allocator)
    }
}
