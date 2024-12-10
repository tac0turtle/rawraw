//! Rust Cosmos SDK RFC 003 hypervisor/state-handler function implementation.

mod authz;
pub mod id_generator;
pub mod state_handler;
pub mod vm_manager;

use crate::authz::AuthorizationMiddleware;
use crate::id_generator::IDGenerator;
use crate::state_handler::{get_account_handler_id, StateHandler};
use allocator_api2::vec::Vec;
use core::borrow::BorrowMut;
use ixc_core_macros::message_selector;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::code::ErrorCode::SystemCode;
use ixc_message_api::code::SystemCode::{
    AccountNotFound, FatalExecutionError, InvalidHandler, MessageNotHandled,
    UnauthorizedCallerAccess,
};
use ixc_message_api::handler::{Allocator, HostBackend};
use ixc_message_api::packet::MessagePacket;
use ixc_message_api::AccountID;
use ixc_vm_api::{ReadonlyStore, VM};
use std::cell::RefCell;
use crate::state_handler::gas::GasMeter;

/// The account manager manages the execution, creation, and destruction of accounts.
pub struct AccountManager<'a, CM: VM> {
    code_manager: &'a CM,
    call_stack_limit: usize,
}

impl<'a, CM: VM> AccountManager<'a, CM> {
    /// Creates a new account manager.
    pub fn new(code_manager: &'a CM) -> Self {
        Self {
            code_manager,
            call_stack_limit: 128,
        }
    }
}

impl<'a, CM: VM> AccountManager<'a, CM>
{
    /// Invokes a message packet in the context of the provided state handler.
    pub fn invoke_msg<'b:'a, ST: StateHandler<&'b dyn Allocator>, IDG: IDGenerator, AUTHZ: AuthorizationMiddleware>(
        &'b self,
        state_handler: &'b mut ST,
        id_generator: &'b mut IDG,
        authz: &'b AUTHZ,
        message_packet: &mut MessagePacket,
        allocator: &'b dyn Allocator,
    ) -> Result<(), ErrorCode> {
        let mut exec_context = ExecContext{
            account_manager: self,
            state_handler,
            id_generator,
            authz,
            call_stack: Vec::new_in(allocator),
        };
        exec_context.invoke_msg(message_packet, allocator)
    }

    /// Invokes a message packet in the context of the provided state handler.
    pub fn invoke_query<'b, ST: StateHandler<&'b dyn Allocator>>(
        &self,
        state_handler: &'b ST,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        // let mut query_context = QueryContext{
        // let mut exec_context = ExecContext{
        //     account_manager: self,
        //     state_handler,
        //     id_generator,
        //     authz,
        //     call_stack: Vec::new_in(allocator),
        // };
        // exec_context.invoke(message_packet, allocator)
        todo!()
    }
}

struct ExecContext<
    'a,
    CM: VM,
    ST: StateHandler<&'a dyn Allocator>,
    IDG: IDGenerator,
    AUTHZ: AuthorizationMiddleware,
> {
    account_manager: &'a AccountManager<'a, CM>,
    state_handler: &'a mut ST,
    id_generator: &'a mut IDG,
    authz: &'a AUTHZ,
    call_stack: Vec<Frame, &'a dyn Allocator>,
}

struct Frame {
    active_account: AccountID,
}

struct QueryContext<'a, CM: VM, ST: StateHandler<&'a dyn Allocator>> {
    account_manager: &'a AccountManager<'a, CM>,
    state_handler: &'a ST,
    call_stack: RefCell<Vec<Frame, &'a dyn Allocator>>,
}

const ROOT_ACCOUNT: AccountID = AccountID::new(1);
const STATE_ACCOUNT: AccountID = AccountID::new(2);

/// Invoke a message packet in the context of the provided state handler.
impl<
        'a,
        CM: VM,
        ST: StateHandler<&'a dyn Allocator>,
        IDG: IDGenerator,
        AUTHZ: AuthorizationMiddleware,
    > HostBackend for ExecContext<'a, CM, ST, IDG, AUTHZ>
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
        self.call_stack.push(Frame { active_account });

        message_packet.header_mut().gas_left = gas.get();

        let res = if target_account == ROOT_ACCOUNT {
            // if the target account is the root account, we can just run the system message
            self.handle_system_message(message_packet, allocator)
        } else {
            // find the account's handler ID
            let handler_id = get_account_handler_id(self.state_handler, target_account, &mut gas, allocator)?
                .ok_or(SystemCode(AccountNotFound))?;

            // run the handler
            self.account_manager.code_manager.run_message(
                &ReadOnlyStoreWrapper::wrap(self.state_handler, &mut gas),
                &handler_id,
                message_packet,
                self,
                allocator,
            )
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

        // pop the call stack
        self.call_stack.pop();

        res
    }

    fn invoke_query(&self, message_packet: &mut MessagePacket, allocator: &dyn Allocator) -> Result<(), ErrorCode> {
        let target_account = message_packet.header().account;
        let mut call_stack = Vec::new_in(allocator);
        call_stack.push(Frame { active_account: target_account });
        // create a nested execution frame for the target account
        let query_ctx = QueryContext {
            account_manager: self.account_manager,
            state_handler: self.state_handler,
            call_stack: RefCell::new(call_stack),
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
        let handler_id = get_account_handler_id(self.state_handler, target_account, &mut gas, allocator)?
            .ok_or(SystemCode(AccountNotFound))?;

        // run the handler
        self.account_manager.code_manager.run_query(
            &ReadOnlyStoreWrapper::wrap(self.state_handler, &mut gas),
            &handler_id,
            message_packet,
            &query_ctx,
            allocator,
        )
    }
}

