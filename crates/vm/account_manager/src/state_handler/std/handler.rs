use crate::state_handler::std::manager::StdStateManager;
use crate::state_handler::StateHandler;
use allocator_api2::alloc::Allocator;
use core::alloc::Layout;
use ixc_core_macros::message_selector;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::code::ErrorCode::SystemCode;
use ixc_message_api::code::SystemCode::{FatalExecutionError, MessageNotHandled};
use ixc_message_api::header::MessageSelector;
use ixc_message_api::message::{QueryStateResponse, StateRequest, UpdateStateResponse};
use ixc_message_api::AccountID;
use crate::gas::GasMeter;

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
    fn kv_get<A: Allocator>(
        &self,
        account_id: AccountID,
        key: &[u8],
        _gas: &GasMeter,
        allocator: A,
    ) -> Result<Option<allocator_api2::vec::Vec<u8, A>>, ErrorCode> {
        self.state
            .kv_get(account_id, None, key, allocator)
            .map_err(|_| SystemCode(FatalExecutionError))
    }

    fn kv_set(
        &mut self,
        account_id: AccountID,
        key: &[u8],
        value: &[u8],
        _gas: &GasMeter,
    ) -> Result<(), ErrorCode> {
        self.state
            .kv_set(account_id, None, key, value)
            .map_err(|_| SystemCode(FatalExecutionError))
    }

    fn kv_delete(
        &mut self,
        account_id: AccountID,
        key: &[u8],
        _gas: &GasMeter,
    ) -> Result<(), ErrorCode> {
        self.state
            .kv_delete(account_id, None, key)
            .map_err(|_| SystemCode(FatalExecutionError))
    }

    fn begin_tx(&mut self, _gas: &GasMeter) -> Result<(), ErrorCode> {
        self.state
            .begin_tx()
            .map_err(|_| SystemCode(FatalExecutionError))
    }

    fn commit_tx(&mut self, _gas: &GasMeter) -> Result<(), ErrorCode> {
        self.state
            .commit_tx()
            .map_err(|_| SystemCode(FatalExecutionError))
    }

    fn rollback_tx(&mut self, _gas: &GasMeter) -> Result<(), ErrorCode> {
        self.state
            .rollback_tx()
            .map_err(|_| SystemCode(FatalExecutionError))
    }

    fn handle_exec<'a>(
        &mut self,
        account_id: AccountID,
        request: &StateRequest<'_>,
        gas: &GasMeter,
        _allocator: &dyn Allocator,
    ) -> Result<UpdateStateResponse<'a>, ErrorCode> {
        match request.message_selector {
            SET_SELECTOR => {
                let key = request.in1;
                let value = request.in2;
                self.kv_set(account_id, key, value, gas)?;
                Ok(Default::default())
            }
            DELETE_SELECTOR => {
                let key = request.in1;
                self.kv_delete(account_id, key, gas)?;
                Ok(Default::default())
            }
            _ => Err(SystemCode(MessageNotHandled)),
        }
    }

    fn handle_query<'a>(
        &self,
        account_id: AccountID,
        request: &StateRequest<'_>,
        gas: &GasMeter,
        allocator: &'a dyn Allocator,
    ) -> Result<QueryStateResponse<'a>, ErrorCode> {
        unsafe {
            match request.message_selector {
                GET_SELECTOR => {
                    let key = request.in1;
                    let value = self.kv_get(account_id, key, gas, allocator)?;
                    if let Some(value) = value {
                        let out = allocator
                            .allocate(Layout::from_size_align_unchecked(value.len(), 16))
                            .map_err(|_| SystemCode(FatalExecutionError))?;
                        let out_slice =
                            core::slice::from_raw_parts_mut(out.as_ptr() as *mut u8, value.len());
                        out_slice.copy_from_slice(value.as_slice());
                        Ok(QueryStateResponse::new1(out_slice))
                    } else {
                        // KV-stores should use handler code 0 to indicate not found
                        const NOT_FOUND: ErrorCode = ErrorCode::HandlerCode(0);
                        Err(NOT_FOUND)
                    }
                }
                _ => Err(SystemCode(MessageNotHandled)),
            }
        }
    }

    fn create_account_storage(
        &mut self,
        account: AccountID,
        _gas: &GasMeter,
    ) -> Result<(), ErrorCode> {
        self.state
            .create_account_storage(account)
            .map_err(|_| SystemCode(FatalExecutionError))
    }

    fn delete_account_storage(
        &mut self,
        account: AccountID,
        _gas: &GasMeter,
    ) -> Result<(), ErrorCode> {
        self.state
            .delete_account_storage(account)
            .map_err(|_| SystemCode(FatalExecutionError))
    }
}

const GET_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.get");
const SET_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.set");
const DELETE_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.delete");
