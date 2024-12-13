use crate::call_stack::CallStack;
use crate::id_generator::IDGenerator;
use crate::query_ctx::QueryContext;
use crate::state_handler::{destroy_account_data, get_account_handler_id, init_next_account, update_handler_id, StateHandler};
use crate::{AccountManager, ReadOnlyStoreWrapper};
use allocator_api2::alloc::Allocator;
use core::alloc::Layout;
use ixc_core_macros::message_selector;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::code::ErrorCode::SystemCode;
use ixc_message_api::code::SystemCode::{AccountNotFound, FatalExecutionError, HandlerNotFound, InvalidHandler, MessageNotHandled};
use ixc_message_api::packet::MessagePacket;
use ixc_message_api::{AccountID, ROOT_ACCOUNT};
use ixc_message_api::handler::HostBackend;
use ixc_message_api::message::{QueryStateResponse, StateRequest, UpdateStateResponse};
use ixc_vm_api::VM;
use crate::gas::GasMeter;

pub(crate) struct ExecContext<
    'a,
    CM: VM,
    ST: StateHandler,
    IDG: IDGenerator,
    const CALL_STACK_LIMIT: usize,
> {
    account_manager: &'a AccountManager<'a, CM, CALL_STACK_LIMIT>,
    state_handler: &'a mut ST,
    id_generator: &'a mut IDG,
    call_stack: CallStack<CALL_STACK_LIMIT>,
}

impl<'a, CM: VM, ST: StateHandler, IDG: IDGenerator, const CALL_STACK_LIMIT: usize> ExecContext<'a, CM, ST, IDG, CALL_STACK_LIMIT> {
    pub fn new(account_manager: &'a AccountManager<'a, CM, CALL_STACK_LIMIT>, state_handler: &'a mut ST, id_generator: &'a mut IDG, account: AccountID) -> Self {
        Self {
            account_manager,
            state_handler,
            id_generator,
            call_stack: CallStack::new(account),
        }
    }
}

/// Invoke a message packet in the context of the provided state handler.
impl<
    CM: VM,
    ST: StateHandler,
    IDG: IDGenerator,
    const CALL_STACK_LIMIT: usize,
> HostBackend for ExecContext<'_, CM, ST, IDG, CALL_STACK_LIMIT>
{
    fn invoke_msg(
        &mut self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        let caller = message_packet.header().caller;
        let target_account = message_packet.header().account;
        let active_account = self.call_stack.active_account()?;

        let mut gas = GasMeter::new(message_packet.header().gas_left);

        // begin a transaction
        self.state_handler
            .begin_tx(&mut gas)
            .map_err(|_| SystemCode(InvalidHandler))?;
        // push the current caller onto the call stack
        message_packet.header_mut().gas_left = gas.get_left().unwrap();

        let res = if target_account == ROOT_ACCOUNT {
            // if the target account is the root account, we can just run the system message
            self.handle_system_message(message_packet, allocator)
        } else {
            // push onto the call stack when we're calling a non-system account
            self.call_stack.push(target_account)?;

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
            self.call_stack.pop();

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
        let query_ctx = QueryContext::new(self.account_manager, self.state_handler, &self.call_stack);
        //
        // we never pass the caller to query handlers and any value set in the caller field is ignored
        message_packet.header_mut().caller = AccountID::EMPTY;

        let target_account = message_packet.header().account;
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

    fn update_state<'a>(&mut self, req: &StateRequest, allocator: &'a dyn Allocator) -> Result<UpdateStateResponse<'a>, ErrorCode> {
        let active_account = self.call_stack.active_account()?;
        let gas_meter = self.call_stack.gas_meter();
        self.state_handler.handle_exec(active_account, req, gas_meter, allocator)
    }

    fn query_state<'a>(&self, req: &StateRequest, allocator: &'a dyn Allocator) -> Result<QueryStateResponse<'a>, ErrorCode> {
        let active_account = self.call_stack.active_account()?;
        let gas_meter = self.call_stack.gas_meter();
        self.state_handler.handle_query(active_account, req, gas_meter, allocator)
    }

    fn consume_gas(&self, gas: u64) -> Result<(), ErrorCode> {
        self.call_stack.gas_meter().consume_gas(gas)
    }
}

impl<
    CM: VM,
    ST: StateHandler,
    IDG: IDGenerator,
    const CALL_STACK_LIMIT: usize,
> ExecContext<'_, CM, ST, IDG, CALL_STACK_LIMIT>
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
        self.call_stack.push(id)?;

        let res = handler.handle_system(&mut on_create_packet, self, allocator);

        // pop the frame
        self.call_stack.pop();

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

const CREATE_SELECTOR: u64 = message_selector!("ixc.account.v1.create");
const ON_CREATE_SELECTOR: u64 = message_selector!("ixc.account.v1.on_create");
const MIGRATE_SELECTOR: u64 = message_selector!("ixc.account.v1.migrate");
const ON_MIGRATE_SELECTOR: u64 = message_selector!("ixc.account.v1.on_migrate");
const SELF_DESTRUCT_SELECTOR: u64 = message_selector!("ixc.account.v1.self_destruct");

