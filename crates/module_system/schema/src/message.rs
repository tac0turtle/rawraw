//! Message descriptors.
use crate::encoding::Encoding;
use ixc_schema_macros::SchemaValue;

/// Describes a message that can be sent to an account.
#[derive(Debug, Clone, Eq, PartialEq, SchemaValue, Default)]
#[non_exhaustive]
pub struct MessageDescriptor<'a> {
    /// The kind of message, e.g. volatile, query, constructor, pure.
    pub kind: MessageKind,
    /// The encoding of the message.
    pub encoding: Encoding,
    /// The name of the request type.
    pub request_type: &'a str,
    /// The name of the response type, if any.
    pub response_type: Option<&'a str>,
    /// The names of the events that can be emitted by the message.
    pub events: &'a [&'a str],
    /// The names of the custom error codes that can be returned by the message.
    pub error_codes: &'a [ErrorCodeDescriptor<'a>],
}

impl<'a> MessageDescriptor<'a> {
    /// Create a new message descriptor.
    pub const fn new(request_type: &'a str) -> Self {
        Self {
            encoding: Encoding::Unknown,
            kind: MessageKind::Volatile,
            request_type,
            response_type: None,
            events: &[],
            error_codes: &[],
        }
    }
}

/// The kind of message.
#[derive(Debug, Clone, Eq, PartialEq, SchemaValue, Default)]
#[non_exhaustive]
pub enum MessageKind {
    /// A regular message that can update state.
    #[default]
    Volatile,
    /// A message that only reads state.
    Query,
    /// A message used to create an account.
    Constructor,
    /// A message that neither reads nor updates state.
    Pure,
}

/// A descriptor for a custom error code.
#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq, SchemaValue, Default)]
pub struct ErrorCodeDescriptor<'a> {
    /// The name of the error code.
    pub name: &'a str,
    /// The value of the error code.
    pub value: i32,
}
