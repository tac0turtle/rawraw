//! Low-level utilities for working with message structs and message packets directly.

use crate::error::{ClientError, HandlerError};
use crate::message::{Message, MessageBase, QueryMessage};
use crate::result::ClientResult;
use crate::Context;
use allocator_api2::alloc::Allocator;
use ixc_message_api::code::{ErrorCode, HandlerCode, StdCode, SystemCode};
use ixc_message_api::handler::InvokeParams;
use ixc_message_api::message::{Request, Response};
use ixc_message_api::AccountID;
use ixc_schema::codec::Codec;
use ixc_schema::mem::MemoryManager;
use ixc_schema::value::OptionalValue;

/// Dynamically invokes an account message.
/// Static account client instances should be preferred wherever possible,
/// so that static dependency analysis can be performed.
pub fn dynamic_invoke_msg<'a, 'b, M: Message<'b>>(
    context: &'a mut Context,
    account: AccountID,
    message: M,
) -> ClientResult<<M::Response<'a> as OptionalValue<'a>>::Value, M::Error> {
    let packet = encode_message_packet(context.memory_manager(), account, message)?;
    let res = dynamic_invoke_msg_packet(context, &packet);
    decode_message_response::<M>(context, &res)
}

/// Dynamically invokes an account query message.
/// Static account client instances should be preferred wherever possible,
/// so that static dependency analysis can be performed.
pub fn dynamic_invoke_query<'a, 'b, M: QueryMessage<'b>>(
    context: &'a Context,
    account: AccountID,
    message: M,
) -> ClientResult<<M::Response<'a> as OptionalValue<'a>>::Value, M::Error> {
    let packet = encode_message_packet(context.memory_manager(), account, message)?;
    let res = dynamic_invoke_query_packet(context, &packet);
    decode_message_response::<M>(context, &res)
}

/// Dynamically invoke a raw query message packet.
pub fn dynamic_invoke_query_packet<'a>(
    ctx: &Context<'a>,
    msg: &ixc_message_api::message::Message,
) -> Result<Response<'a>, ErrorCode> {
    let invoke_params = InvokeParams::new(ctx.mem, &None);
    ctx.with_backend(|backend| backend.invoke_query(msg, &invoke_params))
}

/// Dynamically invoke a raw message packet.
pub fn dynamic_invoke_msg_packet<'a>(
    ctx: &mut Context<'a>,
    msg: &ixc_message_api::message::Message,
) -> Result<Response<'a>, ErrorCode> {
    let invoke_params = InvokeParams::new(ctx.mem, &None);
    ctx.with_backend_mut(|backend| backend.invoke_msg(msg, &invoke_params))?
}

fn encode_message_packet<'a, 'b, M: MessageBase<'b>>(
    mem: &'a MemoryManager,
    account: AccountID,
    message: M,
) -> ClientResult<ixc_message_api::message::Message<'a>, M::Error> {
    // encode the message body
    let cdc = M::Codec::default();
    let msg_body = cdc.encode_value(&message, mem)?;

    // create the message packet and fill in call details
    Ok(ixc_message_api::message::Message::new(
        account,
        Request::new1(M::SELECTOR, msg_body.into()),
    ))
}

fn decode_message_response<'a, 'b, M: MessageBase<'b>>(
    context: &'a Context,
    res: &Result<Response<'a>, ErrorCode>,
) -> ClientResult<<M::Response<'a> as OptionalValue<'a>>::Value, M::Error> {
    match res {
        Ok(res) => {
            let cdc = M::Codec::default();
            let res = M::Response::<'a>::decode_value(&cdc, res.out1(), context.memory_manager())?;
            Ok(res)
        }
        Err(e) => {
            let c: u16 = (*e).into();
            let code = ErrorCode::<M::Error>::from(c);
            Err(ClientError { code })
        }
    }
}

/// Encodes the response to the out1 pointer of the message packet. Used for encoding the response of a message in macros.
pub fn encode_response<'a, 'b, M: MessageBase<'a>>(
    cdc: &dyn Codec,
    res: crate::Result<
        <<M as MessageBase<'a>>::Response<'a> as OptionalValue<'a>>::Value,
        M::Error,
    >,
    allocator: &'b dyn Allocator,
) -> Result<Response<'b>, ErrorCode> {
    match res {
        Ok(value) => {
            if let Some(out1) =
                <<M as MessageBase<'a>>::Response<'a> as OptionalValue<'a>>::encode_value(
                    cdc, &value, allocator,
                )?
            {
                Ok(Response::new1(out1.into()))
            } else {
                Ok(Response::default())
            }
        }
        Err(e) => encode_handler_error(e),
    }
}

/// Encodes a default response to the out1 pointer of the message packet.
/// Used for encoding the response of a message in macros.
pub fn encode_default_response<'b>(res: crate::Result<()>) -> Result<Response<'b>, ErrorCode> {
    match res {
        Ok(_) => Ok(Default::default()),
        Err(e) => encode_handler_error(e),
    }
}

/// Encode a handler error to the out1 pointer of the message packet.
/// Used for encoding the response of a message in macros.
pub fn encode_handler_error<'b, E: HandlerCode>(
    err: HandlerError<E>,
) -> Result<Response<'b>, ErrorCode> {
    Err(match err.code {
        None => StdCode::Other.into(),
        Some(c) => ErrorCode::Custom(c.into()),
    })
}
