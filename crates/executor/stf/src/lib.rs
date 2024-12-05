//! A state transition function that can be used to execute transactions and query state.
mod info;

use allocator_api2::alloc::Allocator;
use ixc_message_api::{code::ErrorCode, header::MessageSelector, packet::MessagePacket, AccountID};

/// A store that can be used to store and retrieve state.
pub trait Store {
    /// Get the value for the given key.
    fn get(&self, key: &Vec<u8>) -> Option<Vec<u8>>;
}

/// A transaction that can be used to execute a message .
pub trait Transation {
    /// Get the sender of the transaction.
    fn sender(&self) -> &AccountID;
    /// Get the recipient of the transaction.
    fn recipient(&self) -> &AccountID;
    /// Get the message of the transaction.
    fn msg(&self) -> &[u8];
    /// Get the message selector of the transaction.
    fn selector(&self) -> &MessageSelector;
}
/// An account manager that can be used to invoke messages and queries. TODO: remove once https://github.com/tac0turtle/rawraw/pull/20 is merged
pub trait AccountManager {
    /// Invokes a message on the account manager.
    fn invoke_msg(
        &mut self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode>;
    /// Invokes a query on the account manager.
    fn invoke_query(
        &self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode>;
}

/// A state transition function that can be used to execute transactions and query state.
pub struct STF {
    account_manager: dyn AccountManager,
}

impl STF {
    /// new creates a new state transition function.
    pub fn new() -> Self {
        Self {
            account_manager: Default::default(),
        }
    }

    /// execute_txs executes a list of transactions and updates the state.
    pub fn execute_txs<S: Store, T: Transation>(
        store: &S,
        transactions: &Vec<T>,
    ) -> Result<(), ErrorCode> {
        // set header

        // Begin block

        // execute transactions
        for tx in transactions {
            Self::exec_tx(store, tx)?;
        }

        // End block
        Ok(())
    }

    /// exec_tx executes a transaction and updates the state.
    pub fn exec_tx<S: Store, T: Transation>(store: &S, tx: &T) -> Result<(), ErrorCode> {
        // Verify the transaction signature

        // antehandler operations

        // execute the transaction
        Ok(())
    }

    /// query queries the state.
    pub fn query<T: Store>(store: &T) -> Result<(), ErrorCode> {
        // set header
        // query operations
        Ok(())
    }

    ///simulate_txs simulates a list of transactions and returns the state changes.
    pub fn simulate_txs<T: Store>(store: &T) {
        // set header

        // verify the transaction signature

        // antehandler operations

        // execute transaction

        // return gas used
    }
}
