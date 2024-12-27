//! JSON encoding and decoding.
//! Encoding is intended to be performant and without unnecessary intermediate allocations.
//! Decoding is not performance-optimized and does a bunch of allocation internally.
//! [`decode_value`] is only available if the `json_decode` feature is enabled
//! and brings in a dependency on `serde_json`.
mod encoder;
mod escape;
#[cfg(feature = "json_decode")]
mod decoder;

pub use encoder::encode_value;

#[cfg(feature = "json_decode")]
pub use decoder::decode_value;

#[cfg(test)]
mod tests {
    use allocator_api2::vec;
    use crate::json::decoder::decode_value;
    use crate::json::encoder::encode_value;
    use crate::testdata::Prims;
    use crate::testdata::{ABitOfEverything, TestEnum};
    use proptest::proptest;

    extern crate std;

    proptest! {
        #[test]
        fn test_roundtrip(value: ABitOfEverything) {
            let mut writer = vec![];
            encode_value(&value, &mut writer).unwrap();
            let res = std::str::from_utf8(&writer).unwrap();
            let decoded = decode_value::<ABitOfEverything>(&res, &Default::default()).unwrap();
            assert_eq!(value, decoded);
        }
    }
}
