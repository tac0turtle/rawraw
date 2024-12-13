use crate::call_stack::CallStack;
use crate::state_handler::{get_account_handler_id, StateHandler};
use crate::{AccountManager, ReadOnlyStoreWrapper};
use allocator_api2::alloc::Allocator;
use allocator_api2::vec::Vec;
use ixc_core_macros::message_selector;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::code::ErrorCode::SystemCode;
use ixc_message_api::code::SystemCode::{AccountNotFound, EncodingError, MessageNotHandled};
use ixc_message_api::handler::HostBackend;
use ixc_message_api::packet::MessagePacket;
use ixc_message_api::{AccountID, ROOT_ACCOUNT};
use ixc_message_api::message::{QueryStateResponse, StateRequest, UpdateStateResponse};
use ixc_vm_api::VM;
use crate::gas::GasMeter;

pub(crate) struct QueryContext<'b, 'a: 'b, CM: VM, ST: StateHandler, const CALL_STACK_LIMIT: usize> {
    account_manager: &'a AccountManager<'a, CM, CALL_STACK_LIMIT>,
    state_handler: &'a ST,
    call_stack: &'b CallStack<CALL_STACK_LIMIT>,
}

impl<'b, 'a: 'b, CM: VM, ST: StateHandler, const CALL_STACK_LIMIT: usize> QueryContext<'b, 'a, CM, ST, CALL_STACK_LIMIT> {
    pub(crate) fn new(account_manager: &'a AccountManager<'a, CM, CALL_STACK_LIMIT>, state_handler: &'a ST, call_stack: &'b CallStack<CALL_STACK_LIMIT>) -> Self {
        Self { account_manager, state_handler, call_stack }
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
        self.call_stack.push(target_account)?;

        // run the handler
        let handler = self.account_manager.code_manager.resolve_handler(
            &ReadOnlyStoreWrapper::wrap(self.state_handler, &mut gas),
            &handler_id,
            allocator,
        )?;

        let res = handler.handle_query(message_packet, self, allocator);

        // pop the call stack
        self.call_stack.pop();

        res
    }

    fn update_state<'c>(&mut self, _req: &StateRequest, _allocator: &'c dyn Allocator) -> Result<UpdateStateResponse<'c>, ErrorCode> {
        Err(SystemCode(
            ixc_message_api::code::SystemCode::VolatileAccessError,
        ))
    }

    fn query_state<'c>(&self, req: &StateRequest, allocator: &'c dyn Allocator) -> Result<QueryStateResponse<'c>, ErrorCode> {
        let active_account = self.call_stack.active_account()?;
        let gas_meter = self.call_stack.gas_meter();
        self.state_handler.handle_query(active_account, req, gas_meter, allocator)
    }

    fn consume_gas(&self, gas: u64) -> Result<(), ErrorCode> {
        self.call_stack.gas_meter().consume_gas(gas)
    }
}

impl<'b, 'a: 'b, CM: VM, ST: StateHandler, const CALL_STACK_LIMIT: usize>
QueryContext<'b, 'a, CM, ST, CALL_STACK_LIMIT>
{
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
        let handler_id = get_account_handler_id(
            self.state_handler,
            AccountID::from(account_id),
            &mut gas,
            allocator,
        )?
            .ok_or(SystemCode(AccountNotFound))?;

        // copy the handler ID to the out pointer
        let mut vec = Vec::new_in(allocator);
        vec.extend_from_slice(handler_id.as_slice());
        message_packet
            .header_mut()
            .out_pointer1
            .set_slice(vec.as_slice());

        Ok(())
    }
}

const GET_HANDLER_ID_SELECTOR: u64 = message_selector!("ixc.account.v1.get_handler_id");
