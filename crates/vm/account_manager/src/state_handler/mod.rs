//! State handler traits.
pub mod gas;
pub mod std;

use crate::id_generator::IDGenerator;
use crate::state_handler::gas::GasMeter;
use crate::{id_generator, ROOT_ACCOUNT};
use alloc::format;
use allocator_api2::alloc::Allocator;
use allocator_api2::vec::Vec;
use core::cell::RefCell;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::code::ErrorCode::SystemCode;
use ixc_message_api::code::SystemCode::FatalExecutionError;
use ixc_message_api::packet::MessagePacket;
use ixc_message_api::AccountID;

/// The state handler trait.
pub trait StateHandler {
    /// Get the value of the key.
    fn kv_get<A: Allocator>(
        &self,
        account_id: AccountID,
        key: &[u8],
        gas: &mut GasMeter,
        allocator: A,
    ) -> Result<Option<Vec<u8, A>>, ErrorCode>;
    /// Set the value of the key.
    fn kv_set(
        &mut self,
        account_id: AccountID,
        key: &[u8],
        value: &[u8],
        gas: &mut GasMeter,
    ) -> Result<(), ErrorCode>;
    /// Delete the value of the key.
    fn kv_delete(
        &mut self,
        account_id: AccountID,
        key: &[u8],
        gas: &mut GasMeter,
    ) -> Result<(), ErrorCode>;
    /// Begin a transaction.
    fn begin_tx(&mut self, gas: &mut GasMeter) -> Result<(), ErrorCode>;
    /// Commit a transaction.
    fn commit_tx(&mut self, gas: &mut GasMeter) -> Result<(), ErrorCode>;
    /// Rollback a transaction.
    fn rollback_tx(&mut self, gas: &mut GasMeter) -> Result<(), ErrorCode>;

    /// Handle a message packet.
    fn handle_exec(
        &mut self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode>;

    /// Handle a query message packet.
    fn handle_query(
        &self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode>;

    /// Create storage for a new account.
    fn create_account_storage(
        &mut self,
        account: AccountID,
        gas: &mut GasMeter,
    ) -> Result<(), ErrorCode>;

    /// Delete all of an account's storage.
    fn delete_account_storage(
        &mut self,
        account: AccountID,
        gas: &mut GasMeter,
    ) -> Result<(), ErrorCode>;
}
pub(crate) fn get_account_handler_id<'a, ST: StateHandler>(
    state_handler: &ST,
    account_id: AccountID,
    gas: &mut GasMeter,
    allocator: &'a dyn Allocator,
) -> Result<Option<Vec<u8, &'a dyn Allocator>>, ErrorCode> {
    let id: u128 = account_id.into();
    let key = format!("h:{}", id);
    state_handler.kv_get(ROOT_ACCOUNT, key.as_bytes(), gas, allocator)
}

pub(crate) fn init_next_account<ST: StateHandler, IDG: IDGenerator>(
    id_generator: &mut IDG,
    state_handler: &mut ST,
    handler_id: &[u8],
    allocator: &dyn Allocator,
    gas: &mut GasMeter,
) -> Result<AccountID, ErrorCode> {
    let id: u128 = id_generator
        .new_account_id(&mut StoreWrapper::wrap(state_handler, gas, allocator))?
        .into();
    state_handler.create_account_storage(AccountID::new(id), gas)?;
    state_handler.kv_set(
        ROOT_ACCOUNT,
        // TODO choose a different layout for the key
        format!("h:{}", id).as_bytes(),
        handler_id,
        gas,
    )?;
    Ok(AccountID::new(id))
}

pub(crate) fn update_handler_id<ST: StateHandler>(
    state_handler: &mut ST,
    account_id: AccountID,
    new_handler_id: &[u8],
    gas: &mut GasMeter,
) -> Result<(), ErrorCode> {
    let id: u128 = account_id.into();
    state_handler.kv_set(
        ROOT_ACCOUNT,
        format!("h:{}", id).as_bytes(),
        new_handler_id,
        gas,
    )
}

pub(crate) fn destroy_account_data<ST: StateHandler>(
    state_handler: &mut ST,
    account: AccountID,
    gas: &mut GasMeter,
) -> Result<(), ErrorCode> {
    let id: u128 = account.into();
    let key = format!("h:{}", id);
    state_handler.kv_delete(ROOT_ACCOUNT, key.as_bytes(), gas)?;
    state_handler.delete_account_storage(account, gas)
}

struct StoreWrapper<'a, S: StateHandler> {
    state_handler: &'a mut S,
    gas: RefCell<&'a mut GasMeter>,
    allocator: &'a dyn Allocator,
}

impl<'a, S: StateHandler> StoreWrapper<'a, S> {
    fn wrap(state_handler: &'a mut S, gas: &'a mut GasMeter, allocator: &'a dyn Allocator) -> Self {
        Self {
            state_handler,
            gas: RefCell::new(gas),
            allocator,
        }
    }
}

impl<S: StateHandler> id_generator::Store for StoreWrapper<'_, S> {
    fn get(&self, key: &[u8]) -> Result<Option<&[u8]>, ErrorCode> {
        let mut gas = self
            .gas
            .try_borrow_mut()
            .map_err(|_| SystemCode(FatalExecutionError))?;
        if let Some(value) = self
            .state_handler
            .kv_get(ROOT_ACCOUNT, key, *gas, self.allocator)?
        {
            let (ptr, len, _) = value.into_raw_parts();
            Ok(Some(unsafe { core::slice::from_raw_parts(ptr, len) }))
        } else {
            Ok(None)
        }
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), ErrorCode> {
        let mut gas = self
            .gas
            .try_borrow_mut()
            .map_err(|_| SystemCode(FatalExecutionError))?;
        self.state_handler.kv_set(ROOT_ACCOUNT, key, value, *gas)
    }
}
