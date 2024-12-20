//! Handler error types.
//!
#[cfg(feature = "std")]
extern crate alloc;

use core::fmt::{Debug, Display, Formatter};
use crate::code::ErrorCode;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct HandlerError {
    pub code: ErrorCode,
    #[cfg(feature = "std")]
    pub msg: alloc::string::String,
}

impl HandlerError {
    fn new(code: ErrorCode) -> Self {
        Self { code, msg: alloc::string::String::new() }
    }
}