//! Standard result types.
use crate::error::{ClientError, HandlerError};

/// The standard result type which should be used as a return type in handler implementations.
pub type Result<R, E = u8> = core::result::Result<R, HandlerError<E>>;

/// The standard result type returned by client methods.
pub type ClientResult<R, E = u8> = core::result::Result<R, ClientError<E>>;
