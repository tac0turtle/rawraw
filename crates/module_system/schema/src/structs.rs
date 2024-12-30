//! Struct codec and schema traits.

use crate::decoder::{DecodeError, Decoder};
use crate::encoder::{EncodeError, Encoder};
use crate::field::Field;
use crate::types::TypeVisitor;
use ixc_schema_macros::SchemaValue;

/// StructSchema describes the schema of a struct.
/// # Safety
/// The trait is marked as unsafe because it is meant to be implemented by macros.
pub unsafe trait StructSchema {
    /// The schema of the struct.
    const STRUCT_TYPE: StructType<'static>;

    /// A hash of the struct's name which should be unique within a reasonable schema.
    /// Can be used to decode a struct from a message by matching on its type selector.
    const TYPE_SELECTOR: u64;

    /// Visit the struct's field types.
    fn visit_field_types<V: TypeVisitor>(visitor: &mut V);
}

/// StructDecodeVisitor is the trait that should be derived to decode a struct.
/// # Safety
/// The trait is marked as unsafe because it is meant to be implemented by macros.
pub unsafe trait StructDecodeVisitor<'a> {
    /// Decode a field from the input data.
    fn decode_field(
        &mut self,
        index: usize,
        decoder: &mut dyn Decoder<'a>,
    ) -> Result<(), DecodeError>;
}

/// StructEncodeVisitor is the trait that should be derived to encode a struct.
/// # Safety
/// the trait is marked as unsafe to detour users from using it
pub unsafe trait StructEncodeVisitor {
    /// Encode a field to the output data.
    fn encode_field(&self, index: usize, encoder: &mut dyn Encoder) -> Result<(), EncodeError>;
}

/// StructType contains the schema of a struct.
/// # Safety
/// the trait is marked as unsafe to detour users from using it
#[derive(Debug, Clone, Eq, PartialEq, SchemaValue, Default)]
#[non_exhaustive]
pub struct StructType<'a> {
    /// The name of the struct.
    pub name: &'a str,
    /// The fields of the struct.
    pub fields: &'a [Field<'a>],
    /// Sealed indicates whether new fields can be added to the struct.
    /// If sealed is true, the struct is considered sealed and new fields cannot be added.
    pub sealed: bool,
}

impl<'a> StructType<'a> {
    /// Create a new struct type.
    pub const fn new(name: &'a str, fields: &'a [Field<'a>], sealed: bool) -> Self {
        Self {
            name,
            fields,
            sealed,
        }
    }
}
