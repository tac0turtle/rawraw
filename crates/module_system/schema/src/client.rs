//! Client descriptors. T

use crate::list::List;
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
    pub messages: List<'a, MessageDescriptor<'a>>,
}

impl<'a> ClientDescriptor<'a> {
    /// Create a new client descriptor with a name and account ID but no messages.
    pub const fn new(name: &'a str, account_id: AccountID) -> Self {
        Self {
            name,
            account_id,
            messages: List::Empty,
        }
    }
}
