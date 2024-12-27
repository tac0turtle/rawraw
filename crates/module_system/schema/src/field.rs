//! Field definition.

use crate::kind::Kind;
use ixc_schema_macros::SchemaValue;

/// A field in a type.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, SchemaValue)]
#[non_exhaustive]
pub struct Field<'a> {
    /// The name of the field.
    pub name: &'a str,
    /// The kind of the field.
    pub kind: Kind,
    /// Whether the field is nullable.
    pub nullable: bool,
    /// The element kind for list fields.
    pub element_kind: Option<Kind>,
    /// The referenced type for fields which reference another type.
    pub referenced_type: Option<&'a str>,
}

impl<'a> Field<'a> {
    /// Create a new field.
    pub const fn new(name: &'a str, kind: Kind, nullable: bool, element_kind: Option<Kind>, referenced_type: Option<&'a str>) -> Self {
        Self {
            name,
            kind,
            nullable,
            element_kind,
            referenced_type,
        }
    }

    /// Returns a copy of the field with the provided name set.
    pub const fn with_name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }
}
