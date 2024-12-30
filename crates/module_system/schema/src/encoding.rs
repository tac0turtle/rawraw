//! Well-known encoding types.
use ixc_schema_macros::SchemaValue;

/// The encoding of a message or state object.
#[derive(Debug, Clone, Eq, PartialEq, SchemaValue, Default)]
#[non_exhaustive]
pub enum Encoding {
    /// An unknown, undecodable encoding.
    #[default]
    Unknown,
    /// The native binary encoding.
    NativeBinary,
}
