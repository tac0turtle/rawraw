//! The Message trait for invoking messages dynamically.

use crate::handler::{ClientSchemaVisitor, InitMessage};
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
    /// Visit the events that can be emitted by the message.
    fn visit_events<V: TypeVisitor>(visitor: &mut V);
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
pub fn visit_init_descriptor<'a, M: InitMessage<'a>, V: ClientSchemaVisitor<'a>>(visitor: &mut V)  {
    let mut desc = MessageDescriptor::new(M::STRUCT_TYPE.name);
    desc.kind = MessageKind::Constructor;
    visitor.visit_message(&desc);
    visitor.visit::<M::Type>();
}

/// Extract the message descriptor for a message.
pub fn visit_message_descriptor<'a, M: Message<'a>, V: ClientSchemaVisitor<'a>>(visitor: &mut V)  {
    let mut desc = visit_message_base::<M, V>(visitor);
    desc.kind = MessageKind::Volatile;
    visitor.visit_message(&desc);
}

/// Extract the message descriptor for a query message.
pub fn visit_query_descriptor<'a, M: QueryMessage<'a>, V: ClientSchemaVisitor<'a>>(visitor: &mut V)  {
    let mut desc = visit_message_base::<M, V>(visitor);
    desc.kind = MessageKind::Query;
    visitor.visit_message(&desc);
}

fn visit_message_base<'a, M: MessageBase<'a>, V: ClientSchemaVisitor<'a>>(visitor: &mut V) -> MessageDescriptor<'static> {
    M::visit_type(visitor);
    let mut desc: MessageDescriptor = MessageDescriptor::new(M::STRUCT_TYPE.name);
    if let Some(res) = M::Response::SCHEMA_TYPE {
        desc.response_type = Some(res.name());
    }
    desc
}
