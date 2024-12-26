//! Schema definition.

// WARNING: this is a terrible hack to make macros work
// either with ixc_schema or just ixc with the use_ixc_macro_path feature,
// hopefully some day we find a better solution!
#[cfg(feature = "use_ixc_macro_path")]
pub(crate) use crate::*;

use crate::client::ClientDescriptor;
use crate::enums::EnumType;
use crate::message::MessageDescriptor;
use crate::state_object::StateObjectDescriptor;
use crate::structs::StructType;
use crate::SchemaValue;
use ixc_schema_macros::SchemaValue;

/// A type in a schema.
#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq, Default, SchemaValue)]
#[repr(u8)]
pub enum SchemaType<'a> {
    /// An invalid type.
    #[default]
    Invalid,
    /// A struct type.
    Struct(StructType<'a>),
    /// An enum type.
    Enum(EnumType<'a>),
}

impl<'a> SchemaType<'a> {
    /// Get the name of the schema type.
    pub const fn name(&self) -> &'a str {
        match self {
            SchemaType::Invalid => "",
            SchemaType::Struct(s) => s.name,
            SchemaType::Enum(e) => e.name,
        }
    }
}

impl PartialOrd for SchemaType<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SchemaType<'_> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.name().cmp(other.name())
    }
}

/// An account handler schema.
#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq, Default, SchemaValue)]
pub struct Schema<'a> {
    pub types: &'a [SchemaType<'a>],
    pub messages: &'a [MessageDescriptor<'a>],
    pub state_objects: &'a [StateObjectDescriptor<'a>],
    pub clients: &'a [ClientDescriptor<'a>],
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json;
    use crate::types::collect_types;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn test_schema_in_schema() {
        let types_map = collect_types::<SchemaType>().unwrap();
        let types_vec = types_map.values().cloned().collect::<Vec<_>>();
        let schema_schema = Schema {
            types: types_vec.as_slice(),
            messages: &[],
            state_objects: &[],
            clients: &[],
        };
        let as_json = json::encode_value(&schema_schema).unwrap();
        println!("{}", as_json);
    }
}
