//! Rust Cosmos SDK RFC 003 hypervisor/state-handler function implementation.

mod authz;
mod code_manager;
mod id_generator;
mod state;
mod state_handler;
mod store;
pub mod vm_manager;

use crate::authz::AuthorizationMiddleware;
use crate::code_manager::CodeManager;
use crate::id_generator::IDGenerator;
use crate::state_handler::{
    destroy_account_data, get_account_handler_id, init_next_account, StateHandler,
};
use allocator_api2::alloc::Global;
use allocator_api2::vec::Vec;
use core::alloc::Layout;
use core::borrow::BorrowMut;
use std::cell::RefCell;
use ixc_core_macros::message_selector;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::code::ErrorCode::SystemCode;
use ixc_message_api::code::SystemCode::{
    AccountNotFound, FatalExecutionError, HandlerNotFound, InvalidHandler, MessageNotHandled,
    UnauthorizedCallerAccess,
};
use ixc_message_api::handler::{Allocator, HostBackend};
use ixc_message_api::packet::MessagePacket;
use ixc_message_api::AccountID;

pub struct AccountManager<'a, CM: CodeManager> {
    code_manager: &'a CM,
    call_stack_limit: usize,
}

impl<'a, CM: CodeManager> AccountManager<'a, CM> {
    pub fn new(code_manager: &'a CM) -> Self {
        Self {
            code_manager,
            call_stack_limit: 128,
        }
    }
}

impl<'a, CM: CodeManager>
    AccountManager<'a, CM>
{
    pub fn invoke<'a, ST: StateHandler, IDG: IDGenerator, AUTHZ: AuthorizationMiddleware>(
        &self,
        id_generator: &'a mut IDG,
        state_handler: &'a mut ST,
        message_packet: &mut MessagePacket,
        authz: &'a AUTHZ,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        let mut exec_context = ExecContext{
            account_manager: self,
            state_handler,
            id_generator,
            authz,
            call_stack: Vec::new_in(allocator),
        };
        exec_context.invoke(message_packet, allocator)
    }
}

struct ExecContext<
    'a,
    CM: CodeManager,
    ST: StateHandler,
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

struct QueryContext<'a, CM: CodeManager, ST: StateHandler> {
    account_manager: &'a AccountManager<'a, CM>,
    state_handler: &'a ST,
    call_stack: RefCell<Vec<Frame, &'a dyn Allocator>>,
}

struct QueryFrame<'a, CM: CodeManager, ST: StateHandler> {
    active_account: AccountID,
    query_context: &'a QueryContext<'a, CM, ST>,
}

const ROOT_ACCOUNT: AccountID = AccountID::new(1);
const STATE_ACCOUNT: AccountID = AccountID::new(2);

/// Invoke a message packet in the context of the provided state handler.
impl<
        'a,
        CM: CodeManager,
        ST: StateHandler,
        IDG: IDGenerator,
        AUTHZ: AuthorizationMiddleware,
        A: Allocator,
    > HostBackend for ExecContext<'a, CM, ST, IDG, AUTHZ, A>
{
    fn invoke(
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
            return self.state_handler.handle(message_packet, allocator);
        }

        // begin a transaction
        self.state_handler
            .begin_tx()
            .map_err(|_| SystemCode(InvalidHandler))?;
        // push the current caller onto the call stack
        self.call_stack.push(Frame { active_account });

        let res = if target_account == ROOT_ACCOUNT {
            // if the target account is the root account, we can just run the system message
            self.handle_system_message(message_packet, allocator)
        } else {
            // find the account's handler ID
            let handler_id = get_account_handler_id(self.state_handler, target_account)
                .ok_or(SystemCode(AccountNotFound))?;

            // run the handler
            self.account_manager.code_manager.run_message(
                self.state_handler,
                &handler_id,
                message_packet,
                &self,
                allocator,
            )
        };

        // commit or rollback the transaction
        if res.is_ok() {
            self.state_handler
                .commit_tx()
                .map_err(|_| SystemCode(InvalidHandler))?;
        } else {
            self.state_handler
                .rollback_tx()
                .map_err(|_| SystemCode(InvalidHandler))?;
        }

        // pop the call stack
        self.call_stack.pop();

        res
    }

    fn invoke_query(&self, message_packet: &mut MessagePacket, allocator: &dyn Allocator) -> Result<(), ErrorCode> {
        let target_account = message_packet.header().account;
        // create a nested execution frame for the target account
        let query_ctx = QueryContext {
            account_manager: self.account_manager,
            state_handler: self.state_handler,
            call_stack: todo!(),
        };

        // we never pass the caller to query handlers and any value set in the caller field is ignored
        message_packet.header_mut().caller = AccountID::EMPTY;

        let target_account = message_packet.header().account;
        if target_account == STATE_ACCOUNT {
            return self.state_handler.handle_query(message_packet, allocator);
        }

        // find the account's handler ID
        let handler_id = get_account_handler_id(self.state_handler, target_account)
            .ok_or(SystemCode(AccountNotFound))?;

        // run the handler
        self.account_manager.code_manager.run_query(
            self.state_handler,
            &handler_id,
            message_packet,
            &query_ctx,
            allocator,
        )
    }
}

impl<'a, CM: CodeManager, ST: StateHandler> HostBackend for QueryFrame<'a, CM, ST> {
    fn invoke(
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
        let target_account = message_packet.header().account;
        message_packet.header_mut().caller = self.active_account;
        if target_account == STATE_ACCOUNT {
            return self
                .query_context
                .state_handler
                .handler_query(message_packet, allocator);
        }

        message_packet.header_mut().caller = AccountID::EMPTY;

        // find the account's handler ID
        let handler_id = get_account_handler_id(self.query_context.state_handler, target_account)
            .ok_or(SystemCode(AccountNotFound))?;

        // create a nested execution frame for the target account
        let frame = QueryFrame {
            active_account: target_account,
            query_context: self,
        };

        // run the handler
        self.query_context.code_manager.run_query(
            self.query_context.state_handler,
            &handler_id,
            message_packet,
            &frame,
            allocator,
        )
    }
}

impl<'a, CM: CodeManager, ST: StateHandler, IDG: IDGenerator, AUTHZ: AuthorizationMiddleware>
    ExecContext<'a, CM, ST, IDG, AUTHZ>
{
    fn invoke(
        &mut self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
    }

    fn invoke_query(
        &self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
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
        // get the input data
        let create_header = message_packet.header_mut();
        let handler_id = create_header.in_pointer1.get(message_packet);
        let init_data = create_header.in_pointer2.get(message_packet);

        // resolve the handler ID and retrieve the VM
        let handler_id = self
            .code_manager
            .resolve_handler_id(handler_id)
            .ok_or(SystemCode(HandlerNotFound))?;

        // get the next account ID and initialize the account storage
        let id = init_next_account(self.id_generator, self.state_handler, &handler_id)
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
        let mut frame = Frame::new(id, self);
        let res = self.code_manager.run_system_message(
            self.state_handler,
            &handler_id,
            &mut on_create_packet,
            &mut frame,
            allocator,
        );

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

    unsafe fn handle_self_destruct(
        &mut self,
        message_packet: &mut MessagePacket,
    ) -> Result<(), ErrorCode> {
        destroy_account_data(self.state_handler, message_packet.header().caller)
            .map_err(|_| SystemCode(FatalExecutionError))
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
