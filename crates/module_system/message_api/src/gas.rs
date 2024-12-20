//! Gas metering utility.
use crate::code::{ErrorCode, StdCode, SystemCode};
use core::cell::Cell;

/// A wrapper for gas.
#[derive(Debug, Default, Clone)]
pub struct Gas {
    limit: u64,
    consumed: Cell<u64>,
}

impl Gas {
    /// Create a new gas meter with a limit.
    /// If the limit is set to 0, then gas metering will be unlimited.
    pub fn limited(limit: u64) -> Self {
        Self {
            limit,
            consumed: Cell::new(0),
        }
    }

    /// Create a new unlimited gas meter.
    pub fn unlimited() -> Self {
        Self {
            limit: 0,
            consumed: Cell::new(0),
        }
    }

    /// Returns the gas limit if there is one.
    pub fn limit(&self) -> Option<u64> {
        if self.limit == 0 {
            None
        } else {
            Some(self.limit)
        }
    }

    /// Get the amount of gas left.
    pub fn left(&self) -> Option<u64> {
        if self.limit == 0 {
            None
        } else {
            Some(self.limit - self.consumed.get())
        }
    }

    /// Consume gas.
    pub fn consume(&self, amount: u64) -> Result<(), ErrorCode> {
        let consumed = self.consumed.get().saturating_add(amount);
        self.consumed.set(consumed);
        if self.limit > 0 && consumed > self.limit {
            Err(ErrorCode::Std(StdCode::OutOfGas))
        } else {
            Ok(())
        }
    }

    /// Returns the amount of gas consumed.
    pub fn consumed(&self) -> u64 {
        self.consumed.get()
    }
}
