use crate::gas::GasMeter;
use crate::state_handler::std::manager::StdStateManager;
use crate::state_handler::StateHandler;
use allocator_api2::alloc::Allocator;
use ixc_core_macros::message_selector;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::code::ErrorCode::SystemCode;
use ixc_message_api::code::SystemCode::MessageNotHandled;
use ixc_message_api::message::{MessageSelector, Request, Response};
use ixc_message_api::AccountID;

/// The standard state handler.
pub struct StdStateHandler<'a, S: StdStateManager> {
    state: &'a mut S,
    _gas_config: GasConfig,
}

/// Gas configuration for the standard state handler.
#[derive(Default)]
pub struct GasConfig {
    /// The cost of deleting a value from storage.
    pub delete_cost: u64,
    /// The flat cost of reading a value from storage.
    pub read_cost_flat: u64,
    /// The cost per byte of reading a value from storage.
    pub read_cost_per_byte: u64,
    /// The flat cost of writing a value to storage.
    pub write_cost_flat: u64,
    /// The cost per byte of writing a value to storage.
    pub write_cost_per_byte: u64,
}

impl<'a, S: StdStateManager> StdStateHandler<'a, S> {
    /// Create a new standard state handler.
    pub fn new(state: &'a mut S, gas_config: GasConfig) -> Self {
        Self {
            state,
            _gas_config: gas_config,
        }
    }
}

impl<S: StdStateManager> StateHandler for StdStateHandler<'_, S> {
    fn kv_get<'a>(
        &self,
        account_id: AccountID,
        key: &[u8],
        _gas: &GasMeter,
        allocator: &'a dyn Allocator,
    ) -> Result<Option<&'a [u8]>, ErrorCode> {
        self.state.kv_get(account_id, None, key, allocator)
    }

    fn kv_set(
        &mut self,
        account_id: AccountID,
        key: &[u8],
        value: &[u8],
        _gas: &GasMeter,
    ) -> Result<(), ErrorCode> {
        self.state.kv_set(account_id, None, key, value)
    }

    fn kv_delete(
        &mut self,
        account_id: AccountID,
        key: &[u8],
        _gas: &GasMeter,
    ) -> Result<(), ErrorCode> {
        self.state.kv_delete(account_id, None, key)
    }

    fn begin_tx(&mut self, _gas: &GasMeter) -> Result<(), ErrorCode> {
        self.state.begin_tx()
    }

    fn commit_tx(&mut self, _gas: &GasMeter) -> Result<(), ErrorCode> {
        self.state.commit_tx()
    }

    fn rollback_tx(&mut self, _gas: &GasMeter) -> Result<(), ErrorCode> {
        self.state.rollback_tx()
    }

    fn handle_exec<'a>(
        &mut self,
        account_id: AccountID,
        request: &Request,
        gas: &GasMeter,
        _allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        match request.message_selector() {
            SET_SELECTOR => {
                let key = request.in1().expect_bytes()?;
                let value = request.in2().expect_bytes()?;
                self.kv_set(account_id, key, value, gas)?;
                Ok(Default::default())
            }
            DELETE_SELECTOR => {
                let key = request.in1().expect_bytes()?;
                self.kv_delete(account_id, key, gas)?;
                Ok(Default::default())
            }
            EMIT_EVENT_SELECTOR => {
                let body = request.in1().expect_bytes()?;
                let type_selector = request.in2().expect_u64()?;
                self.state.emit_event(account_id, type_selector, body)?;
                Ok(Default::default())
            }
            _ => Err(SystemCode(MessageNotHandled)),
        }
    }

    fn handle_query<'a>(
        &self,
        account_id: AccountID,
        request: &Request,
        gas: &GasMeter,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        match request.message_selector() {
            GET_SELECTOR => {
                let key = request.in1().expect_bytes()?;
                let value = self.kv_get(account_id, key, gas, allocator)?;
                match value {
                    Some(value) => Ok(Response::new1(value.into())),
                    _ => Ok(Default::default()),
                }
            }
            _ => Err(SystemCode(MessageNotHandled)),
        }
    }

    fn create_account_storage(
        &mut self,
        account: AccountID,
        _gas: &GasMeter,
    ) -> Result<(), ErrorCode> {
        self.state.create_account_storage(account)
    }

    fn delete_account_storage(
        &mut self,
        account: AccountID,
        _gas: &GasMeter,
    ) -> Result<(), ErrorCode> {
        self.state.delete_account_storage(account)
    }
}

const GET_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.get");
const SET_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.set");
const DELETE_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.delete");
const EMIT_EVENT_SELECTOR: MessageSelector = message_selector!("ixc.events.1.emit");
