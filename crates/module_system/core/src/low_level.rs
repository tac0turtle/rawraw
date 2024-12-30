//! Low-level utilities for working with message structs and message packets directly.

use crate::error::{ClientError, HandlerError};
use crate::message::{Message, MessageBase, QueryMessage};
use crate::result::ClientResult;
use crate::Context;
use allocator_api2::alloc::Allocator;
use ixc_core_macros::message_selector;
use ixc_message_api::code::{ErrorCode, HandlerCode, SystemCode};
use ixc_message_api::gas::GasTracker;
use ixc_message_api::handler::InvokeParams;
use ixc_message_api::message::{MessageSelector, Request, Response};
use ixc_message_api::AccountID;
use ixc_schema::binary::NativeBinaryCodec;
use ixc_schema::codec::Codec;
use ixc_schema::mem::MemoryManager;
use ixc_schema::structs::StructSchema;
use ixc_schema::value::OptionalValue;
use ixc_schema::SchemaValue;

/// Dynamically invokes an account message.
/// Static account client instances should be preferred wherever possible,
/// so that static dependency analysis can be performed.
pub fn dynamic_invoke_msg<'a, 'b, M: Message<'b>>(
    context: &mut Context<'a>,
    account: AccountID,
    message: M,
) -> ClientResult<<M::Response<'a> as OptionalValue<'a>>::Value, M::Error> {
    dynamic_invoke_msg_with_gas_tracker(context, account, message, None)
}

/// Dynamically invokes a message with a gas tracker which can
/// be used to limit and track gas consumption of the message.
pub fn dynamic_invoke_msg_with_gas_tracker<'a, 'b, 'c, M: Message<'b>>(
    context: &mut Context<'a>,
    account: AccountID,
    message: M,
    gas_tracker: Option<&'c GasTracker>,
) -> ClientResult<<M::Response<'a> as OptionalValue<'a>>::Value, M::Error> {
    let packet = encode_message_packet(context.memory_manager(), account, message)?;
    let res = dynamic_invoke_msg_packet(context, &packet, gas_tracker);
    decode_message_response::<M>(context, &res)
}

/// Dynamically invokes an account query message.
/// Static account client instances should be preferred wherever possible,
/// so that static dependency analysis can be performed.
pub fn dynamic_invoke_query<'a, 'b, M: QueryMessage<'b>>(
    context: &Context<'a>,
    account: AccountID,
    message: M,
) -> ClientResult<<M::Response<'a> as OptionalValue<'a>>::Value, M::Error> {
    dynamic_invoke_query_with_gas_tracker(context, account, message, None)
}

/// Dynamically invokes a query message with a gas tracker which can
/// be used to limit and track gas consumption of the message.
pub fn dynamic_invoke_query_with_gas_tracker<'a, 'b, 'c, M: QueryMessage<'b>>(
    context: &Context<'a>,
    account: AccountID,
    message: M,
    gas_tracker: Option<&'c GasTracker>,
) -> ClientResult<<M::Response<'a> as OptionalValue<'a>>::Value, M::Error> {
    let packet = encode_message_packet(context.memory_manager(), account, message)?;
    let res = dynamic_invoke_query_packet(context, &packet, gas_tracker);
    decode_message_response::<M>(context, &res)
}

/// Dynamically invoke a raw query message packet.
pub fn dynamic_invoke_query_packet<'a>(
    ctx: &Context<'a>,
    msg: &ixc_message_api::message::Message,
    gas_tracker: Option<&GasTracker>,
) -> Result<Response<'a>, ErrorCode> {
    let invoke_params = InvokeParams::new(ctx.mem, gas_tracker);
    ctx.with_backend(|backend| backend.invoke_query(msg, &invoke_params))
}

/// Dynamically invoke a raw message packet.
pub fn dynamic_invoke_msg_packet<'a>(
    ctx: &mut Context<'a>,
    msg: &ixc_message_api::message::Message,
    gas_limit: Option<&GasTracker>,
) -> Result<Response<'a>, ErrorCode> {
    let invoke_params = InvokeParams::new(ctx.mem, gas_limit);
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
        Request::new1(M::TYPE_SELECTOR, msg_body.into()),
    ))
}

fn decode_message_response<'a, 'b, M: MessageBase<'b>>(
    context: &Context<'a>,
    res: &Result<Response<'a>, ErrorCode>,
) -> ClientResult<<M::Response<'a> as OptionalValue<'a>>::Value, M::Error> {
    match res {
        Ok(res) => {
            let cdc = M::Codec::default();
            let res = M::Response::<'a>::decode_value(&cdc, &res.out1(), context.memory_manager())?;
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
) -> Result<Response<'b>, ::ixc_message_api::error::HandlerError> {
    match res {
        Ok(value) => {
            if let Some(out1) =
                <<M as MessageBase<'a>>::Response<'a> as OptionalValue<'a>>::encode_value(
                    cdc, &value, allocator,
                )
                .map_err(|_| {
                    ixc_message_api::error::HandlerError::new(ErrorCode::SystemCode(
                        SystemCode::EncodingError,
                    ))
                })?
            {
                Ok(Response::new1(out1.into()))
            } else {
                Ok(Response::default())
            }
        }
        Err(e) => Err(encode_handler_error(e)),
    }
}

/// Encodes a default response to the out1 pointer of the message packet.
/// Used for encoding the response of a message in macros.
pub fn encode_default_response<'b>(
    res: crate::Result<()>,
) -> Result<Response<'b>, ::ixc_message_api::error::HandlerError> {
    match res {
        Ok(_) => Ok(Default::default()),
        Err(e) => Err(encode_handler_error(e)),
    }
}

/// Encode a handler error to the out1 pointer of the message packet.
/// Used for encoding the response of a message in macros.
pub fn encode_handler_error<E: HandlerCode + SchemaValue<'static>>(
    err: HandlerError<E>,
) -> ixc_message_api::error::HandlerError {
    let code: u16 = err.code.into();
    let mut res = ixc_message_api::error::HandlerError::new(code.into());
    set_error_message(err, &mut res);
    res
}

#[cfg(feature = "std")]
fn set_error_message<E: HandlerCode + SchemaValue<'static>>(
    err: HandlerError<E>,
    res: &mut ixc_message_api::error::HandlerError,
) {
    res.message = err.msg;
}

#[cfg(not(feature = "std"))]
fn set_error_message<E: HandlerCode + SchemaValue<'static>>(
    _err: HandlerError<E>,
    _res: &mut ixc_message_api::error::HandlerError,
) {
}

/// Emits an event.
pub fn emit_event<'a, E: StructSchema + SchemaValue<'a>>(
    ctx: &mut Context,
    event: &E,
) -> ClientResult<()> {
    let cdc = NativeBinaryCodec;
    let event_bytes = cdc.encode_value(event, ctx.memory_manager())?;
    let req = Request::new2(
        EMIT_EVENT_SELECTOR,
        event_bytes.into(),
        E::TYPE_SELECTOR.into(),
    );
    let params = InvokeParams::new(ctx.memory_manager(), None);
    let _ = ctx.with_backend_mut(|backend| backend.update_state(&req, &params))?;
    Ok(())
}

const EMIT_EVENT_SELECTOR: MessageSelector = message_selector!("ixc.events.1.emit");
