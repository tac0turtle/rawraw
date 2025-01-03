//! The decoder trait and error type.

use crate::enums::{EnumDecodeVisitor, EnumType};
use crate::list::ListDecodeVisitor;
use crate::mem::MemoryManager;
use crate::structs::{StructDecodeVisitor, StructType};
use crate::value::ValueCodec;
use crate::SchemaValue;
use core::error::Error;
use core::fmt::{Display, Formatter};
use ixc_message_api::code::{ErrorCode, SystemCode};
use ixc_message_api::AccountID;

/// The trait that decoders must implement.
pub trait Decoder<'a> {
    /// Decode a `bool`.
    fn decode_bool(&mut self) -> Result<bool, DecodeError>;
    /// Decode a `u8`.
    fn decode_u8(&mut self) -> Result<u8, DecodeError>;
    /// Decode a `u16`.
    fn decode_u16(&mut self) -> Result<u16, DecodeError>;
    /// Decode a `u32`.
    fn decode_u32(&mut self) -> Result<u32, DecodeError>;
    /// Decode a `u64`.
    fn decode_u64(&mut self) -> Result<u64, DecodeError>;
    /// Decode a `u128`.
    fn decode_u128(&mut self) -> Result<u128, DecodeError>;
    /// Decode a `i8`.
    fn decode_i8(&mut self) -> Result<i8, DecodeError>;
    /// Decode a `i16`.
    fn decode_i16(&mut self) -> Result<i16, DecodeError>;
    /// Decode a `i32`.
    fn decode_i32(&mut self) -> Result<i32, DecodeError>;
    /// Decode a `i64`.
    fn decode_i64(&mut self) -> Result<i64, DecodeError>;
    /// Decode a `i128`.
    fn decode_i128(&mut self) -> Result<i128, DecodeError>;
    /// Decode a borrowed `str`.
    fn decode_borrowed_str(&mut self) -> Result<&'a str, DecodeError>;
    #[cfg(feature = "std")]
    /// Decode an owned `String`.
    fn decode_owned_str(&mut self) -> Result<alloc::string::String, DecodeError>;
    /// Decode borrowed bytes.
    fn decode_borrowed_bytes(&mut self) -> Result<&'a [u8], DecodeError>;
    #[cfg(feature = "std")]
    /// Decode owned bytes.
    fn decode_owned_bytes(&mut self) -> Result<alloc::vec::Vec<u8>, DecodeError>;
    /// Decode a struct.
    fn decode_struct(
        &mut self,
        visitor: &mut dyn StructDecodeVisitor<'a>,
        struct_type: &StructType,
    ) -> Result<(), DecodeError>;
    /// Decode a list.
    fn decode_list(&mut self, visitor: &mut dyn ListDecodeVisitor<'a>) -> Result<(), DecodeError>;
    /// Decode an optional value. The visitor will only be called if the value is present.
    /// Returns `true` if the value is present, `false` if it is not.
    fn decode_option(&mut self, visitor: &mut dyn ValueCodec<'a>) -> Result<bool, DecodeError>;
    /// Decode an account ID.
    fn decode_account_id(&mut self) -> Result<AccountID, DecodeError>;
    /// Encode an enum value.
    fn decode_enum_variant(
        &mut self,
        visitor: &mut dyn EnumDecodeVisitor<'a>,
        enum_type: &EnumType,
    ) -> Result<(), DecodeError>;

    /// Get the memory manager.
    fn mem_manager(&self) -> &'a MemoryManager;
}

/// A decoding error.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum DecodeError {
    /// The input data is out of data.
    OutOfData,
    /// The input data is invalid.
    InvalidData,
    /// An unknown and unhandled field number was encountered.
    UnknownField,
    /// The input data contains an invalid UTF-8 string.
    InvalidUtf8,
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            DecodeError::OutOfData => write!(f, "out of data"),
            DecodeError::InvalidData => write!(f, "invalid data"),
            DecodeError::UnknownField => write!(f, "unknown field number"),
            DecodeError::InvalidUtf8 => write!(f, "invalid UTF-8"),
        }
    }
}

impl Error for DecodeError {}

impl From<DecodeError> for ErrorCode {
    fn from(_value: DecodeError) -> Self {
        ErrorCode::SystemCode(SystemCode::EncodingError)
    }
}

/// Decode a single value.
pub fn decode_one<'a, V: SchemaValue<'a>>(decoder: &mut dyn Decoder<'a>) -> Result<V, DecodeError> {
    let mut x = V::default();
    x.decode(decoder)?;
    Ok(x)
}
