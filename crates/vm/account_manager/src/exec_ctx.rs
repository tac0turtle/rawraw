use crate::call_stack::CallStack;
use crate::id_generator::IDGenerator;
use crate::query_ctx::QueryContext;
use crate::state_handler::{
    destroy_account_data, get_account_handler_id, init_next_account, set_handler_id,
    StateHandler,
};
use crate::{AccountManager, ReadOnlyStoreWrapper};
use allocator_api2::alloc::Allocator;
use ixc_core_macros::message_selector;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::code::ErrorCode::SystemCode;
use ixc_message_api::code::SystemCode::{
    AccountNotFound, FatalExecutionError, HandlerNotFound, InvalidHandler, MessageNotHandled,
};
use ixc_message_api::handler::{HostBackend, InvokeParams};
use ixc_message_api::message::{Message, Request, Response};
use ixc_message_api::{AccountID, ROOT_ACCOUNT};
use ixc_vm_api::VM;

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

impl<'a, CM: VM, ST: StateHandler, IDG: IDGenerator, const CALL_STACK_LIMIT: usize>
    ExecContext<'a, CM, ST, IDG, CALL_STACK_LIMIT>
{
    pub fn new(
        account_manager: &'a AccountManager<'a, CM, CALL_STACK_LIMIT>,
        state_handler: &'a mut ST,
        id_generator: &'a mut IDG,
        account: AccountID,
    ) -> Self {
        Self {
            account_manager,
            state_handler,
            id_generator,
            call_stack: CallStack::new(account),
        }
    }
}

/// Invoke a message packet in the context of the provided state handler.
impl<CM: VM, ST: StateHandler, IDG: IDGenerator, const CALL_STACK_LIMIT: usize> HostBackend
    for ExecContext<'_, CM, ST, IDG, CALL_STACK_LIMIT>
{
    fn invoke_msg<'a>(
        &mut self,
        message: &Message,
        invoke_params: &InvokeParams<'a>,
    ) -> Result<Response<'a>, ErrorCode> {
        let target_account = message.target_account;
        let allocator = invoke_params.allocator;

        // begin a transaction
        self.state_handler
            .begin_tx(&self.call_stack.gas)
            .map_err(|_| SystemCode(InvalidHandler))?;

        let res = if target_account == ROOT_ACCOUNT {
            // if the target account is the root account, we can just run the system message
            self.handle_system_message(&message.request, allocator)
        } else {
            // push onto the call stack when we're calling a non-system account
            self.call_stack.push(target_account)?;

            // find the account's handler ID
            let handler_id =
                get_account_handler_id(self.state_handler, target_account, &self.call_stack.gas, allocator)?
                    .ok_or(SystemCode(AccountNotFound))?;

            // run the handler
            let handler = self.account_manager.code_manager.resolve_handler(
                &ReadOnlyStoreWrapper::wrap(self.state_handler, &self.call_stack.gas, allocator),
                &handler_id,
                allocator,
            )?;
            let res = handler.handle_msg(&message.request, self, allocator);

            // pop the call stack
            self.call_stack.pop();

            res
        };

        // commit or rollback the transaction
        if res.is_ok() {
            self.state_handler
                .commit_tx(&self.call_stack.gas)
                .map_err(|_| SystemCode(InvalidHandler))?;
        } else {
            self.state_handler
                .rollback_tx(&self.call_stack.gas)
                .map_err(|_| SystemCode(InvalidHandler))?;
        }

        res
    }

    fn invoke_query<'a>(
        &self,
        message: &Message,
        invoke_params: &InvokeParams<'a>,
    ) -> Result<Response<'a>, ErrorCode> {
        // create a nested query execution frame
        let query_ctx =
            QueryContext::new(self.account_manager, self.state_handler, &self.call_stack);
        query_ctx.invoke_query(message, invoke_params)
    }

    fn update_state<'a>(
        &mut self,
        req: &Request,
        invoke_params: &InvokeParams<'a>,
    ) -> Result<Response<'a>, ErrorCode> {
        let active_account = self.call_stack.active_account()?;
        let gas_meter = &self.call_stack.gas;
        self.state_handler
            .handle_exec(active_account, req, gas_meter, invoke_params.allocator)
    }

    fn query_state<'a>(
        &self,
        req: &Request,
        invoke_params: &InvokeParams<'a>,
    ) -> Result<Response<'a>, ErrorCode> {
        let active_account = self.call_stack.active_account()?;
        let gas_meter = &self.call_stack.gas;
        self.state_handler
            .handle_query(active_account, req, gas_meter, invoke_params.allocator)
    }

    fn consume_gas(&self, gas: u64) -> Result<(), ErrorCode> {
        self.call_stack.gas.consume_gas(gas)
    }

    fn self_account_id(&self) -> AccountID {
        self.call_stack.active_account().unwrap()
    }

    fn caller(&self) -> AccountID {
        self.call_stack.caller().unwrap()
    }
}

