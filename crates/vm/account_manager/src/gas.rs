//! Gas metering utility.
use core::cell::Cell;
use ixc_message_api::code::{ErrorCode, SystemCode};

/// A wrapper for gas.
#[derive(Debug, Default, Clone)]
pub struct GasMeter {
    pub(crate) limit: Cell<u64>,
    pub(crate) consumed: Cell<u64>,
}

impl GasMeter {
    /// Create a new gas meter with a limit.
    pub fn new(limit: Option<u64>) -> Self {
        Self {
            limit: Cell::new(limit.unwrap_or(0)),
            consumed: Cell::new(0),
        }
    }

    /// Create a new gas meter with a limit.
    /// If the limit is set to 0, then gas metering will be unlimited.
    pub fn with_limit(limit: u64) -> Self {
        Self::new(Some(limit))
    }

    /// Create a new unlimited gas meter.
    pub fn unlimited() -> Self {
        Self::new(None)
    }

    /// Returns the gas limit if there is one.
    pub(crate) fn limit(&self) -> Option<u64> {
        if self.limit.get() == 0 {
            None
        } else {
            Some(self.limit.get())
        }
    }

    /// Get the amount of gas left.
    pub(crate) fn left(&self) -> Option<u64> {
        if self.limit.get() == 0 {
            None
        } else {
            Some(self.limit.get().saturating_sub(self.consumed.get()))
        }
    }

    /// Consume gas.
    pub fn consume(&self, amount: u64) -> Result<(), ErrorCode> {
        let consumed = self.consumed.get().saturating_add(amount);
        self.consumed.set(consumed);
        let limit = self.limit.get();
        if limit > 0 && consumed > limit {
            Err(ErrorCode::SystemCode(SystemCode::OutOfGas))
        } else {
            Ok(())
        }
    }

    /// Returns the total amount of gas consumed since this meter was created.
    pub(crate) fn consumed(&self) -> u64 {
        self.consumed.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gas_limit() {
        let gas_meter = GasMeter::new(Some(100));
        assert_eq!(gas_meter.left(), Some(100));
        gas_meter.consume(90).unwrap();
        assert_eq!(gas_meter.consumed(), 90);
        assert_eq!(gas_meter.left(), Some(10));
        assert_eq!(gas_meter.consume(10), Ok(()));
        assert_eq!(gas_meter.consumed(), 100);
        assert_eq!(gas_meter.left(), Some(0));
        assert_eq!(gas_meter.consume(1), Err(ErrorCode::SystemCode(SystemCode::OutOfGas)));
        assert_eq!(gas_meter.consumed(), 101); // gas is consumed even if it's out of gas
        assert_eq!(gas_meter.left(), Some(0));
    }

    #[test]
    fn test_unlimited_gas() {
        let gas_meter = GasMeter::unlimited();
        assert_eq!(gas_meter.limit(), None);
        assert_eq!(gas_meter.left(), None);
        assert_eq!(gas_meter.consumed(), 0);
        assert_eq!(gas_meter.consume(100), Ok(()));
        assert_eq!(gas_meter.consumed(), 100);
        assert_eq!(gas_meter.left(), None);
        assert_eq!(gas_meter.consume(100), Ok(()));
        assert_eq!(gas_meter.consumed(), 200);
        assert_eq!(gas_meter.left(), None);
    }
}