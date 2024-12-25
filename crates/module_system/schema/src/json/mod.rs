mod decoder;
mod encoder;

#[cfg(test)]
mod tests {
    use proptest::proptest;
    use crate::json::decoder::decode_value;
    use crate::json::encoder::encode_value;
    use crate::testdata::{ABitOfEverything, TestEnum};
    use crate::testdata::Prims;

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
