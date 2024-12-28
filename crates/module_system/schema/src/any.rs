use ixc::structs::StructSchema;
use ixc_message_api::AccountID;
use ixc_message_api::message::MessageSelector;
use crate::decoder::{DecodeError, Decoder};
use crate::encoder::{EncodeError, Encoder};
use crate::kind::Kind;
use crate::list::List;
use crate::SchemaValue;
use crate::types::{ListElementType, Type};
use crate::value::{ListElementValue, ValueCodec};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct AnyMessage<'a> {
    pub target: AccountID,
    pub selector: MessageSelector,
    pub bytes: List<'a, u8>,
}

impl <'a> AnyMessage<'a> {
    pub fn new(target: AccountID, selector: MessageSelector, bytes: List<'a, u8>) -> Self {
        Self {
            target,
            selector,
            bytes,
        }
    }

    pub fn decode_message<M: StructSchema<'a> + ValueCodec<'a>>(&self) -> Result<Option<M>, DecodeError> {
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

pub struct AnyMessageT;

impl <'a> Type for AnyMessageT {
    const KIND: Kind = Kind::AnyMessage;
}

impl <'a> ListElementType for AnyMessageT {}
impl <'a> ListElementValue<'a> for AnyMessage<'a> {}


