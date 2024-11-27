use ixc_core::low_level::create_packet;
use ixc_core::result::ClientResult;
use ixc_core::Context;
use ixc_core_macros::message_selector;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::header::MessageSelector;
use ixc_message_api::AccountID;

const STATE_ACCOUNT: AccountID = AccountID::new(2);

const GET_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.get");
const SET_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.set");
const DELETE_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.delete");

pub(crate) struct KVStoreClient;

impl KVStoreClient {
    pub(crate) fn get<'a>(&self, ctx: &'a Context, key: &[u8]) -> ClientResult<Option<&'a [u8]>> {
        let mut packet = create_packet(ctx, STATE_ACCOUNT, GET_SELECTOR)?;
        let header = packet.header_mut();
        unsafe {
            header.in_pointer1.set_slice(key);
            if let Err(ErrorCode::HandlerCode(0)) = ctx
                .host_backend()
                .invoke(&mut packet, &ctx.memory_manager())
            {
                return Ok(None);
            }
        }
        let res_bz = unsafe { packet.header().out_pointer1.get(&packet) };
        Ok(Some(res_bz))
    }

    pub(crate) unsafe fn set(&self, ctx: &Context, key: &[u8], value: &[u8]) -> ClientResult<()> {
        let mut packet = create_packet(ctx, STATE_ACCOUNT, SET_SELECTOR)?;
        let header = packet.header_mut();
        unsafe {
            header.in_pointer1.set_slice(key);
            header.in_pointer2.set_slice(value);
            ctx.host_backend()
                .invoke(&mut packet, &ctx.memory_manager())?;
        }
        Ok(())
    }

    pub(crate) unsafe fn delete(&self, ctx: &Context, key: &[u8]) -> ClientResult<()> {
        let mut packet = create_packet(ctx, STATE_ACCOUNT, DELETE_SELECTOR)?;
        let header = packet.header_mut();
        unsafe {
            header.in_pointer1.set_slice(key);
            ctx.host_backend()
                .invoke(&mut packet, &ctx.memory_manager())?;
        }
        Ok(())
    }
}
