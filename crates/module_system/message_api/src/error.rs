//! Handler error types.
//!
#[cfg(feature = "std")]
extern crate alloc;

use core::fmt::{Debug, Display, Formatter};
use crate::code::{ErrorCode, SystemCode};

/// An error type that can be returned by a handler that includes an optional message.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct HandlerError {
    /// The error code.
    pub code: ErrorCode,
    /// An optional message.
    #[cfg(feature = "std")]
    pub message: Option<alloc::string::String>,
}

impl HandlerError {
    /// Create a new handler error with the given error code.
    pub fn new(code: ErrorCode) -> Self {
        Self { code, message: None }
    }
}

impl From<SystemCode> for HandlerError {
    fn from(code: SystemCode) -> Self {
        Self { code: code.into(), message: None }
    }
}

impl From<ErrorCode> for HandlerError {
    fn from(code: ErrorCode) -> Self {
        Self { code, message: None }
    }
}

impl From<HandlerError> for ErrorCode {
    fn from(err: HandlerError) -> Self {
        err.code
    }
}