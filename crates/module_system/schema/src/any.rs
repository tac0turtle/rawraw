//! The AnyMessage type which packs a struct within the scope of an account into a message.

use crate::decoder::{DecodeError, Decoder};
use crate::encoder::{EncodeError, Encoder};
use crate::kind::Kind;
use crate::list::List;
use crate::mem::MemoryManager;
use crate::structs::StructSchema;
use crate::types::{ListElementType, Type};
use crate::value::{ListElementValue, ValueCodec};
use crate::{binary, SchemaValue};
use ixc_message_api::AccountID;

/// A message (any struct type) within the scope of an account.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum AnyMessage<'a> {
    #[default]
    Empty,
    Message {
        account: AccountID,
        selector: u64,
        bytes: List<'a, u8>,
    },
    CreateAccount {
        handler_id: &'a str,
        init_data: List<'a, u8>,
    },
    Migrate {
        account: AccountID,
        new_handler_id: &'a str,
    },
}

impl<'a> AnyMessage<'a> {
    /// Create a new AnyMessage.
    pub fn new_message(account: AccountID, selector: u64, bytes: List<'a, u8>) -> Self {
        Self {
            account,
            selector,
            bytes,
        }
    }

    /// Decode the message if it matches the given struct type.
    pub fn decode_message<M: StructSchema + SchemaValue<'a>>(
        &'a self,
        mem: &'a MemoryManager,
    ) -> Result<Option<M>, DecodeError> {
        // if self.selector != M::TYPE_SELECTOR {
        //     return Ok(None);
        // }
        // let mut res = M::default();
        // binary::decoder::decode_value(self.bytes.as_slice(), mem, &mut res)
        //     .map_err(|_| DecodeError::InvalidData)?;
        // Ok(Some(res))
        todo!()
    }
}

impl<'a> ValueCodec<'a> for AnyMessage<'a> {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        *self = decoder.decode_any_message()?;
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        encoder.encode_any_message(self)
    }
}

impl<'a> SchemaValue<'a> for AnyMessage<'a> {
    type Type = AnyMessageT;
}

/// The type of the AnyMessage type.
pub struct AnyMessageT;

impl<'a> Type for AnyMessageT {
    const KIND: Kind = Kind::AnyMessage;
}

impl<'a> ListElementType for AnyMessageT {}
impl<'a> ListElementValue<'a> for AnyMessage<'a> {}
