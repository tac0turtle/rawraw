use ixc_core::error::ClientError;
use ixc_core::low_level::create_packet;
use ixc_core::result::ClientResult;
use ixc_core::Context;
use ixc_core_macros::message_selector;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::header::MessageSelector;
use ixc_message_api::message::StateRequest;
use ixc_message_api::AccountID;

const GET_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.get");
const SET_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.set");
const DELETE_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.delete");

pub(crate) struct KVStoreClient;

impl KVStoreClient {
    pub(crate) fn get<'a>(&self, ctx: &'a Context, key: &[u8]) -> ClientResult<Option<&'a [u8]>> {
        unsafe {
            let res = ctx.dynamic_query_state(&StateRequest::new1(GET_SELECTOR, key));
            match res {
                Ok(res) => {
                    let res_bz = res.out1;
                    Ok(Some(res_bz))
                }
                Err(ErrorCode::HandlerCode(0)) => Ok(None),
                Err(code) => Err(ClientError::from(code)),
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
            ctx.dynamic_update_state(&StateRequest::new2(SET_SELECTOR, key, value))?;
        }
        Ok(())
    }

    pub(crate) unsafe fn delete(&self, ctx: &mut Context, key: &[u8]) -> ClientResult<()> {
        unsafe {
            ctx.dynamic_update_state(&StateRequest::new1(DELETE_SELECTOR, key))?;
        }
        Ok(())
    }
}
