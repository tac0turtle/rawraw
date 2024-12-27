//! JSON encoding and decoding.
mod decoder;
mod encoder;
mod escape;

pub use decoder::decode_value;
pub use encoder::encode_value;

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