impl<'a, CM: VM, ST: StateHandler<&'a dyn Allocator>> HostBackend for QueryContext<'a, CM, ST> {
    fn invoke_msg(
        &mut self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
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
        // let target_account = message_packet.header().account;
        // message_packet.header_mut().caller = self.active_account;
        // if target_account == STATE_ACCOUNT {
        //     return self
        //         .query_context
        //         .state_handler
        //         .handler_query(message_packet, allocator);
        // }
        //
        // message_packet.header_mut().caller = AccountID::EMPTY;
        //
        // // find the account's handler ID
        // let handler_id = get_account_handler_id(self.query_context.state_handler, target_account)
        //     .ok_or(SystemCode(AccountNotFound))?;
        //
        // // create a nested execution frame for the target account
        // let frame = QueryFrame {
        //     active_account: target_account,
        //     query_context: self,
        // };

        // run the handler
        // self.query_context.code_manager.run_query(
        //     self.query_context.state_handler,
        //     &handler_id,
        //     message_packet,
        //     &frame,
        //     allocator,
        // )
        todo!()
    }
}

impl<'a, CM: VM, ST: StateHandler<&'a dyn Allocator>, IDG: IDGenerator, AUTHZ: AuthorizationMiddleware>
    ExecContext<'a, CM, ST, IDG, AUTHZ>
{
    fn invoke(
        &mut self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        todo!()
    }

    fn invoke_query(
        &self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        todo!()
    }

    fn handle_system_message(
        &mut self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        unsafe {
            match message_packet.header().message_selector {
                CREATE_SELECTOR => self.handle_create(message_packet, allocator),
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
        // // get the input data
        // let create_header = message_packet.header_mut();
        // let handler_id = create_header.in_pointer1.get(message_packet);
        // let init_data = create_header.in_pointer2.get(message_packet);
        //
        // // resolve the handler ID and retrieve the VM
        // let handler_id = self
        //     .account_manager
        //     .code_manager
        //     .resolve_handler_id(self.state_handler, handler_id)
        //     .ok_or(SystemCode(HandlerNotFound))?;
        //
        // // get the next account ID and initialize the account storage
        // let id = init_next_account(self.id_generator, self.state_handler, &handler_id)
        //     .map_err(|_| SystemCode(InvalidHandler))?;
        //
        // // create a packet for calling on_create
        // let mut on_create_packet =
        //     MessagePacket::allocate(allocator, 0).map_err(|_| SystemCode(FatalExecutionError))?;
        // let on_create_header = on_create_packet.header_mut();
        // on_create_header.account = id;
        // on_create_header.caller = create_header.caller;
        // on_create_header.message_selector = ON_CREATE_SELECTOR;
        // on_create_header.in_pointer1.set_slice(init_data);

        // run the on_create handler
        // let mut frame = Frame::new(id, self);
        // let res = self.code_manager.run_system_message(
        //     self.state_handler,
        //     &handler_id,
        //     &mut on_create_packet,
        //     &mut frame,
        //     allocator,
        // );

        // let is_ok = match res {
        //     Ok(_) => true,
        //     // we accept the case where the handler doesn't have an on_create method
        //     Err(SystemCode(MessageNotHandled)) => true,
        //     _ => false,
        // };
        //
        // if is_ok {
        //     // the result is ID of the newly created account
        //     let mut res = allocator
        //         .allocate(Layout::from_size_align_unchecked(16, 16))
        //         .map_err(|_| SystemCode(FatalExecutionError))?;
        //     let id: u128 = id.into();
        //     res.as_mut().copy_from_slice(&id.to_le_bytes());
        //     create_header.in_pointer1.set_slice(res.as_ref());
        //     Ok(())
        // } else {
        //     res
        // }
        todo!()
    }

    unsafe fn handle_self_destruct(
        &mut self,
        message_packet: &mut MessagePacket,
    ) -> Result<(), ErrorCode> {
        // destroy_account_data(self.state_handler, message_packet.header().caller)
        //     .map_err(|_| SystemCode(FatalExecutionError))
        todo!()
    }
}

// fn invoke_query<
//     'a,
//     CM: CodeManager,
//     ST: StateHandler>
// (
//     code_handler: &'a CM,
//     state_handler: &'a ST,
//     message_packet: &mut MessagePacket,
//     allocator: &dyn Allocator,
// ) -> Result<(), ErrorCode> {
//     let mut exec_context = ExecContext::new(code_handler, state_handler);
//     let mut exec_frame = ExecFrame::new(message_packet.header().account, &mut exec_context);
//     todo!()
// }

const CREATE_SELECTOR: u64 = message_selector!("ixc.account.v1.create");
const ON_CREATE_SELECTOR: u64 = message_selector!("ixc.account.v1.on_create");
const SELF_DESTRUCT_SELECTOR: u64 = message_selector!("ixc.account.v1.self_destruct");

struct ReadOnlyStoreWrapper<'b, 'a, S: StateHandler<&'b dyn Allocator>> {
    state_handler: &'a S,
    gas: &'a mut GasMeter,
    _phantom: core::marker::PhantomData<&'b ()>,
}

impl<'b, 'a, S: StateHandler<&'b dyn Allocator>> ReadOnlyStoreWrapper<'b, 'a, S> {
    fn wrap(state_handler: &'a S, gas: &'a mut GasMeter) -> Self {
        Self { state_handler, gas, _phantom: Default::default() }
    }
}

impl<'b, 'a, S: StateHandler<&'b dyn Allocator>> ReadonlyStore for ReadOnlyStoreWrapper<'b, 'a, S> {
    fn get<'c>(&self, account_id: AccountID, key: &[u8], allocator: &'c dyn Allocator) -> Result<Option<Vec<u8, &'c dyn Allocator>>, ErrorCode> {
        self.state_handler.kv_get(account_id, key, self.gas, allocator)
    }
}