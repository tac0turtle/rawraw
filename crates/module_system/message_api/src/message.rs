//! Message request and response types.

use crate::code::{ErrorCode, SystemCode};
use crate::AccountID;

/// A message.
#[non_exhaustive]
pub struct Message<'a> {
    /// The target account of the message.
    pub target_account: AccountID,
    /// The request.
    pub request: Request<'a>,
}

/// A message selector code.
pub type MessageSelector = u64;

/// A request.
#[non_exhaustive]
pub struct Request<'a> {
    /// The message selector.
    pub message_selector: MessageSelector,
    /// The inputs to the message.
    /// There can be up to three inputs.
    pub inputs: [Param<'a>; 3],
}

/// A message response.
#[non_exhaustive]
#[derive(Default)]
pub struct Response<'a> {
    /// The outputs of the message.
    /// There can be up to two outputs.
    pub inputs: [Param<'a>; 2],
}

/// A message response.
#[derive(Default)]
#[non_exhaustive]
pub enum Param<'a> {
    /// An empty response.
    #[default]
    Empty,
    /// A slice output parameter.
    Slice(&'a [u8]),
    /// A u128 output parameter.
    U128(u128),
}

impl<'a> Message<'a> {
    /// Create a new message.
    pub fn new(target_account: AccountID, request: Request<'a>) -> Self {
        Self {
            target_account,
            request,
        }
    }
}

impl<'a> Request<'a> {
    /// Create a new request with no inputs.
    pub fn new(message_selector: MessageSelector) -> Self {
        Self {
            message_selector,
            inputs: Default::default(),
        }
    }

    /// Create a new request with one input.
    pub fn new1(message_selector: MessageSelector, in1: Param<'a>) -> Self {
        Self {
            message_selector,
            inputs: [in1, Param::Empty, Param::Empty],
        }
    }

    /// Create a new request with two inputs.
    pub fn new2(message_selector: MessageSelector, in1: Param<'a>, in2: Param<'a>) -> Self {
        Self {
            message_selector,
            inputs: [in1, in2, Param::Empty],
        }
    }

    /// Create a new request with three inputs.
    pub fn new3(
        message_selector: MessageSelector,
        in1: Param<'a>,
        in2: Param<'a>,
        in3: Param<'a>,
    ) -> Self {
        Self {
            message_selector,
            inputs: [in1, in2, in3],
        }
    }
}

impl<'a> Response<'a> {
    /// Create a new response with no outputs.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new response with one output.
    pub fn new1(out1: Param<'a>) -> Self {
        Self {
            inputs: [out1, Param::Empty],
        }
    }

    /// Create a new response with two outputs.
    pub fn new2(out1: Param<'a>, out2: Param<'a>) -> Self {
        Self {
            inputs: [out1, out2],
        }
    }
}

impl<'a> Param<'a> {
    /// Expect the parameter to be a slice or return an encoding error.
    pub fn expect_slice(&self) -> Result<&'a [u8], ErrorCode> {
        match self {
            Param::Slice(slice) => Ok(slice),
            _ => Err(ErrorCode::SystemCode(SystemCode::EncodingError)),
        }
    }

    /// Expect the parameter to be a u128 or return an encoding error.
    pub fn expect_u128(&self) -> Result<u128, ErrorCode> {
        match self {
            Param::U128(u128) => Ok(*u128),
            _ => Err(ErrorCode::SystemCode(SystemCode::EncodingError)),
        }
    }
}

impl<'a> From<&'a [u8]> for Param<'a> {
    fn from(slice: &'a [u8]) -> Self {
        Param::Slice(slice)
    }
}

impl From<u128> for Param<'_> {
    fn from(u128: u128) -> Self {
        Param::U128(u128)
    }
}
