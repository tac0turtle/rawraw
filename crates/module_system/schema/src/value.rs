//! This module contains traits that must be implemented by types that can be used in the schema.

use crate::buffer::WriterFactory;
use crate::codec::{decode_value, Codec};
use crate::decoder::{DecodeError, Decoder};
use crate::encoder::{EncodeError, Encoder};
use crate::list::AllocatorVecBuilder;
use crate::mem::MemoryManager;
use crate::types::*;

/// A visitor for decoding values. Unlike SchemaValue, this trait is object safe.
pub trait ValueCodec<'a> {
    /// Visit the value.
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError>;

    /// Visit the value.
    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError>;
}

/// Any type used directly as a message function argument or struct field must implement this trait.
/// Unlike [`ObjectFieldValue`](crate::state_object::ObjectFieldValue) it takes a lifetime parameter so value may already be borrowed where it is
/// declared.
pub trait SchemaValue<'a>: ValueCodec<'a> + Default + 'a {
    /// The type of the value.
    type Type: Type;
}

impl<'a> ValueCodec<'a> for u8 {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_u8()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_u8(*self)
    }
}

impl<'a> SchemaValue<'a> for u8 {
    type Type = u8;
}

impl<'a> ValueCodec<'a> for u16 {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_u16()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_u16(*self)
    }
}

impl<'a> SchemaValue<'a> for u16 {
    type Type = u16;
}

impl<'a> ValueCodec<'a> for u32 {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_u32()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_u32(*self)
    }
}

impl<'a> SchemaValue<'a> for u32 {
    type Type = u32;
}

impl<'a> ValueCodec<'a> for u64 {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_u64()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_u64(*self)
    }
}

impl<'a> SchemaValue<'a> for u64 {
    type Type = u64;
}

impl<'a> ValueCodec<'a> for u128 {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_u128()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_u128(*self)
    }
}

impl<'a> SchemaValue<'a> for u128 {
    type Type = UIntNT<16>;
}

impl<'a> ValueCodec<'a> for i8 {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_i8()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_i8(*self)
    }
}

impl<'a> SchemaValue<'a> for i8 {
    type Type = i8;
}

impl<'a> ValueCodec<'a> for i16 {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_i16()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_i16(*self)
    }
}

impl<'a> SchemaValue<'a> for i16 {
    type Type = i16;
}

impl<'a> ValueCodec<'a> for i32 {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_i32()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_i32(*self)
    }
}

impl<'a> SchemaValue<'a> for i32 {
    type Type = i32;
}

impl<'a> ValueCodec<'a> for i64 {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_i64()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_i64(*self)
    }
}

impl<'a> SchemaValue<'a> for i64 {
    type Type = i64;
}

impl<'a> ValueCodec<'a> for i128 {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_i128()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_i128(*self)
    }
}

impl<'a> SchemaValue<'a> for i128 {
    type Type = IntNT<16>;
}

impl<'a> ValueCodec<'a> for bool {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_bool()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_bool(*self)
    }
}

impl<'a> SchemaValue<'a> for bool {
    type Type = bool;
}

impl<'a> ValueCodec<'a> for &'a str {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_borrowed_str()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_str(self)
    }
}

impl<'a> SchemaValue<'a> for &'a str {
    type Type = StrT;
}

#[cfg(feature = "std")]
impl<'a> ValueCodec<'a> for alloc::string::String {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_owned_str()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_str(self)
    }
}

#[cfg(feature = "std")]
impl<'a> SchemaValue<'a> for alloc::string::String {
    type Type = StrT;
}

impl<'a> ValueCodec<'a> for simple_time::Time {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_time()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_time(*self)
    }
}

impl<'a> SchemaValue<'a> for simple_time::Time {
    type Type = TimeT;
}

impl<'a> ValueCodec<'a> for simple_time::Duration {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_duration()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_duration(*self)
    }
}

impl<'a> SchemaValue<'a> for simple_time::Duration {
    type Type = DurationT;
}

impl<'a, V: SchemaValue<'a>> ValueCodec<'a> for Option<V> {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        let mut value = V::default();
        if decoder.decode_option(&mut value)? {
            *self = Some(value);
        } else {
            *self = None;
        }
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        match self {
            Some(value) => encoder.encode_option(Some(value as &dyn ValueCodec)),
            None => encoder.encode_option(None),
        }
    }
}

impl<'a, V: SchemaValue<'a>> SchemaValue<'a> for Option<V> {
    type Type = Option<V::Type>;
}

impl<'a> ValueCodec<'a> for &'a [u8] {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_borrowed_bytes()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_bytes(self)
    }
}

impl<'a> SchemaValue<'a> for &'a [u8] {
    type Type = BytesT;
}

