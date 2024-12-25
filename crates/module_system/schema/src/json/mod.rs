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

    #[test]
    fn test_encode() {
        let mut value = ABitOfEverything::default();
        value.e = TestEnum::B;

        let s = encode_value(&value).unwrap();
        std::println!("{}", s);
        let res = decode_value::<ABitOfEverything>(&s, &Default::default()).unwrap();
        assert_eq!(res, value);
    }

    proptest! {
        #[test]
        fn test_roundtrip(value: ABitOfEverything) {
            let res = encode_value(&value).unwrap();
            let decoded = decode_value::<ABitOfEverything>(&res, &Default::default()).unwrap();
            assert_eq!(value, decoded);
        }
    }
}
