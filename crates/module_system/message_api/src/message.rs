//! Message request and response types.
use crate::header::MessageSelector;

/// A message request.
#[non_exhaustive]
pub struct MessageRequest<'a> {
    /// The message selector.
    pub message_selector: MessageSelector,
    /// A u128 input parameter.
    pub in0: u128,
    /// A slice input parameter.
    pub in1: &'a [u8],
}

/// A message response.
#[non_exhaustive]
#[derive(Default)]
pub enum MessageResponse<'a> {
    /// An empty response.
    #[default]
    Empty,
    /// A slice output parameter.
    Slice(&'a [u8]),
    /// A u128 output parameter.
    U128(u128),
}

/// A state request.
#[non_exhaustive]
pub struct StateRequest<'a> {
    /// The message selector.
    pub message_selector: MessageSelector,
    /// A u128 input parameter.
    pub in0: u128,
    /// A slice input parameter.
    pub in1: &'a [u8],
    /// A slice output parameter.
    pub in2: &'a [u8],
}

impl<'a> StateRequest<'a> {
    /// Create a new state request with one input parameter.
    pub fn new1(message_selector: MessageSelector, in1: &'a [u8]) -> Self {
        Self { message_selector, in0: 0, in1, in2: &[] }
    }

    /// Create a new state request with two input parameters.
    pub fn new2(message_selector: MessageSelector, in1: &'a [u8], in2: &'a [u8]) -> Self {
        Self { message_selector, in0: 0, in1, in2 }
    }
}

/// A state update response.
/// Currently, this is empty and is left as a placeholder for future use.
#[non_exhaustive]
#[derive(Default)]
pub struct UpdateStateResponse<'a> {
    _marker: core::marker::PhantomData<&'a ()>,
}

/// A query state response.
#[non_exhaustive]
pub struct QueryStateResponse<'a> {
    /// The first slice output parameter.
    pub out1: &'a [u8],
    /// The second slice output parameter.
    pub out2: &'a [u8],
}

impl <'a> QueryStateResponse<'a> {
    /// Create a new query state response with one output parameter.
    pub fn new1(out1: &'a [u8]) -> Self {
        Self { out1, out2: &[] }
    }

    /// Create a new query state response with two output parameters.
    pub fn new2(out1: &'a [u8], out2: &'a [u8]) -> Self {
        Self { out1, out2 }
    }
}
