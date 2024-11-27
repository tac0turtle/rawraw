use crate::account_api::ROOT_ACCOUNT;
use crate::error::ClientError;
use crate::handler::Client;
use crate::low_level::create_packet;
use crate::result::ClientResult;
use crate::Context;
use ixc_core_macros::message_selector;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::code::SystemCode::EncodingError;

/// Returns a unique ID in the context of the current message execution.
/// The unique ID is a 128-bit value that is guaranteed to be unique in the context
/// of the application and deterministic across different executions of the same application.
pub fn new_unique_id(ctx: &mut Context) -> ClientResult<u128> {
    let mut packet = create_packet(ctx, ROOT_ACCOUNT, NEW_UNIQUE_ID_SELECTOR)?;
    unsafe {
        ctx.host_backend()
            .invoke(&mut packet, ctx.memory_manager())?;
        let res = packet.header().out_pointer1.get(&packet);
        if res.len() != size_of::<u128>() {
            return Err(ClientError::new(
                ErrorCode::SystemCode(EncodingError),
                "invalid unique ID".into(),
            ));
        }
        Ok(u128::from_le_bytes(res.try_into().unwrap()))
    }
}

const NEW_UNIQUE_ID_SELECTOR: u64 = message_selector!("ixc.id.v1.new_unique_id");
