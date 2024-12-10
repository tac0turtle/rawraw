//! A state handler that can be used to store and retrieve state.
mod snapshot_state;

use crate::snapshot_state::{Snapshot, SnapshotState};
use allocator_api2::alloc::{Allocator, Global};
use allocator_api2::vec::Vec;

/// A store that can be used to store and retrieve state.
pub trait Store {
    /// Get the value for the given key.
    fn get(&self, key: &Vec<u8>) -> Option<Vec<u8>>;
}

// TODO: remove this trait once https://github.com/tac0turtle/rawraw/pull/20
/// A state handler that can be used to store and retrieve state.
pub trait StateHandler {
    /// Begins a new transaction.
    fn begin_tx(&mut self) -> Result<(), ()>;
    /// Commits the current transaction.
    fn commit_tx(&mut self) -> Result<(), ()>;
    /// Rolls back the current transaction.
    fn rollback_tx(&mut self) -> Result<(), ()>;
}

/// StateHandler is a cache-based state handler that can be used to store and retrieve state.
pub struct Handler<'a, S: Store> {
    snapshot_state: SnapshotState<'a, S>,
    /// Checkpoints are used to revert state changes.
    /// checkpoints are used to follow the call stack of a transaction
    /// and revert the state changes when the transaction is rolled back.
    checkpoints: Vec<Snapshot>,
}

impl<'a, S: Store> Handler<'a, S> {
    /// Create a new state handler with the given store.
    pub fn new(store: &'a S) -> Self {
        Self {
            snapshot_state: SnapshotState::new(store),
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

    /// Gets the value for the given key.
    pub fn get<A: Allocator>(&self, key: &Vec<u8>, allocator: A) -> Option<Vec<u8, A>> {
        self.snapshot_state.get(key, allocator)
    }

    /// Sets the value for the given key.
    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.snapshot_state.set(key, value);
    }

    /// Deletes the value for the given key.
    pub fn delete(&mut self, key: &Vec<u8>) {
        self.snapshot_state.delete(key);
    }
}
