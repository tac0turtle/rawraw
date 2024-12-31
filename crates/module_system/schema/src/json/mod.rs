//! JSON encoding and decoding.
//! Encoding is intended to be performant and without unnecessary intermediate allocations.
//! Decoding is not performance-optimized and does a bunch of allocation internally.
//! [`decode_value`] is only available if the `json_decode` feature is enabled
//! and brings in a dependency on `serde_json`.
#[cfg(feature = "json_decode")]
mod decoder;
mod encoder;
mod escape;
mod account_id;
pub use account_id::AccountIDStringCodec;

/// A codec for encoding and decoding values using the JSON format.
#[derive(Clone)]
pub struct JSONCodec<'a> {
    account_id_codec: &'a dyn AccountIDStringCodec,
}

impl<'a> JSONCodec<'a> {
    /// Create a new JSON codec with the provided account ID codec.
    pub fn new(account_id_codec: &'a dyn AccountIDStringCodec) -> Self {
        Self {
            account_id_codec,
        }
    }
}

use core::fmt::Write;

#[cfg(test)]
mod tests {
    use crate::testdata::ABitOfEverything;
    use allocator_api2::vec;
    use proptest::proptest;
    use crate::json::account_id::DefaultAccountIDStringCodec;
    use crate::json::JSONCodec;

    extern crate std;

    proptest! {
        #[test]
        fn test_roundtrip(value: ABitOfEverything) {
            let mut writer = vec![];
            let codec = JSONCodec::new(&DefaultAccountIDStringCodec);
            codec.encode_value(&value, &mut writer).unwrap();
            let res = std::str::from_utf8(&writer).unwrap();
            let decoded = codec.decode_value::<ABitOfEverything>(res, &Default::default()).unwrap();
            assert_eq!(value, decoded);
        }
    }
}
