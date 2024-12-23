//! Message request and response types.

use crate::code::{ErrorCode, SystemCode};
use crate::AccountID;
use alloc::string::String;

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

    pub fn new1_g(message_selector: MessageSelector, in1: impl Into<Param<'a>>) -> Self {
        Self::new1(message_selector, in1.into())
    }

    pub fn new2_g(
        message_selector: MessageSelector,
        in1: impl Into<Param<'a>>,
        in2: impl Into<Param<'a>>,
    ) -> Self {
        Self::new2(message_selector, in1.into(), in2.into())
    }

    pub fn new3_g(
        message_selector: MessageSelector,
        in1: impl Into<Param<'a>>,
        in2: impl Into<Param<'a>>,
        in3: impl Into<Param<'a>>,
    ) -> Self {
        Self::new3(message_selector, in1.into(), in2.into(), in3.into())
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

    pub fn borrow_in1<'b, T>(&'b self) -> Result<Option<T>, ErrorCode>
    where
        'a: 'b,
        T: TryFrom<&'b Param<'a>, Error = ErrorCode>,
    {
        match &self.inputs[0] {
            Param::Empty => Ok(None),
            other => T::try_from(other).map(Some),
        }
    }

    pub fn borrow_in2<'b, T>(&'b self) -> Result<Option<T>, ErrorCode>
    where
        'a: 'b,
        T: TryFrom<&'b Param<'a>, Error = ErrorCode>,
    {
        match &self.inputs[1] {
            Param::Empty => Ok(None),
            other => T::try_from(other).map(Some),
        }
    }

    pub fn borrow_in3<'b, T>(&'b self) -> Result<Option<T>, ErrorCode>
    where
        'a: 'b,
        T: TryFrom<&'b Param<'a>, Error = ErrorCode>,
    {
        match &self.inputs[2] {
            Param::Empty => Ok(None),
            other => T::try_from(other).map(Some),
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

    /// Expect the parameter to be an account ID or return an encoding error.
    pub fn expect_account_id(&self) -> Result<AccountID, ErrorCode> {
        match self {
            Param::AccountID(account_id) => Ok(*account_id),
            _ => Err(ErrorCode::SystemCode(SystemCode::EncodingError)),
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Param::Empty)
    }
}

impl<'a> TryFrom<&'a Param<'a>> for &'a AccountID {
    type Error = ErrorCode;

    fn try_from(value: &'a Param<'a>) -> Result<Self, Self::Error> {
        match value {
            Param::AccountID(account_id) => Ok(account_id),
            _ => Err(ErrorCode::SystemCode(SystemCode::EncodingError)),
        }
    }
}

impl<'a> TryFrom<&'a Param<'a>> for &'a [u8] {
    type Error = ErrorCode;
    fn try_from(value: &'a Param<'a>) -> Result<Self, Self::Error> {
        match value {
            Param::Slice(slice) => Ok(slice),
            _ => Err(ErrorCode::SystemCode(SystemCode::EncodingError)),
        }
    }
}

impl<'a> TryFrom<&'a Param<'a>> for &'a str {
    type Error = ErrorCode;
    fn try_from(value: &'a Param<'a>) -> Result<Self, Self::Error> {
        match value {
            Param::String(s) => Ok(s),
            _ => Err(ErrorCode::SystemCode(SystemCode::EncodingError)),
        }
    }
}

impl<'a> TryFrom<&'a Param<'a>> for u128 {
    type Error = ErrorCode;
    fn try_from(value: &'a Param<'a>) -> Result<Self, Self::Error> {
        match value {
            Param::U128(num) => Ok(*num),
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

impl From<AccountID> for Param<'_> {
    fn from(account_id: AccountID) -> Self {
        Param::AccountID(account_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrow_in1() {
        // Create a request with the first param as a byte slice.
        let input_bytes = b"hello";
        let req = Request::new1_g(MessageSelector::default(), input_bytes.as_slice());

        // Borrow the first param as &[u8].
        let got = req
            .borrow_in1::<&[u8]>()
            .expect("Failed to borrow_in1")
            .expect("Expected param not to be empty");

        assert_eq!(got, input_bytes);
    }

    #[test]
    fn test_borrow_in2() {
        // Create a request with two params: first is a string, second is a u128.
        let input_str = "world";
        let input_num = 123u128;
        let req = Request::new2_g(MessageSelector::default(), input_str, input_num);

        // Borrow the second param as a u128.
        let got_second = req
            .borrow_in2::<u128>()
            .expect("Failed to borrow_in2")
            .expect("Expected second param not to be empty");

        assert_eq!(got_second, input_num);

        // (Optional) Borrow the first param as a &str to verify it's still correct.
        let got_first = req
            .borrow_in1::<&str>()
            .expect("Failed to borrow_in1")
            .expect("Expected first param not to be empty");
        assert_eq!(got_first, input_str);
    }

    #[test]
    fn test_borrow_in3() {
        // Create a request with three params: &str, &[u8], AccountID.
        let input_str = "test_str";
        let input_bytes = b"test_bytes";
        let input_account = AccountID::new(999);

        let req = Request::new3_g(
            MessageSelector::default(),
            input_str,
            input_bytes.as_slice(),
            input_account,
        );

        // Borrow the third param as an AccountID.
        let got_third = req
            .borrow_in3::<&AccountID>()
            .expect("Failed to borrow_in3")
            .expect("Expected third param not to be empty");

        assert_eq!(got_third.to_bytes(), input_account.to_bytes());

        // (Optional) check first and second too:
        let got_first = req
            .borrow_in1::<&str>()
            .expect("Failed to borrow_in1")
            .expect("Expected first param not to be empty");
        assert_eq!(got_first, input_str);

        let got_second = req
            .borrow_in2::<&[u8]>()
            .expect("Failed to borrow_in2")
            .expect("Expected second param not to be empty");
        assert_eq!(got_second, input_bytes);
    }
}
