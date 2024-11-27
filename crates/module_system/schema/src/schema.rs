//! Schema definition.
use crate::enums::EnumType;
use crate::message::MessageDescriptor;
use crate::oneof::OneOfType;
use crate::state_object::StateObjectType;
use crate::structs::StructType;

/// A type in a schema.
#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SchemaType<'a> {
    /// A struct type.
    Struct(StructType<'a>),
    /// An enum type.
    Enum(EnumType<'a>),
    /// A one-of type.
    OneOf(OneOfType<'a>),
    /// A state object type.
    StateObjectType(StateObjectType<'a>),
}

impl<'a> SchemaType<'a> {
    /// Get the name of the schema type.
    pub const fn name(&self) -> &'a str {
        match self {
            SchemaType::Struct(s) => s.name,
            SchemaType::Enum(e) => e.name,
            SchemaType::OneOf(o) => o.name,
            SchemaType::StateObjectType(s) => s.name,
        }
    }
}

impl PartialOrd for SchemaType<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.name().partial_cmp(other.name())
    }
}

impl Ord for SchemaType<'_> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.name().cmp(other.name())
    }
}

/// A schema.
#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Schema<'a> {
    types: &'a [SchemaType<'a>],
    messages: &'a [MessageDescriptor<'a>],
}

// impl Schema<'static> {
//     pub const fn add(&self, schema_type: SchemaType<'static>) -> Self {
//         todo!()
//     }
// }
//
// pub trait HasSchema {
//     const SCHEMA: Schema<'static>;
// }

// WARNING: this is a terrible hack to make macros work
// either with ixc_schema or just ixc with the use_ixc_macro_path feature,
// hopefully some day we find a better solution!
#[cfg(feature = "use_ixc_macro_path")]
pub(crate) use crate::*;
