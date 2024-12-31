//! Account handler schemas.

use crate::client::ClientDescriptor;
use crate::list::List;
use crate::message::MessageDescriptor;
use crate::schema::SchemaType;
use crate::state_object::StateObjectDescriptor;
use crate::types::TypeMap;
use allocator_api2::alloc::Allocator;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::AccountID;
use ixc_schema_macros::SchemaValue;

/// An account handler schema.
#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq, Default, SchemaValue)]
pub struct HandlerSchema<'a> {
    /// The fully qualified identifier of the handler.
    pub id: &'a str,
    /// The types used by the account in its public API or to interact with other accounts.
    pub types: List<'a, SchemaType<'a>>,
    /// The messages that the account handles.
    pub messages: List<'a, MessageDescriptor<'a>>,
    /// The state objects defining the account's state.
    pub state_objects: List<'a, StateObjectDescriptor<'a>>,
    /// The statically defined clients that the client uses to communicate with other accounts.
    pub clients: List<'a, ClientDescriptor<'a>>,
}

/// A wrapper around a handler schema and its type map.
pub struct HandlerSchemaInfo<'a> {
    /// The handler schema.
    pub schema: HandlerSchema<'a>,
    /// A type map for the handler's types.
    pub type_map: TypeMap<'a>,
}

/// A resolver for handler schemas.
pub trait HandlerSchemaResolver {
    /// Get the schema for the provided account ID.
    fn schema_for_account<'a>(
        &self,
        account_id: &AccountID,
        allocator: &'a dyn Allocator,
    ) -> Option<HandlerSchemaInfo<'a>>;
    /// Get the schema for the provided handler ID.
    fn schema_for_handler<'a>(
        &self,
        handler_id: &str,
        allocator: &'a dyn Allocator,
    ) -> Option<HandlerSchemaInfo<'a>>;
}

/// An empty resolver that returns no schema.
pub struct EmptyHandlerSchemaResolver;

impl HandlerSchemaResolver for EmptyHandlerSchemaResolver {
    fn schema_for_account<'a>(
        &self,
        _account_id: &AccountID,
        _allocator: &'a dyn Allocator,
    ) -> Option<HandlerSchemaInfo<'a>> {
        None
    }

    fn schema_for_handler<'a>(
        &self,
        _handler_id: &str,
        _allocator: &'a dyn Allocator,
    ) -> Option<HandlerSchemaInfo<'a>> {
        None
    }
}

pub fn type_map_from_schema<'a>(schema: &HandlerSchema<'a>, allocator: &'a dyn Allocator) -> TypeMap<'a> {
    let mut type_map = TypeMap::new(allocator);
    for typ in schema.types.as_slice().iter() {
        type_map.name_to_type.insert(typ.name(), typ.clone());
        if let SchemaType::Struct(struct_type) = typ {
            type_map.selector_to_type.insert(struct_type.type_selector, struct_type.clone());
        }
    }
    type_map
}