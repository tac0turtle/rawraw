//! Gas metering utility.
use core::cell::Cell;
use ixc_message_api::code::{ErrorCode, SystemCode};

/// A wrapper for gas.
#[derive(Debug, Default, Clone)]
pub struct GasMeter {
    limit: u64,
    pub(crate) consumed: Cell<u64>,
}

impl GasMeter {
    /// Create a new gas meter with a limit.
    pub fn new(limit: Option<u64>) -> Self {
        Self {
            limit: limit.unwrap_or(0),
            consumed: Cell::new(0),
        }
    }

    /// Create a new gas meter with a limit.
    /// If the limit is set to 0, then gas metering will be unlimited.
    pub fn with_limit(limit: u64) -> Self {
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
    pub(crate) fn limit(&self) -> Option<u64> {
        if self.limit == 0 {
            None
        } else {
            Some(self.limit)
        }
    }

    /// Get the amount of gas left.
    pub(crate) fn left(&self) -> Option<u64> {
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
            Err(ErrorCode::SystemCode(SystemCode::OutOfGas))
        } else {
            Ok(())
        }
    }
}
