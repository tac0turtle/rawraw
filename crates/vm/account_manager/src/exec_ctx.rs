use crate::call_stack::CallStack;
use crate::gas_stack::GasStack;
use crate::id_generator::IDGenerator;
use crate::query_ctx::QueryContext;
use crate::state_handler::{
    destroy_account_data, get_account_handler_id, init_next_account, set_handler_id, StateHandler,
};
use crate::wrapper::ExecContextWrapper;
use crate::{AccountManager, ReadOnlyStoreWrapper};
use allocator_api2::alloc::Allocator;
use core::cell::RefCell;
use ixc_core_macros::message_selector;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::code::ErrorCode::SystemCode;
use ixc_message_api::code::SystemCode::{
    AccountNotFound, FatalExecutionError, HandlerNotFound, InvalidHandler, MessageNotHandled,
};
use ixc_message_api::gas::GasTracker;
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
    state_handler: RefCell<&'a mut ST>,
    id_generator: &'a IDG,
    call_stack: CallStack<CALL_STACK_LIMIT>,
    gas_stack: GasStack<CALL_STACK_LIMIT>,
}

impl<'a, CM: VM, ST: StateHandler, IDG: IDGenerator, const CALL_STACK_LIMIT: usize>
    ExecContext<'a, CM, ST, IDG, CALL_STACK_LIMIT>
{
    pub fn new(
        account_manager: &'a AccountManager<'a, CM, CALL_STACK_LIMIT>,
        state_handler: &'a mut ST,
        id_generator: &'a IDG,
        account: AccountID,
        gas_tracker: Option<&'a GasTracker>,
    ) -> Self {
        Self {
            account_manager,
            state_handler: RefCell::new(state_handler),
            id_generator,
            call_stack: CallStack::new(account),
            gas_stack: GasStack::new(gas_tracker.and_then(|g| g.limit)),
        }
    }
}

/// Invoke a message packet in the context of the provided state handler.
impl<CM: VM, ST: StateHandler, IDG: IDGenerator, const CALL_STACK_LIMIT: usize>
    ExecContext<'_, CM, ST, IDG, CALL_STACK_LIMIT>
{
    pub(crate) fn do_invoke_msg<'a>(
        &self,
        message: &Message,
        invoke_params: &InvokeParams<'a, '_>,
    ) -> Result<Response<'a>, ErrorCode> {
        let gas_scope = self.gas_stack.push(invoke_params.gas_tracker)?;
        let target_account = message.target_account();
        let allocator = invoke_params.allocator;

        // begin a transaction
        self.state_handler
            .borrow_mut()
            .begin_tx(self.gas_stack.meter())
            .map_err(|_| SystemCode(InvalidHandler))?;

        let res = if target_account == ROOT_ACCOUNT {
            // if the target account is the root account, we can just run the system message
            self.handle_system_message(message.request(), allocator)
        } else {
            // push onto the call stack when we're calling a non-system account
            let call_scope = self.call_stack.push(target_account)?;

            // find the account's handler ID
            let handler_id = get_account_handler_id(
                *self.state_handler.borrow(),
                target_account,
                self.gas_stack.meter(),
                allocator,
            )?
            .ok_or(SystemCode(AccountNotFound))?;

            // run the handler
            let handler = self.account_manager.code_manager.resolve_handler(
                &ReadOnlyStoreWrapper::wrap(
                    *self.state_handler.borrow(),
                    self.gas_stack.meter(),
                    allocator,
                ),
                handler_id,
                allocator,
            )?;
            let caller = self.call_stack.caller()?;
            let mut wrapper = ExecContextWrapper::new(self);
            let res = handler.handle_msg(&caller, message, &mut wrapper, allocator);

            // pop the call stack
            call_scope.pop();

            res
        };

        // commit or rollback the transaction
        if res.is_ok() {
            self.state_handler
                .borrow_mut()
                .commit_tx(self.gas_stack.meter())
                .map_err(|_| SystemCode(InvalidHandler))?;
        } else {
            self.state_handler
                .borrow_mut()
                .rollback_tx(self.gas_stack.meter())
                .map_err(|_| SystemCode(InvalidHandler))?;
        }

        gas_scope.pop();
        res
    }

    pub(crate) fn do_invoke_query<'a>(
        &self,
        message: &Message,
        invoke_params: &InvokeParams<'a, '_>,
    ) -> Result<Response<'a>, ErrorCode> {
        // create a nested query execution frame
        let gas_scope = self.gas_stack.push(invoke_params.gas_tracker)?;
        let state_handler = self.state_handler.borrow();
        let query_ctx = QueryContext::new(
            self.account_manager,
            *state_handler,
            &self.call_stack,
            &self.gas_stack,
        );
        let res = query_ctx.invoke_query(message, invoke_params);
        gas_scope.pop();
        res
    }

    pub(crate) fn do_update_state<'a>(
        &self,
        req: &Request,
        invoke_params: &InvokeParams<'a, '_>,
    ) -> Result<Response<'a>, ErrorCode> {
        let gas_scope = self.gas_stack.push(invoke_params.gas_tracker)?;
        let active_account = self.call_stack.active_account()?;
        let res = self.state_handler.borrow_mut().handle_exec(
            active_account,
            req,
            self.gas_stack.meter(),
            invoke_params.allocator,
        );
        gas_scope.pop();
        res
    }

    pub(crate) fn do_query_state<'a>(
        &self,
        req: &Request,
        invoke_params: &InvokeParams<'a, '_>,
    ) -> Result<Response<'a>, ErrorCode> {
        let gas_scope = self.gas_stack.push(invoke_params.gas_tracker)?;
        let active_account = self.call_stack.active_account()?;
        let res = self.state_handler.borrow_mut().handle_query(
            active_account,
            req,
            self.gas_stack.meter(),
            invoke_params.allocator,
        );
        gas_scope.pop();
        res
    }

    pub(crate) fn do_consume_gas(&self, gas: u64) -> Result<(), ErrorCode> {
        self.gas_stack.meter().consume(gas)
    }

    pub(crate) fn do_out_of_gas(&self) -> Result<bool, ErrorCode> {
        Ok(self.gas_stack.meter().out_of_gas())
    }
}

