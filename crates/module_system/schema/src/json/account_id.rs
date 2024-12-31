//! Account ID JSON string encoding and decoding.
use core::fmt::Write;
use ixc_message_api::AccountID;
use crate::decoder::DecodeError;
use crate::encoder::EncodeError;

/// A codec for encoding and decoding account IDs as strings.
/// If address string rendering is desired, then this codec should be implemented
/// to do a stateful conversion between an account ID and an address and its corresponding string
/// representation.
pub trait AccountIDStringCodec {
    /// Encode an account ID as a string.
    fn encode_str(&self, account_id: &AccountID, writer: &mut dyn Write) -> Result<(), EncodeError>;
    /// Decode a string as an account ID.
    fn decode_str(&self, s: &str) -> Result<AccountID, DecodeError>;
}

/// The default account ID string codec which encodes account IDs as hex strings.
pub struct DefaultAccountIDStringCodec;

impl AccountIDStringCodec for DefaultAccountIDStringCodec {
    fn encode_str(&self, account_id: &AccountID, writer: &mut dyn Write) -> Result<(), EncodeError> {
        let id: u128 = (*account_id).into();
        write!(writer, "0x{:x}", id)
            .map_err(|_| EncodeError::UnknownError)
    }

    fn decode_str(&self, s: &str) -> Result<AccountID, DecodeError> {
        let s = s.strip_prefix("0x")
            .ok_or(DecodeError::InvalidData)?;
        let s = u128::from_str_radix(s, 16)
            .map_err(|_| DecodeError::InvalidData)?;
        Ok(AccountID::new(s))
    }
}