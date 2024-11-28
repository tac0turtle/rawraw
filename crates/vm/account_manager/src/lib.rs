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
use std::alloc::Layout;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use crate::PushFrameError::VolatileAccessError;

pub fn invoke<
    'a,
    CM: CodeManager,
    ST: StateHandler,
    IDG: IDGenerator,
    AUTHZ: AuthorizationMiddleware,
>(
    id_generator: &'a mut IDG,
    code_handler: &'a CM,
    state_handler: &'a mut ST,
    authz: &'a AUTHZ,
    message_packet: &mut MessagePacket,
    allocator: &dyn Allocator,
) -> Result<(), ErrorCode> {
    let mut exec_context = ExecContext::new(id_generator, code_handler, state_handler, authz);
    let mut exec_frame = ExecFrame::new(message_packet.header().account, &mut exec_context);
    todo!()
}

struct ExecContext<
    'a,
    CM: CodeManager,
    ST: StateHandler,
    IDG: IDGenerator,
    AUTHZ: AuthorizationMiddleware,
> {
    id_generator: &'a mut IDG,
    code_manager: &'a CM,
    state_handler: &'a mut ST,
    authz: &'a AUTHZ,
}

impl<'a, CM: CodeManager, ST: StateHandler, IDG: IDGenerator, AUTHZ: AuthorizationMiddleware>
    ExecContext<'a, CM, ST, IDG, AUTHZ>
{
    fn new(
        id_generator: &'a mut IDG,
        code_handler: &'a CM,
        state_handler: &'a mut ST,
        authz: &'a AUTHZ,
    ) -> Self {
        Self {
            id_generator,
            code_manager: code_handler,
            state_handler,
            authz,
        }
    }
}

struct ExecFrame<
    'a,
    CM: CodeManager,
    ST: StateHandler,
    IDG: IDGenerator,
    AUTHZ: AuthorizationMiddleware,
> {
    active_account: AccountID,
    exec_context: &'a mut ExecContext<'a, CM, ST, IDG, AUTHZ>,
}

impl<'a, CM: CodeManager, ST: StateHandler, IDG: IDGenerator, AUTHZ: AuthorizationMiddleware>
    ExecFrame<'a, CM, ST, IDG, AUTHZ>
{
    fn new(
        active_account: AccountID,
        exec_context: &'a mut ExecContext<'a, CM, ST, IDG, AUTHZ>,
    ) -> Self {
        Self {
            active_account,
            exec_context,
        }
    }
}

struct QueryContext<'a, CM: CodeManager, ST: StateHandler> {
    code_manager: &'a CM,
    state_handler: &'a ST,
}

struct QueryFrame<'a, CM: CodeManager, ST: StateHandler> {
    active_account: AccountID,
    query_context: &'a QueryContext<'a, CM, ST>,
}

/// An error when committing a transaction.
pub enum CommitError {
    /// Attempted to commit when the call stack was not empty.
    UnfinishedCallStack,
}

/// A push frame error.
#[non_exhaustive]
pub enum PushFrameError {
    /// Tried to push a volatile frame on top of a non-volatile frame.
    VolatileAccessError,
}

/// A pop frame error.
#[non_exhaustive]
pub enum PopFrameError {
    /// No frames to pop.
    NoFrames,
}

const ROOT_ACCOUNT: AccountID = AccountID::new(1);
const STATE_ACCOUNT: AccountID = AccountID::new(2);

// impl<'a, C: CodeManager, ST: StateHandler> HostBackend for ExecContext<'a, C, ST> {
//     fn invoke(
//         &self,
//         message_packet: &mut MessagePacket,
//         allocator: &dyn Allocator,
//     ) -> Result<(), ErrorCode> {
//         let mut state_handler = self.state_handler.borrow_mut();
//         self.invoke::<C, ST>(message_packet, allocator)
//         invoke::<C, ST>(
//             self.active_account,
//             self.code_handler,
//             state_handler.borrow_mut(),
//             message_packet,
//             allocator,
//         )
//     }
// }

/// Invoke a message packet in the context of the provided state handler.
impl<'a, CM: CodeManager, ST: StateHandler, IDG: IDGenerator, AUTHZ: AuthorizationMiddleware>
    ExecFrame<'a, CM, ST, IDG, AUTHZ>
{
    fn invoke(
        &mut self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        let caller = message_packet.header().caller;
        let target_account = message_packet.header().account;

        // check if the caller matches the active account
        if caller != self.active_account {
            if target_account == STATE_ACCOUNT {
                // when calling the state handler, we NEVER allow impersonation
                return Err(SystemCode(UnauthorizedCallerAccess));
            }
            // otherwise we check the authorization middleware to see if impersonation is allowed
            self.exec_context
                .authz
                .authorize(message_packet.header().caller, message_packet)?;
        }

        self.exec_context.invoke(message_packet, allocator)
    }
}

impl<'a, CM: CodeManager, ST: StateHandler, IDG: IDGenerator, AUTHZ: AuthorizationMiddleware>
    HostBackend for ExecFrame<'a, CM, ST, IDG, AUTHZ>
{
    fn invoke(
        &mut self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        self.invoke(message_packet, allocator)
    }

    fn invoke_query(
        &self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        self.exec_context.invoke_query(message_packet, allocator)
    }
}

impl<'a, CM: CodeManager, ST: StateHandler>
    HostBackend for QueryFrame<'a, CM, ST>
{
    fn invoke(
        &mut self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        Err(SystemCode(SystemCode::VolatileAccessError))
    }

    fn invoke_query(
        &self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        let target_account = message_packet.header().account;
        message_packet.header_mut().caller = self.active_account;
        if target_account == STATE_ACCOUNT {
            return self.query_context.state_handler.handler_query(message_packet, allocator);
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
        self.query_context.code_manager
            .run_query(self.query_context.state_handler, &handler_id, message_packet, &frame, allocator)
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
        let target_account = message_packet.header().account;
        if target_account == STATE_ACCOUNT {
            return self.state_handler.handle(message_packet, allocator);
        }

        // begin a transaction
        self.state_handler
            .begin_tx()
            .map_err(|_| SystemCode(InvalidHandler))?;

        let res = if target_account == ROOT_ACCOUNT {
            // if the target account is the root account, we can just run the system message
            self.handle_system_message(message_packet, allocator)
        } else {
            // find the account's handler ID
            let handler_id = get_account_handler_id(self.state_handler, target_account)
                .ok_or(SystemCode(AccountNotFound))?;

            // create a nested execution frame for the target account
            let mut frame = ExecFrame::new(target_account, self);

            // run the handler
            self.code_manager
                .run_message(self.state_handler, &handler_id, message_packet, &frame, allocator)
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

        res
    }

    fn invoke_query(
        &self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        let target_account = message_packet.header().account;
        // create a nested execution frame for the target account
        let ctx = QueryContext {
            code_manager: self.code_manager,
            state_handler: self.state_handler,
        };
        let frame = QueryFrame {
            active_account: target_account,
            query_context: &ctx,
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
        self.code_manager
            .run_query(self.state_handler, &handler_id, message_packet, &frame, allocator)
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
        let mut frame = ExecFrame::new(id, self);
        let res = self.code_manager.run_system_message(
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
