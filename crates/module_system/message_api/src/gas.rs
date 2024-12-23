//! Simple gas tracking utility.

use core::cell::Cell;

/// A simple type which can be used to track gas usage for a single message execution.
#[non_exhaustive]
#[derive(Debug, Clone, Default)]
pub struct GasTracker {
    /// The gas limit, if any.
    pub limit: Option<u64>,
    /// The amount of gas consumed.
    pub consumed: Cell<u64>,
}

impl GasTracker {
    /// Creates a new gas tracker with an optional limit.
    pub fn new(limit: Option<u64>) -> Self {
        Self {
            limit,
            consumed: Cell::new(0),
        }
    }

    /// Creates a new gas tracker with no limit.
    pub fn unlimited() -> Self {
        Default::default()
    }

    /// Creates a new gas tracker with a limit.
    pub fn limited(limit: u64) -> Self {
        Self::new(Some(limit))
    }
}
