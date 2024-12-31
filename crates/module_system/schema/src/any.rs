//! The AnyMessage type which packs a struct within the scope of an account into a message.

use crate::decoder::{DecodeError, Decoder};
use crate::encoder::{EncodeError, Encoder};
use crate::kind::Kind;
use crate::list::List;
use crate::types::{ListElementType, Type};
use crate::value::{ListElementValue, ValueCodec};
use crate::SchemaValue;
use ixc_message_api::AccountID;

/// A wrapper around an executable message that can be serialized and executed later.
/// This API is EXPERIMENTAL and will likely change in the future.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
#[non_exhaustive]
pub enum AnyMessage<'a> {
    /// The default value of AnyMessage.
    #[default]
    Empty,
    /// Execute a message within the scope of an account based on the provided selector and bytes.
    ExecMessage {
        /// The account the message is being sent to.
        account: AccountID,
        /// The type selector of the message's struct.
        selector: u64,
        /// The bytes of the message's struct.
        bytes: List<'a, u8>,
    },
    /// Create an account with the provided handler ID and initialization data.
    CreateAccount {
        /// The ID of the handler to create the account with.
        handler_id: &'a str,
        /// The initialization data for the account's handler.
        init_data: List<'a, u8>,
    },
    /// Migrate an account to a new handler with the provided handler ID.
    Migrate {
        /// The account to migrate.
        account: AccountID,
        /// The ID of the new handler to migrate to.
        new_handler_id: &'a str,
    },
}

impl<'a> AnyMessage<'a> {
    /// Create a new AnyMessage.
    pub fn new_exec_message(account: AccountID, selector: u64, bytes: List<'a, u8>) -> Self {
        Self::ExecMessage {
            account,
            selector,
            bytes,
        }
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

pub(crate) const EMPTY_PREFIX: u8 = 0;
pub(crate) const EXEC_MESSAGE_PREFIX: u8 = 1;
pub(crate) const CREATE_ACCOUNT_PREFIX: u8 = 2;
pub(crate) const MIGRATE_PREFIX: u8 = 3;