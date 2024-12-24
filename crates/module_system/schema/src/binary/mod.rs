//! Defines a codec for the native binary format.

use crate::binary::decoder::decode_value;
use crate::binary::encoder::encode_value;
use crate::codec::Codec;
use crate::decoder::DecodeError;
use crate::encoder::EncodeError;
use crate::mem::MemoryManager;
use crate::value::ValueCodec;
use allocator_api2::alloc::Allocator;

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

#[cfg(test)]
mod tests {
    use crate::codec::{decode_value, Codec};
    use crate::mem::MemoryManager;
    use alloc::string::String;
    use alloc::vec::Vec;
    use ixc_schema_macros::SchemaValue;
    use proptest::prelude::*;
    use proptest_derive::Arbitrary;

    #[derive(SchemaValue, Default, Debug, Eq, PartialEq, Arbitrary)]
    #[non_exhaustive]
    struct ABitOfEverything {
        primitives: Prims,
        s: String,
        // t: Time,
        // d: Duration,
        v: Vec<u8>,
        ls: Vec<String>,
        li: Vec<i32>,
        lp: Vec<Prims>,
        os: Option<String>,
        op: Option<Prims>,
        e: TestEnum,
    }

    #[derive(SchemaValue, Default, Debug, Eq, PartialEq, Arbitrary)]
    #[non_exhaustive]
    struct Prims {
        a_u8: u8,
        a_u16: u16,
        a_u32: u32,
        a_u64: u64,
        a_u128: u128,
        a_i8: i8,
        a_i16: i16,
        a_i32: i32,
        a_i64: i64,
        a_i128: i128,
        a_bool: bool,
    }

    #[derive(SchemaValue, Default, Debug, Eq, PartialEq, Arbitrary)]
    #[non_exhaustive]
    enum TestEnum {
        #[default]
        A,
        B = 10,
        C = 20,
        D,
    }

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
