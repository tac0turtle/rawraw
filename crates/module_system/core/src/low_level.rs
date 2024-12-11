//! Low-level utilities for working with message structs and message packets directly.

use crate::error::{ClientError, HandlerError};
use crate::message::{Message, MessageBase, QueryMessage};
use crate::result::ClientResult;
use crate::Context;
use alloc::string::String;
use allocator_api2::alloc::Allocator;
use core::alloc::Layout;
use ixc_message_api::code::{ErrorCode, HandlerCode, SystemCode};
use ixc_message_api::packet::MessagePacket;
use ixc_message_api::AccountID;
use ixc_schema::buffer::WriterFactory;
use ixc_schema::codec::Codec;
use ixc_schema::mem::MemoryManager;
use ixc_schema::value::OptionalValue;

/// Dynamically invokes an account message.
/// Static account client instances should be preferred wherever possible,
/// so that static dependency analysis can be performed.
pub fn dynamic_invoke_msg<'a, 'b, M: Message<'b>>(context: &'a mut Context, account: AccountID, message: M)
                                              -> ClientResult<<M::Response<'a> as OptionalValue<'a>>::Value, M::Error>
{
    unsafe {
        let mut packet = encode_message_packet(context.caller, context.memory_manager(), account, message)?;

        let res = context.dynamic_invoke_msg(&mut packet);

        decode_message_response::<M>(context, &packet, res)
    }
}

/// Dynamically invokes an account query message.
/// Static account client instances should be preferred wherever possible,
/// so that static dependency analysis can be performed.
pub fn dynamic_invoke_query<'a, 'b, M: QueryMessage<'b>>(
    context: &'a Context,
    account: AccountID,
    message: M,
) -> ClientResult<<M::Response<'a> as OptionalValue<'a>>::Value, M::Error> {
    unsafe {
        let mut packet = encode_message_packet(context.caller, context.memory_manager(), account, message)?;

        let res = context.dynamic_invoke_query(&mut packet);

        decode_message_response::<M>(context, &packet, res)
    }
}

unsafe fn encode_message_packet<'a, 'b, M: MessageBase<'b>>(
    caller: AccountID,
    mem: &'a MemoryManager,
    account: AccountID,
    message: M,
) -> ClientResult<MessagePacket<'a>, M::Error> {
    // encode the message body
    let cdc = M::Codec::default();
    let msg_body = cdc.encode_value(&message, mem)?;

    // create the message packet and fill in call details
    let mut packet = create_packet(caller, mem, account, M::SELECTOR)?;
    let header = packet.header_mut();
    header.in_pointer1.set_slice(msg_body);
    Ok(packet)
}

unsafe fn decode_message_response<'a, 'b, M: MessageBase<'b>>(
    context: &'a Context,
    packet: &MessagePacket<'a>,
    res: Result<(), ErrorCode>,
) -> ClientResult<<M::Response<'a> as OptionalValue<'a>>::Value, M::Error> {
    let out1 = packet.header().out_pointer1.get(&packet);

    match res {
        Ok(_) => {
            let cdc = M::Codec::default();
            let res = M::Response::<'a>::decode_value(&cdc, out1, context.memory_manager())?;
            Ok(res)
        }
        Err(e) => {
            let c: u16 = e.into();
            let code = ErrorCode::<M::Error>::from(c);
            let msg = String::from_utf8(out1.to_vec())
                .map_err(|_| ErrorCode::SystemCode(SystemCode::EncodingError))?;
            Err(ClientError { message: msg, code })
        }
    }
}

/// Create a new message packet with the given account and message selector.
pub fn create_packet<'a, E: HandlerCode>(
    self_account_id: AccountID,
    allocator: &'a dyn Allocator,
    account: AccountID,
    selector: u64,
) -> ClientResult<MessagePacket<'a>, E> {
    unsafe {
        let packet = MessagePacket::allocate(allocator, 0)?;
        let header = packet.header_mut();
        header.caller = self_account_id;
        header.account = account;
        header.message_selector = selector;
        Ok(packet)
    }
}

/// Encodes the response to the out1 pointer of the message packet. Used for encoding the response of a message in macros.
pub fn encode_response<'a, 'b, M: MessageBase<'a>>(
    cdc: &dyn Codec,
    res: crate::Result<<<M as MessageBase<'a>>::Response<'a> as OptionalValue<'a>>::Value, M::Error>,
    allocator: &'b dyn Allocator,
    message_packet: &'b mut MessagePacket,
) -> core::result::Result<(), ErrorCode> {
    match res {
        Ok(value) => {
            if let Some(out1) =
                <<M as MessageBase<'a>>::Response<'a> as OptionalValue<'a>>::encode_value(
                    cdc,
                    &value,
                    &allocator as &dyn WriterFactory,
                )?
            {
                unsafe {
                    message_packet.header_mut().out_pointer1.set_slice(out1);
                }
            }
            Ok(())
        }
        Err(e) => encode_handler_error(e, allocator, message_packet),
    }
}

/// Encodes a default response to the out1 pointer of the message packet.
/// Used for encoding the response of a message in macros.
pub fn encode_default_response<'b>(
    res: crate::Result<()>,
    allocator: &'b dyn Allocator,
    message_packet: &'b mut MessagePacket,
) -> core::result::Result<(), ErrorCode> {
    match res {
        Ok(_) => Ok(()),
        Err(e) => encode_handler_error(e, allocator, message_packet),
    }
}

/// Encode a handler error to the out1 pointer of the message packet.
/// Used for encoding the response of a message in macros.
pub fn encode_handler_error<'b, E: HandlerCode>(
    err: HandlerError<E>,
    allocator: &'b dyn Allocator,
    message_packet: &'b mut MessagePacket,
) -> core::result::Result<(), ErrorCode> {
    unsafe {
        let mem = allocator
            .allocate(Layout::from_size_align_unchecked(err.msg.len(), 1))
            .map_err(|_| ErrorCode::SystemCode(SystemCode::EncodingError))?;
        let out1 = mem.as_ptr() as *mut u8;
        out1.copy_from_nonoverlapping(err.msg.as_ptr(), err.msg.len());
        message_packet
            .header_mut()
            .out_pointer1
            .set_slice(core::slice::from_raw_parts(out1, err.msg.len()));
    }
    Err(match err.code {
        None => ErrorCode::SystemCode(SystemCode::Other),
        Some(c) => ErrorCode::HandlerCode(c.into()),
    })
}
