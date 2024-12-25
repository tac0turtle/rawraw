//! State object traits.

mod field_types;
mod key;
mod key_field;
mod prefix;
mod value;
mod value_field;

use ixc_schema_macros::SchemaValue;
use crate::field::Field;
pub use key::{decode_object_key, encode_object_key, ObjectKey};
pub use key_field::KeyFieldValue;
pub use prefix::PrefixKey;
pub use value::{decode_object_value, encode_object_value, ObjectValue};
pub use value_field::{Bytes, ObjectFieldValue, Str};

/// A type representing objects stored in key-value store state.
#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq, SchemaValue, Default)]
pub struct StateObjectType<'a> {
    /// The name of the object.
    pub name: &'a str,
    /// The fields that make up the primary key.
    pub key_fields: &'a [Field<'a>],
    /// The fields that make up the value.
    pub value_fields: &'a [Field<'a>],
    /// Whether to retain deletions in off-chain, indexed state.
    pub retain_deletions: bool,
}
