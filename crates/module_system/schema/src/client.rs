//! Client descriptors.
use crate::message::MessageDescriptor;
use ixc_message_api::AccountID;
use ixc_schema_macros::SchemaValue;

/// Describes a client that can be used to communicate with other accounts.
#[derive(Debug, Clone, Eq, PartialEq, SchemaValue, Default)]
#[non_exhaustive]
pub struct ClientDescriptor<'a> {
    /// The name of the client. Generally just some locally meaningful name.
    pub name: &'a str,
    /// The ID of the account that the client is associated with.
    pub account_id: AccountID,
    /// The messages that the client can send.
    pub messages: &'a [MessageDescriptor<'a>],
}
