use crate::exec_ctx::ExecContext;
use crate::id_generator::IDGenerator;
use crate::state_handler::StateHandler;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::handler::{HostBackend, InvokeParams};
use ixc_message_api::message::{Message, Request, Response};
use ixc_vm_api::VM;

pub(crate) struct ExecContextWrapper<
    'b,
    'a: 'b,
    CM: VM,
    ST: StateHandler,
    IDG: IDGenerator,
    const CALL_STACK_LIMIT: usize,
> {
    exec_ctx: &'b ExecContext<'a, CM, ST, IDG, CALL_STACK_LIMIT>,
}

impl<'b, 'a: 'b, CM: VM, ST: StateHandler, IDG: IDGenerator, const CALL_STACK_LIMIT: usize>
    ExecContextWrapper<'b, 'a, CM, ST, IDG, CALL_STACK_LIMIT>
{
    pub(crate) fn new(exec_ctx: &'b ExecContext<'a, CM, ST, IDG, CALL_STACK_LIMIT>) -> Self {
        Self { exec_ctx }
    }
}

impl<CM: VM, ST: StateHandler, IDG: IDGenerator, const CALL_STACK_LIMIT: usize> HostBackend
    for ExecContextWrapper<'_, '_, CM, ST, IDG, CALL_STACK_LIMIT>
{
    fn invoke_msg<'a>(
        &mut self,
        message: &Message,
        invoke_params: &InvokeParams<'a>,
    ) -> Result<Response<'a>, ErrorCode> {
        self.exec_ctx.do_invoke_msg(message, invoke_params)
    }

    fn invoke_query<'a>(
        &self,
        message: &Message,
        invoke_params: &InvokeParams<'a>,
    ) -> Result<Response<'a>, ErrorCode> {
        self.exec_ctx.do_invoke_query(message, invoke_params)
    }

    fn update_state<'a>(
        &mut self,
        req: &Request,
        invoke_params: &InvokeParams<'a>,
    ) -> Result<Response<'a>, ErrorCode> {
        self.exec_ctx.do_update_state(req, invoke_params)
    }

    fn query_state<'a>(
        &self,
        req: &Request,
        invoke_params: &InvokeParams<'a>,
    ) -> Result<Response<'a>, ErrorCode> {
        self.exec_ctx.do_query_state(req, invoke_params)
    }

    fn consume_gas(&self, gas: u64) -> Result<(), ErrorCode> {
        self.exec_ctx.do_consume_gas(gas)
    }
}
