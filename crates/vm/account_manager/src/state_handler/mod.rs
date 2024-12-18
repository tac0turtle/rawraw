//! State handler traits.
pub mod std;

use crate::gas::GasMeter;
use crate::id_generator;
use crate::id_generator::IDGenerator;
use alloc::format;
use allocator_api2::alloc::Allocator;
use allocator_api2::vec::Vec;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::message::{Request, Response};
use ixc_message_api::{AccountID, ROOT_ACCOUNT};
use ixc_message_api::code::SystemCode::EncodingError;

/// The state handler trait.
pub trait StateHandler {
    /// Get the value of the key.
    fn kv_get<'a>(
        &self,
        account_id: AccountID,
        key: &[u8],
        gas: &GasMeter,
        allocator: &'a dyn Allocator,
    ) -> Result<Option<&'a [u8]>, ErrorCode>;
    /// Set the value of the key.
    fn kv_set(
        &mut self,
        account_id: AccountID,
        key: &[u8],
        value: &[u8],
        gas: &GasMeter,
    ) -> Result<(), ErrorCode>;
    /// Delete the value of the key.
    fn kv_delete(
        &mut self,
        account_id: AccountID,
        key: &[u8],
        gas: &GasMeter,
    ) -> Result<(), ErrorCode>;
    /// Begin a transaction.
    fn begin_tx(&mut self, gas: &GasMeter) -> Result<(), ErrorCode>;
    /// Commit a transaction.
    fn commit_tx(&mut self, gas: &GasMeter) -> Result<(), ErrorCode>;
    /// Rollback a transaction.
    fn rollback_tx(&mut self, gas: &GasMeter) -> Result<(), ErrorCode>;

    /// Handle a message packet.
    fn handle_exec<'a>(
        &mut self,
        account_id: AccountID,
        request: &Request,
        gas: &GasMeter,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode>;

    /// Handle a query message packet.
    fn handle_query<'a>(
        &self,
        account_id: AccountID,
        request: &Request,
        gas: &GasMeter,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode>;

    /// Create storage for a new account.
    fn create_account_storage(
        &mut self,
        account: AccountID,
        gas: &GasMeter,
    ) -> Result<(), ErrorCode>;

    /// Delete all of an account's storage.
    fn delete_account_storage(
        &mut self,
        account: AccountID,
        gas: &GasMeter,
    ) -> Result<(), ErrorCode>;
}

pub(crate) fn get_account_handler_id<'a, ST: StateHandler>(
    state_handler: &ST,
    account_id: AccountID,
    gas: &GasMeter,
    allocator: &'a dyn Allocator,
) -> Result<Option<&'a str>, ErrorCode> {
    let id: u128 = account_id.into();
    let key = format!("h:{}", id);
    let value = state_handler.kv_get(ROOT_ACCOUNT, key.as_bytes(), gas, allocator)?;
    if let Some(value) = value {
        let handler_id = core::str::from_utf8(value)
            .map_err(|_| ErrorCode::SystemCode(EncodingError))?;
        Ok(Some(handler_id))
    } else {
        Ok(None)
    }
}

pub(crate) fn init_next_account<ST: StateHandler, IDG: IDGenerator>(
    id_generator: &IDG,
    state_handler: &mut ST,
    handler_id: &str,
    allocator: &dyn Allocator,
    gas: &GasMeter,
) -> Result<AccountID, ErrorCode> {
    let id: u128 = id_generator
        .new_account_id(&mut StoreWrapper::wrap(state_handler, gas, allocator))?
        .into();
    let id: AccountID = AccountID::new(id);
    state_handler.create_account_storage(id, gas)?;
    set_handler_id(state_handler, id, handler_id, gas)?;
    Ok(id)
}

pub(crate) fn set_handler_id<ST: StateHandler>(
    state_handler: &mut ST,
    account_id: AccountID,
    new_handler_id: &str,
    gas: &GasMeter,
) -> Result<(), ErrorCode> {
    let id: u128 = account_id.into();
    state_handler.kv_set(
        ROOT_ACCOUNT,
        // TODO choose a different layout for the key
        format!("h:{}", id).as_bytes(),
        new_handler_id.as_bytes(),
        gas,
    )
}

pub(crate) fn destroy_account_data<ST: StateHandler>(
    state_handler: &mut ST,
    account: AccountID,
    gas: &GasMeter,
) -> Result<(), ErrorCode> {
    let id: u128 = account.into();
    let key = format!("h:{}", id);
    state_handler.kv_delete(ROOT_ACCOUNT, key.as_bytes(), gas)?;
    state_handler.delete_account_storage(account, gas)
}

struct StoreWrapper<'a, S: StateHandler> {
    state_handler: &'a mut S,
    gas: &'a GasMeter,
    allocator: &'a dyn Allocator,
}

impl<'a, S: StateHandler> StoreWrapper<'a, S> {
    fn wrap(state_handler: &'a mut S, gas: &'a GasMeter, allocator: &'a dyn Allocator) -> Self {
        Self {
            state_handler,
            gas,
            allocator,
        }
    }
}

impl<S: StateHandler> id_generator::Store for StoreWrapper<'_, S> {
    fn get(&self, key: &[u8]) -> Result<Option<&[u8]>, ErrorCode> {
        self.state_handler
            .kv_get(ROOT_ACCOUNT, key, self.gas, self.allocator)
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), ErrorCode> {
        self.state_handler
            .kv_set(ROOT_ACCOUNT, key, value, self.gas)
    }
}
