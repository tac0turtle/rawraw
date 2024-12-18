//! Gas metering utility.

use core::cell::Cell;
use ixc_message_api::code::{ErrorCode, SystemCode};

/// A wrapper for gas.
#[derive(Debug)]
pub enum GasMeter {
    /// An unlimited gas meter.
    Unlimited,
    /// A metered gas meter.
    Metered {
        /// The amount of gas left.
        gas_left: Cell<u64>,
    },
}

impl GasMeter {
    /// Create a new metered gas meter.
    pub fn new(gas_left: u64) -> Self {
        Self::Metered {
            gas_left: Cell::new(gas_left),
        }
    }

    /// Get the amount of gas left.
    /// Returns `None` if gas is unlimited.
    pub fn get_left(&self) -> Option<u64> {
        match self {
            GasMeter::Unlimited => None,
            GasMeter::Metered { gas_left } => Some(gas_left.get()),
        }
    }

    /// Consume gas.
    pub fn consume_gas(&self, amount: u64) -> Result<(), ErrorCode> {
        match self {
            GasMeter::Unlimited => Ok(()),
            GasMeter::Metered { gas_left } => {
                let left = gas_left.get();
                if left < amount {
                    gas_left.set(0);
                    Err(ErrorCode::SystemCode(SystemCode::OutOfGas))
                } else {
                    gas_left.set(left - amount);
                    Ok(())
                }
            }
        }
    }
}
