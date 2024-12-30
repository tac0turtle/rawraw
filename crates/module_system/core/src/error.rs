//! Basic error handling utilities.

use crate::result::ClientResult;
use alloc::format;
use alloc::string::String;
use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use ixc_message_api::code::{ErrorCode, HandlerCode, SystemCode};
use ixc_schema::decoder::DecodeError;
use ixc_schema::encoder::EncodeError;
use ixc_schema::SchemaValue;

/// The standard error type returned by handlers.
#[derive(Clone)]
pub struct HandlerError<E: HandlerCode + SchemaValue<'static> = u8> {
    pub(crate) code: ErrorCode<E>,
    #[cfg(feature = "std")]
    pub(crate) msg: String,
    // TODO no std version - fixed length 256 byte string probably
}

impl<E: HandlerCode + SchemaValue<'static>> HandlerError<E> {
    /// Create a new error message.
    pub fn new(msg: String) -> Self {
        HandlerError {
            code: ErrorCode::SystemCode(SystemCode::Other),
            #[cfg(feature = "std")]
            msg,
        }
    }

    /// Create a new error message with a code.
    pub fn new_with_code(code: E, msg: String) -> Self {
        HandlerError {
            code: ErrorCode::HandlerCode(code),
            #[cfg(feature = "std")]
            msg,
        }
    }

    /// Format a new error message.
    pub fn new_fmt(args: core::fmt::Arguments<'_>) -> Self {
        #[cfg(feature = "std")]
        let mut message = String::new();
        core::fmt::write(&mut message, args).unwrap();
        HandlerError::new(message)
    }

    /// Format a new error message with a code.
    pub fn new_fmt_with_code(code: E, args: core::fmt::Arguments<'_>) -> Self {
        #[cfg(feature = "std")]
        let mut message = String::new();
        core::fmt::write(&mut message, args).unwrap();
        HandlerError::new_with_code(code, message)
    }

    /// Format a new error message with a code.
    pub fn new_from_code(code: E) -> Self {
        HandlerError::new_with_code(code, String::new())
    }

    fn fmt_str(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if self.code != ErrorCode::SystemCode(SystemCode::Other) {
            write!(f, "code: {:?}: {}", self.code, self.msg)
        } else {
            write!(f, "{}", self.msg)
        }
    }
}

impl<E: HandlerCode + SchemaValue<'static>> Debug for HandlerError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.fmt_str(f)
    }
}

impl<E: HandlerCode + SchemaValue<'static>> Display for HandlerError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.fmt_str(f)
    }
}

impl<E: HandlerCode + SchemaValue<'static>, F: HandlerCode + SchemaValue<'static>>
    From<ClientError<E>> for HandlerError<F>
{
    fn from(value: ClientError<E>) -> Self {
        let code: ErrorCode<F> = if value.code == ErrorCode::SystemCode(SystemCode::OutOfGas) {
            ErrorCode::SystemCode(SystemCode::OutOfGas)
        } else {
            ErrorCode::SystemCode(SystemCode::Other)
        };
        HandlerError {
            code,
            #[cfg(feature = "std")]
            msg: format!("got error: {}", value),
        }
    }
}

// /// Format an error message.
// #[macro_export]
// macro_rules! fmt_error {
//     ($code:ident, $($arg:tt)*) => {
//         $crate::error::HandlerError::new_fmt_with_code($code, core::format_args!($($arg)*))
//     };
//     ($($arg:tt)*) => {
//         $crate::error::HandlerError::new_fmt(core::format_args!($($arg)*))
//     };
// }
//
// /// Return an error with a formatted message.
// #[macro_export]
// macro_rules! bail {
//     ($($arg:tt)*) => {
//         return core::result::Err($crate::error::fmt_error!($($arg)*));
//     };
// }
//
// /// Ensure a condition is true, otherwise return an error with a formatted message.
// #[macro_export]
// macro_rules! ensure {
//     ($cond:expr, $($arg:tt)*) => {
//         if !$cond {
//             return core::result::Err($crate::error::fmt_error!($($arg)*));
//         }
//     };
// }

/// The standard error type returned by client methods.
#[derive(Clone)]
#[non_exhaustive]
pub struct ClientError<E: HandlerCode> {
    /// The error code.
    pub code: ErrorCode<E>,
}

impl<E: HandlerCode> ClientError<E> {
    /// Creates a new client error.
    pub fn new(code: ErrorCode<E>) -> Self {
        ClientError { code }
    }
}

impl<E: HandlerCode> Debug for ClientError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "code: {:?}", self.code)
    }
}

impl<E: HandlerCode> Display for ClientError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "code: {:?}", self.code)
    }
}

impl<E: HandlerCode> Error for ClientError<E> {}

impl<E: HandlerCode> From<ErrorCode> for ClientError<E> {
    fn from(value: ErrorCode) -> Self {
        let code = convert_error_code(value);
        ClientError { code }
    }
}

impl<E: HandlerCode> From<EncodeError> for ClientError<E> {
    fn from(_: EncodeError) -> Self {
        ClientError {
            code: ErrorCode::SystemCode(SystemCode::EncodingError),
        }
    }
}

impl<E: HandlerCode> From<DecodeError> for ClientError<E> {
    fn from(_: DecodeError) -> Self {
        ClientError {
            code: ErrorCode::SystemCode(SystemCode::EncodingError),
        }
    }
}

impl<E: HandlerCode> From<allocator_api2::alloc::AllocError> for ClientError<E> {
    fn from(_: allocator_api2::alloc::AllocError) -> Self {
        ClientError {
            code: ErrorCode::SystemCode(SystemCode::EncodingError),
        }
    }
}

/// Converts an error code with one handler code to an error code with another handler code.
pub fn convert_error_code<E: HandlerCode, F: HandlerCode>(code: ErrorCode<E>) -> ErrorCode<F> {
    let c: u16 = code.into();
    ErrorCode::<F>::from(c)
}

/// Converts an error code with one handler code to an error code with another handler code.
pub fn convert_client_error<E: HandlerCode, F: HandlerCode>(err: ClientError<E>) -> ClientError<F> {
    ClientError {
        code: convert_error_code(err.code),
    }
}

/// Returns a default result if the error is `MessageNotHandled`.
pub fn unimplemented_ok<R: Default, E: HandlerCode>(res: ClientResult<R, E>) -> ClientResult<R, E> {
    match res {
        Ok(r) => Ok(r),
        Err(e) => match e.code {
            ErrorCode::SystemCode(SystemCode::MessageNotHandled) => Ok(Default::default()),
            _ => Err(e),
        },
    }
}