impl<CM: VM, ST: StateHandler, IDG: IDGenerator, const CALL_STACK_LIMIT: usize>
    ExecContext<'_, CM, ST, IDG, CALL_STACK_LIMIT>
{
    fn handle_system_message<'a>(
        &mut self,
        request: &Request,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        unsafe {
            match request.message_selector {
                CREATE_SELECTOR => self.handle_create(request, allocator),
                MIGRATE_SELECTOR => self.handle_migrate(request, allocator),
                SELF_DESTRUCT_SELECTOR => {
                    self.handle_self_destruct()?;
                    Ok(Default::default())
                },
                _ => Err(SystemCode(MessageNotHandled)),
            }
        }
    }

    unsafe fn handle_create<'a>(
        &mut self,
        req: &Request,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        // get the input data
        let handler_id = req.in0().expect_string()?;
        let init_data = req.in1().expect_slice()?;

        let gas =  &self.call_stack.gas;

        // resolve the handler ID and retrieve the VM
        let handler_id = self
            .account_manager
            .code_manager
            .resolve_handler_id(
                &ReadOnlyStoreWrapper::wrap(self.state_handler, gas, allocator),
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
            gas,
        )
        .map_err(|_| SystemCode(InvalidHandler))?;

        // create a packet for calling on_create
        let on_create = Request::new1(ON_CREATE_SELECTOR, init_data.into());

        // run the on_create handler
        let handler = self.account_manager.code_manager.resolve_handler(
            &ReadOnlyStoreWrapper::wrap(self.state_handler, gas, allocator),
            &handler_id,
            allocator,
        )?;

        // push a frame onto the call stack
        self.call_stack.push(id)?;

        let res = handler.handle_system(&on_create, self, allocator);

        // pop the frame
        self.call_stack.pop();

        let is_ok = match res {
            Ok(_) => true,
            // we accept the case where the handler doesn't have an on_create method
            Err(SystemCode(MessageNotHandled)) => true,
            Err(_) => false,
        };

        if is_ok {
            // the result is ID of the newly created account, which is the first input
            Ok(Response::new1(id.into()))
        } else {
            res
        }
    }

    unsafe fn handle_migrate<'a>(
        &mut self,
        req: &Request,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        // get the input data
        let active_account = self.call_stack.active_account()?;
        let new_handler_id = req.in0().expect_string()?;

        let gas = &self.call_stack.gas;

        // get the old handler id
        let old_handler_id =
            get_account_handler_id(self.state_handler, active_account, gas, allocator)?
                .ok_or(SystemCode(AccountNotFound))?;

        // resolve the handler ID and retrieve the VM
        let new_handler_id = self
            .account_manager
            .code_manager
            .resolve_handler_id(
                &ReadOnlyStoreWrapper::wrap(self.state_handler, gas, allocator),
                new_handler_id,
                allocator,
            )?
            .ok_or(SystemCode(HandlerNotFound))?;

        // update the handler ID
        set_handler_id(self.state_handler, active_account, &new_handler_id, gas)
            .map_err(|_| SystemCode(InvalidHandler))?;

        // create a packet for calling on_create
        let on_migrate = Request::new1(ON_MIGRATE_SELECTOR, old_handler_id.into());

        // retrieve the handler
        let handler = self.account_manager.code_manager.resolve_handler(
            &ReadOnlyStoreWrapper::wrap(self.state_handler, gas, allocator),
            &new_handler_id,
            allocator,
        )?;

        // execute the on-migrate packet with the system message handler
        handler.handle_system(&on_migrate, self, allocator)
    }

    unsafe fn handle_self_destruct(&mut self) -> Result<(), ErrorCode> {
        destroy_account_data(
            self.state_handler,
            self.call_stack.active_account()?,
            &self.call_stack.gas,
        )
        .map_err(|_| SystemCode(FatalExecutionError))?;
        Ok(())
    }
}

const CREATE_SELECTOR: u64 = message_selector!("ixc.account.v1.create");
const ON_CREATE_SELECTOR: u64 = message_selector!("ixc.account.v1.on_create");
const MIGRATE_SELECTOR: u64 = message_selector!("ixc.account.v1.migrate");
const ON_MIGRATE_SELECTOR: u64 = message_selector!("ixc.account.v1.on_migrate");
const SELF_DESTRUCT_SELECTOR: u64 = message_selector!("ixc.account.v1.self_destruct");
