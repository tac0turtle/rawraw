//! Basic error handling utilities.

use crate::result::ClientResult;
use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use ixc_message_api::code::{ErrorCode, HandlerCode, SystemCode};
use ixc_message_api::code::SystemCode::EncodingError;
use ixc_schema::decoder::DecodeError;
use ixc_schema::encoder::EncodeError;

/// The standard error type returned by handlers.
#[derive(Clone)]
pub struct HandlerError<E: HandlerCode = u8> {
    pub(crate) code: ErrorCode<E>,
    #[cfg(feature = "std")]
    pub(crate) msg: Option<alloc::string::String>,
}

impl<E: HandlerCode> HandlerError<E> {
    /// Format a new error message.
    pub fn new_fmt(args: core::fmt::Arguments<'_>) -> Self {
        #[cfg(feature = "std")]
        let mut message = alloc::string::String::new();
        #[cfg(feature = "std")]
        core::fmt::write(&mut message, args).unwrap();
        Self {
            code: ErrorCode::SystemCode(SystemCode::Other),
            #[cfg(feature = "std")]
            msg: Some(message),
        }
    }

    /// Format a new error message with a code.
    pub fn new_fmt_with_code(code: E, args: core::fmt::Arguments<'_>) -> Self {
        #[cfg(feature = "std")]
        let mut message = alloc::string::String::new();
        #[cfg(feature = "std")]
        core::fmt::write(&mut message, args).unwrap();
        Self {
            code: ErrorCode::HandlerCode(code),
            #[cfg(feature = "std")]
            msg: Some(message),
        }
    }

    /// Format a new error message with a code.
    pub fn new_from_code(code: E) -> Self {
        Self {
            code: ErrorCode::HandlerCode(code),
            #[cfg(feature = "std")]
            msg: None,
        }
    }

    #[cfg(feature = "std")]
    fn fmt_error(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if let Some(msg) = &self.msg {
            if ErrorCode::SystemCode(SystemCode::Other) != self.code {
                write!(f, "code: {:?}: {}", self.code, msg)
            } else {
                write!(f, "{}", msg)
            }
        } else {
            if ErrorCode::SystemCode(SystemCode::Other) != self.code {
                write!(f, "code: {:?}: ", self.code)
            } else {
                write!(f, "unknown error")
            }
        }
    }

    #[cfg(not(feature = "std"))]
    fn fmt_error(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if let Some(code) = &self.code {
            write!(f, "code: {:?}: ", code)
        } else {
            write!(f, "unknown error")
        }
    }
}

impl<E: HandlerCode> Debug for HandlerError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.fmt_error(f)
    }
}

impl<E: HandlerCode> Display for HandlerError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.fmt_error(f)
    }
}

impl<E: HandlerCode, F: HandlerCode> From<ClientError<E>> for HandlerError<F> {
    fn from(value: ClientError<E>) -> Self {
        let code: ErrorCode<F> = if value.code == ErrorCode::SystemCode(SystemCode::OutOfGas) {
            ErrorCode::SystemCode(SystemCode::OutOfGas)
        } else {
            ErrorCode::SystemCode(SystemCode::Other)
        };
        HandlerError {
            code,
            #[cfg(feature = "std")]
            msg: Some(alloc::format!("got error: {}", value)),
        }
    }
}

/// Converts an error code with one handler code to an error code with another handler code.
pub fn convert_error_code<E: HandlerCode, F: HandlerCode>(code: ErrorCode<E>) -> ErrorCode<F> {
    let c: u16 = code.into();
    ErrorCode::<F>::from(c)
}

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
            code: EncodingError.into(),
        }
    }
}

impl<E: HandlerCode> From<DecodeError> for ClientError<E> {
    fn from(_: DecodeError) -> Self {
        ClientError {
            code: EncodingError.into(),
        }
    }
}

impl<E: HandlerCode> From<allocator_api2::alloc::AllocError> for ClientError<E> {
    fn from(_: allocator_api2::alloc::AllocError) -> Self {
        ClientError {
            code: EncodingError.into(),
        }
    }
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
