use ixc_core::error::ClientError;
use ixc_core::result::ClientResult;
use ixc_core::Context;
use ixc_core_macros::message_selector;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::handler::InvokeParams;
use ixc_message_api::message::{MessageSelector, Param, Request, Response};

const GET_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.get");
const SET_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.set");
const DELETE_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.delete");

pub(crate) struct KVStoreClient;

impl KVStoreClient {
    pub(crate) fn get<'a>(&self, ctx: &'a Context, key: &[u8]) -> ClientResult<Option<&'a [u8]>> {
        unsafe {
            let res = dynamic_query_state(ctx, &Request::new1(GET_SELECTOR, key.into()))?;
            match res.out1() {
                Param::Slice(res_bz) => Ok(Some(res_bz)),
                _ => Ok(None),
            }
        }
    }

    pub(crate) unsafe fn set(
        &self,
        ctx: &mut Context,
        key: &[u8],
        value: &[u8],
    ) -> ClientResult<()> {
        unsafe {
            dynamic_update_state(ctx, &Request::new2(SET_SELECTOR, key.into(), value.into()))?;
        }
        Ok(())
    }

    pub(crate) unsafe fn delete(&self, ctx: &mut Context, key: &[u8]) -> ClientResult<()> {
        unsafe {
            dynamic_update_state(ctx, &Request::new1(DELETE_SELECTOR, key.into()))?;
        }
        Ok(())
    }
}

fn dynamic_update_state<'a>(
    ctx: &mut Context<'a>,
    req: &Request,
) -> Result<Response<'a>, ErrorCode> {
    let invoke_params = InvokeParams::new(ctx.memory_manager());
    let res = ctx.with_backend_mut(|backend| backend.update_state(req, &invoke_params))?;
    res
}

fn dynamic_query_state<'a>(
    ctx: &Context<'a>,
    req: &Request,
) -> Result<Response<'a>, ErrorCode> {
    let invoke_params = InvokeParams::new(ctx.memory_manager());
    ctx.with_backend(|backend| backend.query_state(req, &invoke_params))
}