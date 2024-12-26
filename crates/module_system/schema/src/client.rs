use crate::message::MessageDescriptor;
use ixc_message_api::AccountID;
use ixc_schema_macros::SchemaValue;

#[derive(Debug, Clone, Eq, PartialEq, SchemaValue, Default)]
#[non_exhaustive]
pub struct ClientDescriptor<'a> {
    pub name: &'a str,
    pub account_id: AccountID,
    pub messages: &'a [MessageDescriptor<'a>],
}
