//! JSON encoding and decoding.
mod decoder;
mod encoder;
mod escape;

pub use decoder::decode_value;
pub use encoder::encode_value;

#[cfg(test)]
mod tests {
    use crate::json::decoder::decode_value;
    use crate::json::encoder::encode_value;
    use crate::testdata::Prims;
    use crate::testdata::{ABitOfEverything, TestEnum};
    use proptest::proptest;

    extern crate std;

    proptest! {
        #[test]
        fn test_roundtrip(value: ABitOfEverything) {
            let res = encode_value(&value).unwrap();
            let decoded = decode_value::<ABitOfEverything>(&res, &Default::default()).unwrap();
            assert_eq!(value, decoded);
        }
    }
}
