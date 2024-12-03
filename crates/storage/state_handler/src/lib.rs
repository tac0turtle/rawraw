//! A state handler that can be used to store and retrieve state.
mod snapshot_state;

use crate::snapshot_state::{Snapshot, SnapshotState};

use allocator_api2::alloc::Global;
use allocator_api2::vec::Vec;

/// A store that can be used to store and retrieve state.
pub trait Store {
    /// Get the value for the given key.
    fn get(&self, key: &Vec<u8>) -> Option<Vec<u8>>;
    /// Commit the state changes.
    fn commit(&mut self) -> Result<(), ()>;
}

/// StateHandler is a cache-based state handler that can be used to store and retrieve state.
pub struct StateHandler<'a, S: Store> {
    /// The underlying store which we read from.
    store: &'a S,

    snapshot_state: SnapshotState<S>,
    /// Checkpoints are used to revert state changes.
    /// checkpoints are used to follow the call stack of a transaction
    /// and revert the state changes when the transaction is rolled back.
    checkpoints: Vec<Snapshot>,
}

impl<'a, S: Store> StateHandler<'a, S> {
    /// Create a new state handler with the given store.
    pub fn new(store: S) -> Self {
        Self {
            store: &store,
            snapshot_state: SnapshotState::new(store.clone()),
            checkpoints: Vec::with_capacity_in(200, Global),
        }
    }

    /// Begins a new transaction.
    pub fn begin_tx(&mut self) -> Result<(), ()> {
        let snapshot = self.snapshot_state.snapshot();
        self.checkpoints.push(snapshot);
        Ok(())
    }

    /// Commits the current transaction.
    pub fn commit_tx(&mut self) -> Result<(), ()> {
        self.checkpoints.pop();
        Ok(())
    }

    /// Rolls back the current transaction.
    pub fn rollback_tx(&mut self) -> Result<(), ()> {
        let checkpoint = self.checkpoints.pop().unwrap();
        self.snapshot_state.revert_to_snapshot(checkpoint)?;
        Ok(())
    }

    // /// handles a message packet
    // pub fn handle_exec(
    //     &mut self,
    //     message_packet: &mut MessagePacket,
    //     allocator: &dyn Allocator,
    // ) -> Result<(), ErrorCode> {
    //     panic!("not implemented");
    // }

    // /// handles a query packet
    // pub fn handle_query(
    //     &self,
    //     message_packet: &mut MessagePacket,
    //     allocator: &dyn Allocator,
    // ) -> Result<(), ErrorCode> {
    //     panic!("not implemented");
    // }

    //     fn create_account_storage(&mut self, account: AccountID) -> Result<(), ()>{
    //         panic!("not implemented");
    // };
    //     fn delete_account_storage(&mut self, account: AccountID) -> Result<(), ()>{
    //         panic!("not implemented");
    // };
}
