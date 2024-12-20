//! Message request and response types.

use crate::code::{ErrorCode, SystemCode};
use crate::AccountID;

/// A message.
#[non_exhaustive]
#[repr(C)]
pub struct Message<'a> {
    /// The target account of the message.
    target_account: AccountID,
    /// The request.
    request: Request<'a>,
}

/// A message selector code.
pub type MessageSelector = u64;

/// A request.
#[non_exhaustive]
#[repr(C)]
pub struct Request<'a> {
    /// The message selector.
    message_selector: MessageSelector,
    /// The inputs to the message.
    /// There can be up to three inputs.
    inputs: [Param<'a>; 3],
}

/// A message response.
#[non_exhaustive]
#[derive(Default)]
#[repr(C)]
pub struct Response<'a> {
    /// The outputs of the message.
    /// There can be up to two outputs.
    outputs: [Param<'a>; 2],
}

/// A message response.
#[derive(Default)]
#[non_exhaustive]
#[repr(C, u8)]
pub enum Param<'a> {
    /// An empty response.
    #[default]
    Empty,
    /// A slice parameter.
    Slice(&'a [u8]),
    /// A String parameter.
    String(&'a str),
    /// A u128 parameter.
    U128(u128),
    /// A u64 parameter.
    U64(u64),
    /// An account ID parameter.
    AccountID(AccountID),
}

impl<'a> Message<'a> {
    /// Create a new message.
    pub fn new(target_account: AccountID, request: Request<'a>) -> Self {
        Self {
            target_account,
            request,
        }
    }

    /// Get the target account of the message.
    pub fn target_account(&self) -> AccountID {
        self.target_account
    }

    /// Get the message request.
    pub fn request(&self) -> &Request<'a> {
        &self.request
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

    /// Get the message selector.
    pub fn message_selector(&self) -> MessageSelector {
        self.message_selector
    }

    /// Get the first input parameter.
    pub fn in1(&self) -> &Param<'a> {
        &self.inputs[0]
    }

    /// Get the second input parameter.
    pub fn in2(&self) -> &Param<'a> {
        &self.inputs[1]
    }

    /// Get the third input parameter.
    pub fn in3(&self) -> &Param<'a> {
        &self.inputs[2]
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
            outputs: [out1, Param::Empty],
        }
    }

    /// Create a new response with two outputs.
    pub fn new2(out1: Param<'a>, out2: Param<'a>) -> Self {
        Self {
            outputs: [out1, out2],
        }
    }

    /// Get the first output parameter.
    pub fn out1(&self) -> &Param<'a> {
        &self.outputs[0]
    }

    /// Get the second output parameter.
    pub fn out2(&self) -> &Param<'a> {
        &self.outputs[1]
    }
}

impl<'a> Param<'a> {
    /// Expect the parameter to be a slice or return an encoding error.
    pub fn expect_bytes(&self) -> Result<&'a [u8], ErrorCode> {
        match self {
            Param::Slice(slice) => Ok(slice),
            _ => Err(ErrorCode::SystemCode(SystemCode::EncodingError)),
        }
    }

    /// Expect the parameter to be a string or return an encoding error.
    pub fn expect_string(&self) -> Result<&'a str, ErrorCode> {
        match self {
            Param::String(string) => Ok(string),
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

    /// Expect the parameter to be a u64 or return an encoding error.
    pub fn expect_u64(&self) -> Result<u64, ErrorCode> {
        match self {
            Param::U64(u64) => Ok(*u64),
            _ => Err(ErrorCode::SystemCode(SystemCode::EncodingError)),
        }
    }

    /// Expect the parameter to be an account ID or return an encoding error.
    pub fn expect_account_id(&self) -> Result<AccountID, ErrorCode> {
        match self {
            Param::AccountID(account_id) => Ok(*account_id),
            _ => Err(ErrorCode::SystemCode(SystemCode::EncodingError)),
        }
    }
}

impl<'a> From<&'a [u8]> for Param<'a> {
    fn from(slice: &'a [u8]) -> Self {
        Param::Slice(slice)
    }
}

impl<'a> From<&'a str> for Param<'a> {
    fn from(string: &'a str) -> Self {
        Param::String(string)
    }
}

impl From<u128> for Param<'_> {
    fn from(u128: u128) -> Self {
        Param::U128(u128)
    }
}

impl From<u64> for Param<'_> {
    fn from(u64: u64) -> Self {
        Param::U64(u64)
    }
}

impl From<AccountID> for Param<'_> {
    fn from(account_id: AccountID) -> Self {
        Param::AccountID(account_id)
    }
}
