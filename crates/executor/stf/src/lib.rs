//! A state transition function that can be used to execute transactions and query state.
mod info;
use crate::info::Info;

use allocator_api2::alloc::Allocator;
use ixc_message_api::{code::ErrorCode, header::MessageSelector, packet::MessagePacket, AccountID};
use ixc_state_handler::{Handler, StateHandler, Store};

pub struct BlockReq<T: Transation> {
    pub height: u64,
    pub time: u64,
    pub transactions: Vec<T>,
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
pub struct STF<A: AccountManager> {
    account_manager: A,
    preexecution: Vec<AccountID>,
    postexecution: Vec<AccountID>,
}

impl<A: AccountManager + 'static> STF<A> {
    /// new creates a new state transition function.
    pub fn new(
        account_manager: A,
        preexecution: Vec<AccountID>,
        postexecution: Vec<AccountID>,
    ) -> Self {
        Self {
            account_manager,
            preexecution,
            postexecution,
        }
    }

    /// execute_txs executes a list of transactions and updates the state.
    pub unsafe fn execute_txs<S: Store, T: Transation>(
        &mut self,
        store: &S,
        block: &BlockReq<T>,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        // set height and time
        let i = Info::new(block.height, block.time);

        let mut state_handler = Handler::new(store);

        // Begin block
        for account in self.preexecution {
            let mut packet = MessagePacket::allocate(allocator, 0)?;
            let header = packet.header_mut();
            header.account = account;
            header.caller = account;
            header.message_selector = 0;
            self.account_manager.invoke_msg(&mut packet, allocator)?;
        }

        // execute transactions
        for tx in block.transactions {
            self.exec_tx(&state_handler, &tx, allocator)?;
        }

        for account in self.postexecution {
            let mut packet = MessagePacket::allocate(allocator, 0)?;
            let header = packet.header_mut();
            header.account = account;
            header.caller = account;
            header.message_selector = 0;
            self.account_manager.invoke_msg(&mut packet, allocator)?;
        }

        // End block
        Ok(())
    }

    /// exec_tx executes a transaction and updates the state.
    pub unsafe fn exec_tx<S: StateHandler, T: Transation>(
        &mut self,
        state_handler: &S,
        tx: &T,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        // Verify the transaction signature
        let sender = tx.sender().to_owned();
        let mut packet = MessagePacket::allocate(allocator, 0)?;
        let header = packet.header_mut();
        header.account = sender;
        header.caller = sender;
        header.message_selector = 0;
        let res = self.account_manager.invoke_msg(&mut packet, allocator);
        if res.is_err() {
            return res;
        }

        // antehandler operations

        // execute the transaction

        Ok(())
    }

    /// query queries the state.
    pub fn query<S: Store, C: Allocator>(store: &S, allocator: &C) -> Result<(), ErrorCode> {
        // set header
        // query operations
        Ok(())
    }

    ///simulate_txs simulates a list of transactions and returns the state changes.
    pub fn simulate_txs<S: Store, C: Allocator>(store: &S, allocator: &C) {
        // set header

        // verify the transaction signature

        // antehandler operations

        // execute transaction

        // return gas used
    }
}
