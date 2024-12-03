//! A state handler that can be used to store and retrieve state.
mod snapshot_state;

use crate::snapshot_state::SnapshotState;

use allocator_api2::alloc::Global;
use allocator_api2::vec::Vec;
use checkpoint::StateChange;
use quick_cache::unsync::Cache;

/// A store that can be used to store and retrieve state.
pub trait Store {
    /// Get the value for the given key.
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
    /// Commit the state changes.
    fn commit(&mut self) -> Result<(), ()>;
}

// /// A transaction.
// /// TODO: remove after https://github.com/tac0turtle/rawraw/pull/20/ is merged
// pub trait StateHandler: Store {
//     fn begin_tx(&mut self) -> Result<(), ()>;
//     fn commit_tx(&mut self) -> Result<(), ()>;
//     fn rollback_tx(&mut self) -> Result<(), ()>;

//     fn handle_exec(
//         &mut self,
//         message_packet: &mut MessagePacket,
//         allocator: &dyn Allocator,
//     ) -> Result<(), ErrorCode>;

//     fn handle_query(
//         &self,
//         message_packet: &mut MessagePacket,
//         allocator: &dyn Allocator,
//     ) -> Result<(), ErrorCode>;

//     fn create_account_storage(&mut self, account: AccountID) -> Result<(), ()>;
//     fn delete_account_storage(&mut self, account: AccountID) -> Result<(), ()>;
// }

/// StateHandler is a cache-based state handler that can be used to store and retrieve state.
pub struct StateHandler<'a, S: Store> {
    /// The underlying store which we read from.
    store: &'a S,
    /// The cache for items gotten from state
    /// This is used to cache reads
    cache: Cache<Vec<u8>, Vec<u8>>,

    snapshot_state: SnapshotState<S>,
    /// Checkpoints are used to revert state changes.
    /// checkpoints are used to follow the call stack of a transaction
    /// and revert the state changes when the transaction is rolled back.
    checkpoints: Vec<StateChange<S>>,
}

impl<'a, S: Store> StateHandler<'a, S> {
    /// Create a new state handler with the given store.
    pub fn new(store: S) -> Self {
        Self {
            store: store,
            cache: Cache::new(1_000_000), // TODO should this be preset?
            snapshot_state: SnapshotState::new(store),
            checkpoints: Vec::with_capacity_in(200, Global),
        }
    }

    /// Begins a new transaction.
    pub fn begin_tx(&mut self) -> Result<(), ()> {
        self.checkpoints.push(checkpoint);
        Ok(())
    }

    /// Commits the current transaction.
    pub fn commit_tx(&mut self) -> Result<(), ()> {
        Ok(())
    }

    /// Rolls back the current transaction.
    pub fn rollback_tx(&mut self) -> Result<(), ()> {
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
}
