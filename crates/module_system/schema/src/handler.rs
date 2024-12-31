//! Account handler schemas.

use crate::client::ClientDescriptor;
use crate::list::List;
use crate::message::MessageDescriptor;
use crate::schema::SchemaType;
use crate::state_object::StateObjectDescriptor;
use ixc_schema_macros::SchemaValue;

/// An account handler schema.
#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq, Default, SchemaValue)]
pub struct HandlerSchema<'a> {
    /// The types used by the account in its public API or to interact with other accounts.
    pub types: List<'a, SchemaType<'a>>,
    /// The messages that the account handles.
    pub messages: List<'a, MessageDescriptor<'a>>,
    /// The state objects defining the account's state.
    pub state_objects: List<'a, StateObjectDescriptor<'a>>,
    /// The statically defined clients that the client uses to communicate with other accounts.
    pub clients: List<'a, ClientDescriptor<'a>>,
}
