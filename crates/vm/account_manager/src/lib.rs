//! Rust Cosmos SDK RFC 003 hypervisor/state-handler function implementation.

mod state;
pub mod vm_manager;

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

/// A code manager is responsible for resolving handler IDs to code.
pub trait CodeManager {
    /// Resolves a handler ID provided by a caller to the handler ID which should be stored in state
    /// or return None if the handler ID is not valid.
    /// This allows for multiple ways of addressing a single handler in code and for ensuring that
    /// the handler actually exists.
    fn resolve_handler_id(&self, handler_id: &[u8]) -> Option<Vec<u8>>;
    /// Runs a handler with the provided message packet and host backend.
    fn run_handler(
        &self,
        handler_id: &[u8],
        message_packet: &mut MessagePacket,
        backend: &dyn HostBackend,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode>;
}

/// An error when creating a new transaction.
pub struct NewTxError;

/// An error when committing a transaction.
pub enum CommitError {
    /// Attempted to commit when the call stack was not empty.
    UnfinishedCallStack,
}

/// A transaction.
pub trait StateHandler {
    /// Initialize the account storage and push a new frame for the newly initialized storage.
    fn init_account_storage(&mut self, account: AccountID) -> Result<(), PushFrameError>;
    /// Push a new execution frame.
    fn push_frame(&mut self, account: AccountID, volatile: bool) -> Result<(), PushFrameError>;
    /// Pop the current execution frame.
    fn pop_frame(&mut self, commit: bool) -> Result<(), PopFrameError>;
    /// Get the active account.
    fn active_account(&self) -> AccountID;
    /// Removes the data for the active account.
    fn self_destruct_account(&mut self) -> Result<(), ()>;
    /// Directly read a key from the account's KV store.
    fn raw_kv_get(&self, account_id: AccountID, key: &[u8]) -> Option<Vec<u8>>;
    /// Directly write a key to the account's raw KV store.
    fn raw_kv_set(&self, account_id: AccountID, key: &[u8], value: &[u8]);
    /// Directly delete a key from the account's raw KV store.
    fn raw_kv_delete(&self, account_id: AccountID, key: &[u8]);
    /// Handle a message packet.
    fn handle(
        &self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode>;
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

struct ExecContext<'a, C: CodeManager, ST: StateHandler> {
    code_handler: &'a C,
    state_handler: RefCell<&'a mut ST>,
}

fn get_account_handler_id<ST: StateHandler>(
    state_handler: &ST,
    account_id: AccountID,
) -> Option<Vec<u8>> {
    let id: u128 = account_id.into();
    let key = format!("h:{}", id);
    state_handler.raw_kv_get(HYPERVISOR_ACCOUNT, key.as_bytes())
}

fn init_next_account<ST: StateHandler>(
    state_handler: &mut ST,
    handler_id: &[u8],
) -> Result<AccountID, PushFrameError> {
    let id = state_handler
        .raw_kv_get(HYPERVISOR_ACCOUNT, b"next_account_id")
        .map_or(ACCOUNT_ID_NON_RESERVED_START, |v| {
            u128::from_le_bytes(v.try_into().unwrap())
        });
    // we push a new storage frame here because if initialization fails all of this gets rolled back
    state_handler.init_account_storage(AccountID::new(id))?;
    state_handler.raw_kv_set(
        HYPERVISOR_ACCOUNT,
        b"next_account_id",
        &(id + 1).to_le_bytes(),
    );
    state_handler.raw_kv_set(
        HYPERVISOR_ACCOUNT,
        format!("h:{}", id).as_bytes(),
        handler_id,
    );
    Ok(AccountID::new(id))
}

fn destroy_current_account_data<ST: StateHandler>(state_handler: &mut ST) -> Result<(), ()> {
    let current_account = state_handler.active_account();
    let id: u128 = current_account.into();
    let key = format!("h:{}", id);
    state_handler.raw_kv_delete(HYPERVISOR_ACCOUNT, key.as_bytes());
    state_handler.self_destruct_account()
}

const ACCOUNT_ID_NON_RESERVED_START: u128 = u16::MAX as u128 + 1;
const HYPERVISOR_ACCOUNT: AccountID = AccountID::new(1);
const STATE_ACCOUNT: AccountID = AccountID::new(2);

impl<'a, C: CodeManager, ST: StateHandler> HostBackend for ExecContext<'a, C, ST> {
    fn invoke(
        &self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        let mut state_handler = self.state_handler.borrow_mut();
        invoke::<C, ST>(
            self.code_handler,
            state_handler.borrow_mut(),
            message_packet,
            allocator,
        )
    }
}

/// Invoke a message packet in the context of the provided state handler.
pub fn invoke<C: CodeManager, ST: StateHandler>(
    code_handler: &C,
    state_handler: &mut ST,
    message_packet: &mut MessagePacket,
    allocator: &dyn Allocator,
) -> Result<(), ErrorCode> {
    // check if the caller matches the active account
    let account = state_handler.active_account();
    if message_packet.header().caller != account {
        return Err(SystemCode(UnauthorizedCallerAccess));
    }
    // TODO support authorization middleware

    let target_account = message_packet.header().account;
    // check if the target account is a system account
    match target_account {
        HYPERVISOR_ACCOUNT => {
            return handle_system_message(code_handler, state_handler, message_packet, allocator)
        }
        STATE_ACCOUNT => return state_handler.handle(message_packet, allocator),
        _ => {}
    }

    // find the account's handler ID
    let handler_id =
        get_account_handler_id(state_handler, target_account).ok_or(SystemCode(AccountNotFound))?;

    // push an execution frame for the target account
    state_handler.push_frame(target_account, false). // TODO add volatility support
            map_err(|_| SystemCode(InvalidHandler))?;
    // run the handler
    let res = code_handler.run_handler(
        &handler_id,
        message_packet,
        &wrap_backend(code_handler, state_handler),
        allocator,
    );
    // pop the execution frame
    state_handler
        .pop_frame(res.is_ok())
        .map_err(|_| SystemCode(InvalidHandler))?;

    res
}

fn handle_system_message<C: CodeManager, ST: StateHandler>(
    code_handler: &C,
    state_handler: &mut ST,
    message_packet: &mut MessagePacket,
    allocator: &dyn Allocator,
) -> Result<(), ErrorCode> {
    match message_packet.header().message_selector {
        CREATE_SELECTOR => unsafe {
            // get the input data
            let create_header = message_packet.header_mut();
            let handler_id = create_header.in_pointer1.get(message_packet);
            let init_data = create_header.in_pointer2.get(message_packet);

            // resolve the handler ID and retrieve the VM
            let handler_id = code_handler
                .resolve_handler_id(handler_id)
                .ok_or(SystemCode(HandlerNotFound))?;

            // get the next account ID and initialize the account storage
            let id = init_next_account(state_handler, &handler_id)
                .map_err(|_| SystemCode(InvalidHandler))?;

            // create a packet for calling on_create
            let mut on_create_packet = MessagePacket::allocate(allocator, 0)
                .map_err(|_| SystemCode(FatalExecutionError))?;
            let on_create_header = on_create_packet.header_mut();
            // TODO: how do we specify a selector that can only be called by the system?
            on_create_header.account = id;
            on_create_header.caller = create_header.caller;
            on_create_header.message_selector = ON_CREATE_SELECTOR;
            on_create_header.in_pointer1.set_slice(init_data);

            let res = code_handler.run_handler(
                &handler_id,
                &mut on_create_packet,
                &wrap_backend(code_handler, state_handler),
                allocator,
            );
            let is_ok = match res {
                Ok(_) => true,
                Err(SystemCode(MessageNotHandled)) => true,
                _ => false,
            };
            state_handler
                .pop_frame(is_ok)
                .map_err(|_| SystemCode(FatalExecutionError))?;

            if is_ok {
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
        },
        SELF_DESTRUCT_SELECTOR => {
            destroy_current_account_data(state_handler).map_err(|_| SystemCode(FatalExecutionError))
        }
        _ => Err(SystemCode(MessageNotHandled)),
    }
}

fn wrap_backend<'a, C: CodeManager, ST: StateHandler>(
    code_handler: &'a C,
    state_handler: &'a mut ST,
) -> ExecContext<'a, C, ST> {
    ExecContext {
        code_handler,
        state_handler: RefCell::new(state_handler),
    }
}

const CREATE_SELECTOR: u64 = message_selector!("ixc.account.v1.create");
const ON_CREATE_SELECTOR: u64 = message_selector!("ixc.account.v1.on_create");
const SELF_DESTRUCT_SELECTOR: u64 = message_selector!("ixc.account.v1.self_destruct");
