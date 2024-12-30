//! The AnyMessage type which packs a struct within the scope of an account into a message.
use ixc::structs::StructSchema;
use ixc_message_api::AccountID;
use crate::decoder::{DecodeError, Decoder};
use crate::encoder::{EncodeError, Encoder};
use crate::kind::Kind;
use crate::list::List;
use crate::SchemaValue;
use crate::types::{ListElementType, Type};
use crate::value::{ListElementValue, ValueCodec};

/// A message (any struct type) within the scope of an account.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct AnyMessage<'a> {
    /// The account within which the message is scoped.
    pub account: AccountID,
    /// The type selector of the struct.
    pub selector: u64,
    /// The struct encoded as bytes using the native binary encoding.
    pub bytes: List<'a, u8>,
}

impl <'a> AnyMessage<'a> {
    /// Create a new AnyMessage.
    pub fn new(account: AccountID, selector: u64, bytes: List<'a, u8>) -> Self {
        Self {
            account,
            selector,
            bytes,
        }
    }

    /// Decode the message if it matches the given struct type.
    pub fn decode_message<M: StructSchema + ValueCodec<'a>>(&self) -> Result<Option<M>, DecodeError> {
        if self.selector != M::TYPE_SELECTOR {
            return Ok(None);
        }
        todo!()
    }
}

impl <'a> ValueCodec<'a> for AnyMessage<'a> {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_any_message()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_any_message(self)
    }
}

impl <'a> SchemaValue<'a> for AnyMessage<'a> {
    type Type = AnyMessageT;
}

/// The type of the AnyMessage type.
pub struct AnyMessageT;

impl <'a> Type for AnyMessageT {
    const KIND: Kind = Kind::AnyMessage;
}

impl <'a> ListElementType for AnyMessageT {}
impl <'a> ListElementValue<'a> for AnyMessage<'a> {}