impl<'a> ValueCodec<'a> for alloc::vec::Vec<u8> {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_owned_bytes()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_bytes(self)
    }
}

impl<'a> SchemaValue<'a> for alloc::vec::Vec<u8> {
    type Type = BytesT;
}
/// A trait that must be implemented by value types that can be used as list elements.
pub trait ListElementValue<'a>: SchemaValue<'a>
where
    Self::Type: ListElementType,
{
}

impl<'a, V: ListElementValue<'a>> ValueCodec<'a> for &'a [V]
where
    V::Type: ListElementType,
{
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        let mut builder = AllocatorVecBuilder::<'a, V>::default();
        decoder.decode_list(&mut builder)?;
        match builder.xs {
            None => *self = &[],
            Some(xs) => *self = decoder.mem_manager().unpack_slice(xs),
        }
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_list(self)
    }
}

impl<'a, V: ListElementValue<'a>> SchemaValue<'a> for &'a [V]
where
    V::Type: ListElementType,
{
    type Type = ListT<V::Type>;
}

#[cfg(feature = "std")]
impl<'a, V: ListElementValue<'a>> ValueCodec<'a> for alloc::vec::Vec<V>
where
    V::Type: ListElementType,
{
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        decoder.decode_list(self)
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_list(self)
    }
}

#[cfg(feature = "std")]
impl<'a, V: ListElementValue<'a>> SchemaValue<'a> for alloc::vec::Vec<V>
where
    V::Type: ListElementType,
{
    type Type = ListT<V::Type>;
}

impl<'a> ValueCodec<'a> for ixc_message_api::AccountID {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_account_id()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_account_id(*self)
    }
}

impl<'a> SchemaValue<'a> for ixc_message_api::AccountID {
    type Type = AccountIdT;
}

/// OptionalValue is a trait that must be implemented by types that can be used as the return value
/// or anywhere else where a value may or may not be necessary.
/// The unit type `()` is used to represent the absence of a value.
pub trait OptionalValue<'a> {
    /// The value type that is returned.
    type Value;

    /// Decode the value.
    fn decode_value(
        cdc: &dyn Codec,
        data: &'a [u8],
        memory_manager: &'a MemoryManager,
    ) -> Result<Self::Value, DecodeError>;

    /// Encode the value.
    fn encode_value<'b>(
        cdc: &dyn Codec,
        value: &Self::Value,
        writer_factory: &'b dyn WriterFactory,
    ) -> Result<Option<&'b [u8]>, EncodeError>;
}

impl<'a> OptionalValue<'a> for () {
    type Value = ();

    fn decode_value(
        cdc: &dyn Codec,
        data: &'a [u8],
        memory_manager: &'a MemoryManager,
    ) -> Result<Self::Value, DecodeError> {
        Ok(())
    }

    fn encode_value<'b>(
        cdc: &dyn Codec,
        value: &Self::Value,
        writer_factory: &'b dyn WriterFactory,
    ) -> Result<Option<&'b [u8]>, EncodeError> {
        Ok(None)
    }
}

impl<'a, V: SchemaValue<'a>> OptionalValue<'a> for V {
    type Value = V;

    fn decode_value(
        cdc: &dyn Codec,
        data: &'a [u8],
        memory_manager: &'a MemoryManager,
    ) -> Result<Self::Value, DecodeError> {
        unsafe { decode_value(cdc, data, memory_manager) }
    }

    fn encode_value<'b>(
        cdc: &dyn Codec,
        value: &Self::Value,
        writer_factory: &'b dyn WriterFactory,
    ) -> Result<Option<&'b [u8]>, EncodeError> {
        Ok(Some(cdc.encode_value(value, writer_factory)?))
    }
}

impl ListElementValue<'_> for u16 {}
impl ListElementValue<'_> for u32 {}
impl ListElementValue<'_> for u64 {}
impl ListElementValue<'_> for u128 {}
impl ListElementValue<'_> for i8 {}
impl ListElementValue<'_> for i16 {}
impl ListElementValue<'_> for i32 {}
impl ListElementValue<'_> for i64 {}
impl ListElementValue<'_> for i128 {}
impl ListElementValue<'_> for bool {}
impl<'a> ListElementValue<'a> for &'a str {}
#[cfg(feature = "std")]
impl ListElementValue<'_> for alloc::string::String {}
impl<'a> ListElementValue<'a> for &'a [u8] {}
#[cfg(feature = "std")]
impl ListElementValue<'_> for alloc::vec::Vec<u8> {}
impl ListElementValue<'_> for simple_time::Time {}
impl ListElementValue<'_> for simple_time::Duration {}
