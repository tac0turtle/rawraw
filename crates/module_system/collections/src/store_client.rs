use ixc_core::error::ClientError;
use ixc_core::result::ClientResult;
use ixc_core::Context;
use ixc_core_macros::message_selector;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::message::{MessageSelector, Param, Request};

const GET_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.get");
const SET_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.set");
const DELETE_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.delete");

pub(crate) struct KVStoreClient;

impl KVStoreClient {
    pub(crate) fn get<'a>(&self, ctx: &'a Context, key: &[u8]) -> ClientResult<Option<&'a [u8]>> {
        unsafe {
            let res = ctx.dynamic_query_state(&Request::new1(GET_SELECTOR, key.into()))?;
            match res.outputs[0] {
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
            ctx.dynamic_update_state(&Request::new2(SET_SELECTOR, key.into(), value.into()))?;
        }
        Ok(())
    }

    pub(crate) unsafe fn delete(&self, ctx: &mut Context, key: &[u8]) -> ClientResult<()> {
        unsafe {
            ctx.dynamic_update_state(&Request::new1(DELETE_SELECTOR, key.into()))?;
        }
        Ok(())
    }
}
