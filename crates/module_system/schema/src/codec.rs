//! The codec trait.

use crate::decoder::DecodeError;
use crate::encoder::EncodeError;
use crate::encoding::Encoding;
use crate::mem::MemoryManager;
use crate::value::SchemaValue;
use crate::value::ValueCodec;
use allocator_api2::alloc::Allocator;

/// Trait implemented by encoding protocols.
pub trait Codec {
    /// Encode a value.
    fn encode_value<'a>(
        &self,
        value: &dyn ValueCodec,
        writer_factory: &'a dyn Allocator,
    ) -> Result<&'a [u8], EncodeError>;

    /// Decode a value.
    fn decode_value<'a>(
        &self,
        input: &'a [u8],
        memory_manager: &'a MemoryManager,
        visitor: &mut dyn ValueCodec<'a>,
    ) -> Result<(), DecodeError>;
}

/// A codec with a well-known encoding.
pub trait WellKnownCodec: Codec {
    /// Get the well-known encoding of the codec.
    const ENCODING: Encoding;
}

/// Decode a value.
pub fn decode_value<'a, V: SchemaValue<'a>>(
    codec: &dyn Codec,
    input: &'a [u8],
    mem: &'a MemoryManager,
) -> Result<V, DecodeError> {
    let mut visitor = V::default();
    codec.decode_value(input, mem, &mut visitor)?;
    Ok(visitor)
}
