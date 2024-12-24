mod encoder;
mod decoder;

#[cfg(test)]
mod tests {
    use alloc::vec;
    use crate::json::encoder::encode_value;

    #[test]
    fn test_encode() {
        let mut writer = vec![];
        encode_value(&10, &mut writer).unwrap();
        assert_eq!(writer, b"10");
    }
}