impl<CM: VM, ST: StateHandler, IDG: IDGenerator, const CALL_STACK_LIMIT: usize>
    ExecContext<'_, CM, ST, IDG, CALL_STACK_LIMIT>
{
    fn handle_system_message<'a>(
        &self,
        request: &Request,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        unsafe {
            match request.message_selector() {
                CREATE_SELECTOR => self.handle_create(request, allocator),
                MIGRATE_SELECTOR => self.handle_migrate(request, allocator),
                SELF_DESTRUCT_SELECTOR => {
                    self.handle_self_destruct()?;
                    Ok(Default::default())
                }
                _ => Err(SystemCode(MessageNotHandled)),
            }
        }
    }

    unsafe fn handle_create<'a>(
        &self,
        req: &Request,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        // get the input data
        let handler_id = req.in1().expect_string()?;
        let init_data = req.in2().expect_bytes()?;

        // resolve the handler ID and retrieve the VM
        let handler_id = self
            .account_manager
            .code_manager
            .resolve_handler_id(
                &ReadOnlyStoreWrapper::wrap(
                    *self.state_handler.borrow(),
                    self.gas_stack.meter(),
                    allocator,
                ),
                handler_id,
                allocator,
            )?
            .ok_or(SystemCode(HandlerNotFound))?;

        // get the next account ID and initialize the account storage
        let id = init_next_account(
            self.id_generator,
            *self.state_handler.borrow_mut(),
            handler_id,
            allocator,
            self.gas_stack.meter(),
        )
        .map_err(|_| SystemCode(InvalidHandler))?;

        // create a packet for calling on_create
        let on_create = Message::new(id, Request::new1(ON_CREATE_SELECTOR, init_data.into()));

        // run the on_create handler
        let handler = self.account_manager.code_manager.resolve_handler(
            &ReadOnlyStoreWrapper::wrap(
                *self.state_handler.borrow(),
                self.gas_stack.meter(),
                allocator,
            ),
            handler_id,
            allocator,
        )?;

        // push a frame onto the call stack
        let call_scope = self.call_stack.push(id)?;

        let caller = self.call_stack.caller()?;
        let res = handler.handle_system(
            &caller,
            &on_create,
            &mut ExecContextWrapper::new(self),
            allocator,
        );

        // pop the frame
        call_scope.pop();

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
        &self,
        req: &Request,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        // get the input data
        let active_account = self.call_stack.active_account()?;
        let new_handler_id = req.in1().expect_string()?;

        // get the old handler id
        let old_handler_id = get_account_handler_id(
            *self.state_handler.borrow(),
            active_account,
            self.gas_stack.meter(),
            allocator,
        )?
        .ok_or(SystemCode(AccountNotFound))?;

        // resolve the handler ID and retrieve the VM
        let new_handler_id = self
            .account_manager
            .code_manager
            .resolve_handler_id(
                &ReadOnlyStoreWrapper::wrap(
                    *self.state_handler.borrow(),
                    self.gas_stack.meter(),
                    allocator,
                ),
                new_handler_id,
                allocator,
            )?
            .ok_or(SystemCode(HandlerNotFound))?;

        // update the handler ID
        set_handler_id(
            *self.state_handler.borrow_mut(),
            active_account,
            new_handler_id,
            self.gas_stack.meter(),
        )
        .map_err(|_| SystemCode(InvalidHandler))?;

        // create a packet for calling on_create
        let on_migrate = Message::new(
            active_account,
            Request::new1(ON_MIGRATE_SELECTOR, old_handler_id.into()),
        );

        // retrieve the handler
        let handler = self.account_manager.code_manager.resolve_handler(
            &ReadOnlyStoreWrapper::wrap(
                *self.state_handler.borrow(),
                self.gas_stack.meter(),
                allocator,
            ),
            new_handler_id,
            allocator,
        )?;

        // execute the on-migrate packet with the system message handler
        handler.handle_system(
            &active_account,
            &on_migrate,
            &mut ExecContextWrapper::new(self),
            allocator,
        )
    }

    unsafe fn handle_self_destruct(&self) -> Result<(), ErrorCode> {
        destroy_account_data(
            *self.state_handler.borrow_mut(),
            self.call_stack.active_account()?,
            self.gas_stack.meter(),
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
