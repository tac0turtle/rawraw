//! Defines a codec for the native binary format.

use crate::binary::decoder::decode_value;
use crate::binary::encoder::encode_value;
use crate::codec::{Codec, WellKnownCodec};
use crate::decoder::DecodeError;
use crate::encoder::EncodeError;
use crate::mem::MemoryManager;
use crate::value::ValueCodec;
use allocator_api2::alloc::Allocator;
use crate::encoding::Encoding;

pub(crate) mod decoder;
pub(crate) mod encoder;

/// A codec for encoding and decoding values using the native binary format.
#[derive(Default)]
pub struct NativeBinaryCodec;

impl Codec for NativeBinaryCodec {
    fn encode_value<'a>(
        &self,
        value: &dyn ValueCodec,
        writer_factory: &'a dyn Allocator,
    ) -> Result<&'a [u8], EncodeError> {
        encode_value(value, writer_factory)
    }

    fn decode_value<'a>(
        &self,
        input: &'a [u8],
        memory_manager: &'a MemoryManager,
        visitor: &mut dyn ValueCodec<'a>,
    ) -> Result<(), DecodeError> {
        decode_value(input, memory_manager, visitor)
    }
}

impl WellKnownCodec for NativeBinaryCodec {
    const ENCODING: Encoding = Encoding::NativeBinary;
}

#[cfg(test)]
mod tests {
    use crate::codec::{decode_value, Codec};
    use crate::mem::MemoryManager;
    use alloc::string::String;
    use alloc::vec::Vec;
    use ixc_schema_macros::SchemaValue;
    use proptest::prelude::*;
    use proptest_derive::Arbitrary;
    use crate::testdata::ABitOfEverything;

    proptest! {
        #[test]
        fn test_roundtrip(value: ABitOfEverything) {
            let cdc = super::NativeBinaryCodec;
            let mem = MemoryManager::new();
            let bz = cdc.encode_value(&value, &mem).unwrap();
            let value2 = decode_value(&cdc, bz, &mem).unwrap();
            assert_eq!(value, value2);
        }
    }
}
