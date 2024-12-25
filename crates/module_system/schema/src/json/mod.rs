mod decoder;
mod encoder;

#[cfg(test)]
mod tests {
    use crate::json::decoder::decode_value;
    use crate::json::encoder::encode_value;
    use crate::testdata::ABitOfEverything;
    use alloc::vec;
    use proptest::proptest;

    extern crate std;

    #[test]
    fn test_encode() {
        let mut writer = vec![];
        let value = ABitOfEverything::default();
        encode_value(&value, &mut writer).unwrap();
        let s = std::str::from_utf8(&writer).unwrap();
        std::println!("{}", s);
        let res = decode_value::<ABitOfEverything>(s, &Default::default()).unwrap();
        assert_eq!(res, value);
    }

    // proptest! {
    //     #[test]
    //     fn test_roundtrip(value: ABitOfEverything) {
    //         let mut writer = vec![];
    //         encode_value(&value, &mut writer).unwrap();
    //         let s = std::str::from_utf8(&writer).unwrap();
    //         let res = decode_value::<ABitOfEverything>(s, &Default::default()).unwrap();
    //         assert_eq!(res, value);
    //     }
    // }
}
