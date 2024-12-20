//! Error and success codes returned by the message API.

use core::fmt::Debug;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Error and success codes returned by the message API.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum ErrorCode<E: HandlerCode = u8> {
    /// An error code returned by the system.
    System(SystemCode),

    /// A standard error code returned by a handler or the system.
    Std(StdCode),

    /// A custom error code returned by a handler.
    Custom(E),

    /// Unknown error code.
    Unknown(u16),
}

/// A trait implemented by all types that can be used as custom handler error codes.
pub trait HandlerCode: Into<u8> + TryFrom<u8> + Debug + Clone {}
impl<T: Into<u8> + TryFrom<u8> + Debug + Clone> HandlerCode for T {}

/// A set of error codes that only the system can return.
/// Handler may receive these codes, but cannot return them.
#[derive(Clone, Copy, PartialEq, Eq, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
#[non_exhaustive]
pub enum SystemCode {
    /// Fatal execution error that likely cannot be recovered from.
    FatalExecutionError = 1,
    /// Account not-found error.
    AccountNotFound = 2,
    /// Message handler not-found error.
    HandlerNotFound = 3,
    /// The caller attempted to impersonate another caller and was not authorized.
    UnauthorizedCallerAccess = 4,
    /// The handler code was invalid, failed to execute properly within its virtual machine
    /// or otherwise behaved incorrectly.
    InvalidHandler = 5,
    /// A volatile message was attempted to be invoked by a query handler.
    VolatileAccessError = 6,
    /// The call stack overflowed.
    CallStackOverflow = 7,
}

/// A set of standard error codes that handlers or the system can return.
#[derive(Clone, Copy, PartialEq, Eq, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
#[non_exhaustive]
pub enum StdCode {
    /// Any uncategorized error.
    Other = 128,
    /// The handler doesn't handle the specified message.
    MessageNotHandled = 129,
    /// Encoding error.
    EncodingError = 130,
    /// Out of gas error.
    OutOfGas = 131,
    /// Unexpected error. This is used for errors that are not expected to occur, possibly indicating a bug.
    Unexpected = 132,
}

impl<E: HandlerCode> From<u16> for ErrorCode<E> {
    fn from(value: u16) -> Self {
        match value {
            0..256 => {
                if let Ok(e) = SystemCode::try_from(value as u8) {
                    ErrorCode::System(e)
                } else {
                    ErrorCode::Unknown(value)
                }
            }
            256..512 => {
                if let Ok(e) = StdCode::try_from((value - 256) as u8) {
                    ErrorCode::Std(e)
                } else {
                    ErrorCode::Unknown(value)
                }
            },
            512..768 => {
                if let Ok(e) = E::try_from((value - 512) as u8) {
                    ErrorCode::Custom(e)
                } else {
                    ErrorCode::Unknown(value)
                }
            },
            _ => ErrorCode::Unknown(value),
        }
    }
}

impl<E: HandlerCode> From<ErrorCode<E>> for u16 {
    fn from(val: ErrorCode<E>) -> Self {
        match val {
            ErrorCode::System(e) => e as u16,
            ErrorCode::Std(e) => e as u16 + 256,
            ErrorCode::Custom(e) => e.into() as u16 + 512,
            ErrorCode::Unknown(e) => e,
        }
    }
}

impl<E: HandlerCode> From<SystemCode> for ErrorCode<E> {
    fn from(code: SystemCode) -> Self {
        ErrorCode::System(code)
    }
}

impl<E: HandlerCode> From<StdCode> for ErrorCode<E> {
    fn from(code: StdCode) -> Self {
        ErrorCode::Std(code)
    }
}

impl<E: HandlerCode> PartialEq<Self> for ErrorCode<E> {
    fn eq(&self, other: &Self) -> bool {
        let a: u16 = self.clone().into();
        let b: u16 = other.clone().into();
        a == b
    }
}

impl<E: HandlerCode> Eq for ErrorCode<E> {}
