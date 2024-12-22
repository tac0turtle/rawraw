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
    // this struct is packed in this order so that we have the largest and most aligned items first
    // and then smaller and less aligned items last
    inputs_values: [ParamValue<'a>; 3], // size 16 * 3, aligned to 16 bytes
    message_selector: MessageSelector,  // size 8, aligned to 8 bytes
    inputs_types: [ParamType; 3],       // size 3, aligned to 1 byte
}

/// A message response.
#[non_exhaustive]
#[derive(Default)]
#[repr(C)]
pub struct Response<'a> {
    outputs_values: [ParamValue<'a>; 2], // size 16 * 2, aligned to 16 bytes
    outputs_types: [ParamType; 2],       // size 2, aligned to 1 byte
}

/// A message response.
#[derive(Default)]
pub struct Param<'a> {
    value: ParamValue<'a>,
    typ: ParamType,
}

#[derive(Default, Clone, Copy)]
#[non_exhaustive]
#[repr(u8)]
enum ParamType {
    /// An empty response.
    #[default]
    Empty,
    /// A slice parameter.
    Slice,
    /// A String parameter.
    String,
    /// A u128 parameter.
    U128,
    /// An account ID parameter.
    AccountID,
}

#[derive(Clone, Copy)]
union ParamValue<'a> {
    empty: (),
    slice: &'a [u8],
    string: &'a str,
    u128: u128,
    account_id: AccountID,
}

impl<'a> Default for ParamValue<'a> {
    fn default() -> Self {
        Self { empty: () }
    }
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
            inputs_values: Default::default(),
            inputs_types: Default::default(),
        }
    }

    /// Create a new request with one input.
    pub fn new1(message_selector: MessageSelector, in1: Param<'a>) -> Self {
        Self {
            message_selector,
            inputs_values: [in1.value, ParamValue::default(), ParamValue::default()],
            inputs_types: [in1.typ, ParamType::Empty, ParamType::Empty],
        }
    }

    /// Create a new request with two inputs.
    pub fn new2(message_selector: MessageSelector, in1: Param<'a>, in2: Param<'a>) -> Self {
        Self {
            message_selector,
            inputs_values: [in1.value, in2.value, ParamValue::default()],
            inputs_types: [in1.typ, in2.typ, ParamType::Empty],
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
            inputs_values: [in1.value, in2.value, in3.value],
            inputs_types: [in1.typ, in2.typ, in3.typ],
        }
    }

    /// Get the message selector.
    pub fn message_selector(&self) -> MessageSelector {
        self.message_selector
    }

    /// Get the first input parameter.
    pub fn in1(&self) -> Param<'a> {
        Param {
            typ: self.inputs_types[0],
            value: self.inputs_values[0],
        }
    }

    /// Get the second input parameter.
    pub fn in2(&self) -> Param<'a> {
        Param {
            typ: self.inputs_types[1],
            value: self.inputs_values[1],
        }
    }

    /// Get the third input parameter.
    pub fn in3(&self) -> Param<'a> {
        Param {
            typ: self.inputs_types[2],
            value: self.inputs_values[2],
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
            outputs_values: [out1.value, ParamValue::default()],
            outputs_types: [out1.typ, ParamType::Empty],
        }
    }

    /// Create a new response with two outputs.
    pub fn new2(out1: Param<'a>, out2: Param<'a>) -> Self {
        Self {
            outputs_values: [out1.value, out2.value],
            outputs_types: [out1.typ, out2.typ],
        }
    }

    /// Get the first output parameter.
    pub fn out1(&self) -> Param<'a> {
        Param {
            typ: self.outputs_types[0],
            value: self.outputs_values[0],
        }
    }

    /// Get the second output parameter.
    pub fn out2(&self) -> Param<'a> {
        Param {
            typ: self.outputs_types[1],
            value: self.outputs_values[1],
        }
    }
}

impl<'a> Param<'a> {
    /// Expect the parameter to be a slice or return an encoding error.
    pub fn expect_bytes(&self) -> Result<&'a [u8], ErrorCode> {
        match self.typ {
            ParamType::Slice => unsafe { Ok(self.value.slice) },
            _ => Err(ErrorCode::SystemCode(SystemCode::EncodingError)),
        }
    }

    /// Expect the parameter to be a string or return an encoding error.
    pub fn expect_string(&self) -> Result<&'a str, ErrorCode> {
        match self.typ {
            ParamType::String => unsafe { Ok(self.value.string) },
            _ => Err(ErrorCode::SystemCode(SystemCode::EncodingError)),
        }
    }

    /// Expect the parameter to be a u128 or return an encoding error.
    pub fn expect_u128(&self) -> Result<u128, ErrorCode> {
        match self.typ {
            ParamType::U128 => unsafe { Ok(self.value.u128) },
            _ => Err(ErrorCode::SystemCode(SystemCode::EncodingError)),
        }
    }

    /// Expect the parameter to be an account ID or return an encoding error.
    pub fn expect_account_id(&self) -> Result<AccountID, ErrorCode> {
        match self.typ {
            ParamType::AccountID => unsafe { Ok(self.value.account_id) },
            _ => Err(ErrorCode::SystemCode(SystemCode::EncodingError)),
        }
    }
}

impl<'a> From<&'a [u8]> for Param<'a> {
    fn from(slice: &'a [u8]) -> Self {
        Param {
            typ: ParamType::Slice,
            value: ParamValue { slice },
        }
    }
}

impl<'a> From<&'a str> for Param<'a> {
    fn from(string: &'a str) -> Self {
        Param {
            typ: ParamType::String,
            value: ParamValue { string },
        }
    }
}

impl<'a> From<u128> for Param<'a> {
    fn from(u128: u128) -> Self {
        Param {
            typ: ParamType::U128,
            value: ParamValue { u128 },
        }
    }
}

impl From<AccountID> for Param<'_> {
    fn from(account_id: AccountID) -> Self {
        Param {
            typ: ParamType::AccountID,
            value: ParamValue { account_id },
        }
    }
}
