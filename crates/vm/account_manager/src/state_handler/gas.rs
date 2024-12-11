//! Gas metering utility.
use ixc_message_api::code::{ErrorCode, SystemCode};

/// A wrapper for gas.
pub struct GasMeter(u64);

impl GasMeter {
    /// Create a new gas value.
    pub fn new(gas: u64) -> Self {
        Self(gas)
    }

    /// Get the gas value.
    pub fn get(&self) -> u64 {
        self.0
    }

    /// Consume gas.
    pub fn consume(&mut self, amount: u64) -> Result<(), ErrorCode> {
        if self.0 == 0 {
            // if the gas value is 0, that means we're not metering gas
            return Ok(());
        }

        if self.0 < amount {
            self.0 = 0;
            return Err(ErrorCode::SystemCode(SystemCode::OutOfGas));
        }
        self.0 -= amount;
        Ok(())
    }
}


