//! The Message trait for invoking messages dynamically.

use crate::handler::InitMessage;
use ixc_message_api::code::HandlerCode;
use ixc_schema::codec::Codec;
use ixc_schema::message::{MessageDescriptor, MessageKind};
use ixc_schema::structs::StructSchema;
use ixc_schema::types::TypeVisitor;
use ixc_schema::value::{OptionalValue, SchemaValue};

/// The MessageBase trait for invoking messages dynamically.
pub trait MessageBase<'a>: SchemaValue<'a> + StructSchema {
    /// The optional response type.
    type Response<'b>: OptionalValue<'b>;
    /// The optional error type.
    type Error: HandlerCode;
    /// The codec to use for encoding and decoding the message.
    type Codec: Codec + Default;
}

/// The Message trait for invoking messages dynamically.
pub trait Message<'a>: MessageBase<'a> {
    // /// Visit the events that can be emitted by the message.
    // fn visit_events<V: TypeVisitor>(visitor: &mut V);
}

/// The QueryMessage trait for invoking query messages dynamically.
pub trait QueryMessage<'a>: MessageBase<'a> {}

/// Extract the response and error types from a Result.
/// Used internally in macros for building the Message implementation and ClientResult type.
pub trait ExtractResponseTypes {
    /// The response type.
    type Response;
    /// The error type.
    type Error;
    /// The client result type.
    type ClientResult;
}

impl<R, E: HandlerCode> ExtractResponseTypes for crate::Result<R, E> {
    type Response = R;
    type Error = E;
    type ClientResult = crate::result::ClientResult<R, E>;
}

/// Extract the message descriptor for an init message.
pub const fn extract_init_descriptor<'a, M: InitMessage<'a>>() -> MessageDescriptor<'static> {
    let mut desc = MessageDescriptor::new(M::STRUCT_TYPE.name);
    desc.kind = MessageKind::Constructor;
    desc
}

/// Extract the message descriptor for a message.
pub const fn extract_message_descriptor<'a, M: Message<'a>>() -> MessageDescriptor<'static> {
    let mut desc = extract_message_descriptor_base::<M>();
    desc.kind = MessageKind::Volatile;
    desc
}

/// Extract the message descriptor for a query message.
pub const fn extract_query_descriptor<'a, M: QueryMessage<'a>>() -> MessageDescriptor<'static> {
    let mut desc = extract_message_descriptor_base::<M>();
    desc.kind = MessageKind::Query;
    desc
}

const fn extract_message_descriptor_base<'a, M: MessageBase<'a>>() -> MessageDescriptor<'static> {
    let mut desc: MessageDescriptor = MessageDescriptor::new(M::STRUCT_TYPE.name);
    if let Some(res) = M::Response::SCHEMA_TYPE {
        desc.response_type = Some(res.name());
    }
    desc
}
