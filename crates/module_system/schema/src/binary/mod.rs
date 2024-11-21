//! Defines a codec for the native binary format.

use crate::value::ValueCodec;
use crate::binary::decoder::decode_value;
use crate::binary::encoder::encode_value;
use crate::buffer::WriterFactory;
use crate::codec::{Codec};
use crate::decoder::DecodeError;
use crate::encoder::EncodeError;
use crate::mem::MemoryManager;

pub(crate) mod decoder;
pub(crate) mod encoder;

/// A codec for encoding and decoding values using the native binary format.
#[derive(Default)]
pub struct NativeBinaryCodec;

impl Codec for NativeBinaryCodec {
    fn encode_value<'a>(
        &self,
        value: &dyn ValueCodec,
        writer_factory: &'a dyn WriterFactory,
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
    extern crate std;

    use crate::codec::{decode_value, Codec};
    use crate::mem::MemoryManager;
    use alloc::string::String;
    use alloc::vec;
    use alloc::vec::Vec;
    use ixc_schema_macros::SchemaValue;
    use proptest::prelude::*;
    use proptest_derive::Arbitrary;
    use crate::binary::encoder::encode_value;
    use crate::binary::NativeBinaryCodec;

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

    #[test]
    fn test_u32_decode() {
        let buf: [u8; 4] = [10, 0, 0, 0];
        let mem = MemoryManager::new();
        let x = crate::codec::decode_value::<u32>(&NativeBinaryCodec, &buf, &mem).unwrap();
        assert_eq!(x, 10);
    }

    #[test]
    fn test_decode_borrowed_string() {
        let str = "hello";
        let mem = MemoryManager::new();
        let x =
            crate::codec::decode_value::<&str>(&NativeBinaryCodec, str.as_bytes(), &mem).unwrap();
        assert_eq!(x, "hello");
    }

    #[test]
    fn test_decode_owned_string() {
        let str = "hello";
        let mem = MemoryManager::new();
        let x = crate::codec::decode_value::<alloc::string::String>(
            &NativeBinaryCodec,
            str.as_bytes(),
            &mem,
        )
            .unwrap();
        assert_eq!(x, "hello");
    }

    #[derive(Debug, PartialEq, Eq, Default, SchemaValue)]
    #[sealed]
    struct Coin<'b> {
        denom: &'b str,
        amount: u128,
    }

    impl Drop for Coin<'_> {
        fn drop(&mut self) {
            std::println!("drop Coin");
        }
    }

    #[test]
    fn test_coin() {
        let coin = Coin {
            denom: "uatom",
            amount: 1234567890,
        };
        let mem = MemoryManager::new();
        let res = encode_value(&coin, &mem).unwrap();
        let decoded = crate::codec::decode_value::<Coin>(&NativeBinaryCodec, res, &mem).unwrap();
        assert_eq!(decoded, coin);
    }

    #[test]
    fn test_coins() {
        let coins = vec![
            Coin {
                denom: "uatom",
                amount: 1234567890,
            },
            Coin {
                denom: "foo",
                amount: 9876543210,
            },
        ];
        let mem = MemoryManager::new();
        let res = encode_value(&coins, &mem).unwrap();
        let decoded = crate::codec::decode_value::<&[Coin]>(&NativeBinaryCodec, res, &mem).unwrap();
        assert_eq!(decoded, coins);
    }

    #[derive(SchemaValue, Default, PartialEq, Eq, Debug)]
    #[sealed]
    struct MsgSend<'a> {
        from: &'a str,
        to: &'a str,
        amount: &'a [Coin<'a>],
    }
}
