//! Field kinds.
use ixc_schema_macros::SchemaValue;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// The basic type of a field.
#[non_exhaustive]
#[repr(i32)]
#[derive(
    TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy, PartialEq, Eq, SchemaValue, Default,
)]
pub enum Kind {
    /// An unknown and invalid kind.
    #[default]
    Invalid = 0,
    /// A string.
    String = 1,
    /// A byte array.
    Bytes = 2,
    /// A signed 8-bit integer.
    Int8 = 3,
    /// An unsigned 8-bit integer.
    Uint8 = 4,
    /// A signed 16-bit integer.
    Int16 = 5,
    /// An unsigned 16-bit integer.
    Uint16 = 6,
    /// A signed 32-bit integer.
    Int32 = 7,
    /// An unsigned 32-bit integer.
    Uint32 = 8,
    /// A signed 64-bit integer.
    Int64 = 9,
    /// An unsigned 64-bit integer.
    Uint64 = 10,
    /// A signed 128-bit integer.
    Int128,
    /// An unsigned N-byte integer.
    UInt128,
    /// A boolean.
    Bool,
    /// A timestamp with nano-second precision.
    Time,
    /// A duration with nano-second precision.
    Duration,
    /// An account ID.
    AccountID,
    /// An enumeration value.
    Enum,
    /// A JSON value.
    Struct,
    /// A list value.
    List,
    /// Any message.
    AnyMessage
}
