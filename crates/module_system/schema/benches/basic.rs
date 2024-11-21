//! Basic benchmarks for the schema crate.

/// Run benchmarks.
fn main() {
    // Run registered benchmarks.
    divan::main();
}

/// Native binary benchmarks with borrowed strings and slices.
mod schema_binary_borrowed {
    use ixc_schema::binary::NativeBinaryCodec;
    use ixc_schema::codec::{decode_value, Codec};
    use ixc_schema::mem::MemoryManager;
    use ixc_schema_macros::SchemaValue;

    #[derive(SchemaValue, Default, PartialEq, Eq, Debug)]
    #[sealed]
    struct MsgSend<'a> {
        from: &'a str,
        to: &'a str,
        amount: &'a [Coin<'a>],
    }
    #[derive(SchemaValue, Default, PartialEq, Eq, Debug)]
    #[sealed]
    struct Coin<'b> {
        denom: &'b str,
        amount: u128,
    }

    fn msg1() -> MsgSend<'static> {
        MsgSend {
            from: "foo",
            to: "bar",
            amount: &[
                Coin {
                    denom: "uatom",
                    amount: 1234567890,
                },
                Coin {
                    denom: "foo",
                    amount: 9876543210,
                },
            ],
        }
    }
    #[divan::bench]
    fn encode() {
        let msg = msg1();
        let mem = MemoryManager::new();
        divan::black_box(NativeBinaryCodec.encode_value(&msg, &mem).unwrap());
    }

    #[divan::bench]
    fn decode(bencher: divan::Bencher) {
        let msg = msg1();
        let mem = MemoryManager::new();
        let bz = NativeBinaryCodec.encode_value(&msg, &mem).unwrap();
        bencher.bench(|| {
            let mem2 = MemoryManager::new();
            let msg2 = decode_value::<MsgSend>(&NativeBinaryCodec, bz, &mem2).unwrap();
            assert_eq!(msg, msg2);
        });
    }
}

/// Native binary benchmarks with owned strings and slices.
mod schema_binary_owned {
    extern crate alloc;

    use alloc::string::String;
    use alloc::vec::Vec;
    use ixc_schema::binary::NativeBinaryCodec;
    use ixc_schema::codec::{decode_value, Codec};
    use ixc_schema::mem::MemoryManager;
    use ixc_schema_macros::SchemaValue;

    #[derive(SchemaValue, Default, PartialEq, Eq, Debug)]
    #[sealed]
    struct MsgSend {
        from: String,
        to: String,
        amount: Vec<Coin>,
    }
    #[derive(SchemaValue, Default, PartialEq, Eq, Debug)]
    #[sealed]
    struct Coin {
        denom: String,
        amount: u128,
    }

    fn msg1() -> MsgSend {
        MsgSend {
            from: "foo".to_string(),
            to: "bar".to_string(),
            amount: vec![
                Coin {
                    denom: "uatom".to_string(),
                    amount: 1234567890,
                },
                Coin {
                    denom: "foo".to_string(),
                    amount: 9876543210,
                },
            ],
        }
    }

    #[divan::bench]
    fn encode() {
        let msg = msg1();
        let mem = MemoryManager::new();
        divan::black_box(NativeBinaryCodec.encode_value(&msg, &mem).unwrap());
    }

    #[divan::bench]
    fn decode(bencher: divan::Bencher) {
        let msg = msg1();
        let mem = MemoryManager::new();
        let bz = NativeBinaryCodec.encode_value(&msg, &mem).unwrap();
        bencher.bench(|| {
            let mem2 = MemoryManager::new();
            let msg2 = decode_value::<MsgSend>(&NativeBinaryCodec, bz, &mem2).unwrap();
            assert_eq!(msg, msg2);
        });
    }
}

/// Prost benchmarks.
pub mod prost_bench {
    extern crate alloc;
    use alloc::string::String;
    use alloc::vec::Vec;
    use prost::Message;

    #[derive(Message, PartialEq, Eq)]
    struct Coin {
        #[prost(string, tag = "1")]
        denom: String,
        #[prost(string, tag = "2")]
        amount: String,
    }

    #[derive(Message, PartialEq, Eq)]
    struct MsgSend {
        #[prost(string, tag = "1")]
        from: String,
        #[prost(string, tag = "2")]
        to: String,
        #[prost(message, repeated, tag = "3")]
        amount: Vec<Coin>,
    }

    fn msg1() -> MsgSend {
        MsgSend {
            from: "foo".to_string(),
            to: "bar".to_string(),
            amount: vec![
                Coin {
                    denom: "uatom".to_string(),
                    amount: "1234567890".to_string(),
                },
                Coin {
                    denom: "foo".to_string(),
                    amount: "9876543210".to_string(),
                },
            ],
        }
    }

    #[divan::bench]
    fn encode() {
        let msg = msg1();
        let mut buf = Vec::new();
        msg.encode(&mut buf).unwrap();
        divan::black_box(buf);
    }

    #[divan::bench]
    fn decode(bencher: divan::Bencher) {
        let msg = msg1();
        let mut buf = Vec::new();
        msg.encode(&mut buf).unwrap();
        bencher.bench(|| {
            let msg2 = MsgSend::decode(buf.as_slice()).unwrap();
            assert_eq!(msg, msg2);
        });
    }
}
