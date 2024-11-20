//! The codec trait.

use crate::value::ValueCodec;
use crate::buffer::WriterFactory;
use crate::decoder::DecodeError;
use crate::decoder::Decoder;
use crate::encoder::{EncodeError, Encoder};
use crate::mem::MemoryManager;
use crate::value::{SchemaValue};

/// Trait implemented by encoding protocols.
pub trait Codec {
    /// Encode a value.
    fn encode_value<'a>(
        &self,
        value: &dyn ValueCodec,
        writer_factory: &'a dyn WriterFactory,
    ) -> Result<&'a [u8], EncodeError>;

    /// Decode a value.
    fn decode_value<'a>(
        &self,
        input: &'a [u8],
        memory_manager: &'a MemoryManager,
        visitor: &mut dyn ValueCodec<'a>,
    ) -> Result<(), DecodeError>;
}

/// Decode a value.
pub fn decode_value<'a, V: SchemaValue<'a>>(
    codec: &dyn Codec,
    input: &'a [u8],
    mem: &'a MemoryManager,
) -> Result<V, DecodeError> {
    // struct Visitor<'b, U: SchemaValue<'b>>(U::DecodeState);
    // impl<'b, U: SchemaValue<'b>> ValueDecodeVisitor<'b> for Visitor<'b, U> {
    //     fn decode(&mut self, decoder: &mut dyn Decoder<'b>) -> Result<(), DecodeError> {
    //         U::visit_decode_state(&mut self.0, decoder)
    //     }
    // }
    // let mut visitor = Visitor::<V>(V::DecodeState::default());
    // codec.decode_value(input, mem, &mut visitor)?;
    // V::finish_decode_state(visitor.0, mem)
    todo!()
}
