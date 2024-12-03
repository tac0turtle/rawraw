#![allow(unused)]
use allocator_api2::{
    alloc::{Allocator, Global},
    vec::Vec,
};
use quick_cache::unsync::Cache;
use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
};

use crate::Store;

pub struct Snapshot {
    index: usize,
}

pub struct SnapshotState<S> {
    state: S,
    changes: HashMap<Vec<u8>, Value>,
    changelog: Vec<StateChange>,
    /// The cache for items gotten from state
    /// This is used to cache reads
    cache: Cache<Vec<u8>, Vec<u8>>,
}

impl<S> SnapshotState<S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            changes: Default::default(),
            changelog: Vec::new(),
            cache: Cache::new(10_000), // TODO Decide on values
        }
    }
}

impl<S: Store> SnapshotState<S> {
    pub fn get<A: Allocator>(&mut self, key: &Vec<u8>, allocator: A) -> Option<Vec<u8, A>> {
        // try to get from values
        match self.changes.get(key) {
            // get from disk db
            None => {
                // check the cache first for the key, we may have already read it
                let value = self.cache.get(key);
                match value {
                    Some(value) => {
                        let mut v = Vec::new_in(allocator);
                        v.extend_from_slice(value);
                        Some(v)
                    }
                    None => {
                        // if not in cache, read from state
                        let v = self.state.get(key).unwrap();
                        // insert into cache
                        self.cache.insert(key.clone(), v.clone());
                        let mut vec = Vec::new_in(allocator);
                        vec.extend_from_slice(v.borrow());
                        Some(vec)
                    }
                }
            }

            // found in change list
            Some(value) => match value {
                Value::Updated(data) => {
                    let mut vec = Vec::new_in(allocator);
                    vec.extend_from_slice(data.borrow());
                    Some(vec)
                }
                Value::Deleted => None,
            },
        }
    }

    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) {
        let previous_value = self
            .changes
            .insert(key.clone(), Value::Updated(value.clone()));
        self.changelog.push(StateChange::Update {
            key,
            value,
            previous_value,
        });
    }

    pub fn delete(&mut self, key: &Vec<u8>) {
        let value = self.get(key.borrow(), Global); //TODO: change from global allocator
        self.changes.insert(key.clone(), Value::Deleted);
        self.changelog.push(StateChange::Delete {
            key: key.clone(),
            old_value: value,
        });
    }

    pub fn state_changes(self) -> Vec<StateChange> {
        self.changelog
    }

    pub fn snapshot(&mut self) -> Snapshot {
        Snapshot {
            index: self.changelog.len() - 1,
        }
    }

    pub fn revert_to_snapshot(&mut self, snapshot: Snapshot) -> Result<(), ()> {
        for i in snapshot.index..self.changelog.len() {
            // pop in reverse
            let change = self.changelog.pop().unwrap();
            change.revert(&mut self.changes)
        }

        Ok(())
    }
}

/// Value is a enum that represents the if a value is deleted or updated.
#[derive(Debug, PartialEq)]
enum Value {
    Deleted,
    Updated(Vec<u8>),
}

/// StateChange is a struct that represents a change in state.
#[derive(PartialEq, Debug)]
enum StateChange {
    Delete {
        key: Vec<u8>,
        old_value: Option<Vec<u8>>,
    },
    Update {
        key: Vec<u8>,
        value: Vec<u8>,
        previous_value: Option<Value>,
    },
}

/// Revert a state change.
impl StateChange {
    pub(crate) fn revert(self, changes: &mut HashMap<Vec<u8>, Value>) {
        match self {
            StateChange::Update {
                key,
                value,
                previous_value,
            } => match previous_value {
                Some(previous_value) => {
                    changes.insert(key, previous_value);
                }
                None => {
                    changes.remove(&key);
                }
            },

            StateChange::Delete { key, old_value } => {
                changes.insert(
                    key,
                    match old_value {
                        None => Value::Deleted,
                        Some(old_value) => Value::Updated(old_value),
                    },
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use allocator_api2::vec::Vec;

    // implement in memory disk db
    impl Store for HashMap<Vec<u8>, Vec<u8>> {
        fn get(&self, key: &Vec<u8>) -> Option<Vec<u8>> {
            self.get(key).cloned()
        }
        fn commit(&mut self) -> Result<(), ()> {
            Ok(())
        }
    }
    #[test]
    fn test_flow() {
        let mut state = HashMap::new();
        state.insert(b"alice".to_vec(), b"1ixc".to_vec());
        state.insert(b"bob".to_vec(), b"2ixc".to_vec());
        state.insert(b"charlie_grant".to_vec(), b"10ixc".to_vec());

        let mut snapshot_state = SnapshotState::new(HashMap::new());

        // set some values
        snapshot_state.set(b"begin_block".to_vec(), b"begin_block".to_vec());

        let before_ante_handler_snapshot = snapshot_state.snapshot();

        snapshot_state.set(b"ante_handler".to_vec(), b"ante".to_vec());
        snapshot_state.set(b"bob".to_vec(), b"0ixc".to_vec());
        snapshot_state.delete(b"charlie_grant");

        let before_tx_exec_snapshot = snapshot_state.snapshot();
        snapshot_state.set(b"alice".to_vec(), b"3ixc".to_vec());

        // test revert
        snapshot_state
            .revert_to_snapshot(before_tx_exec_snapshot)
            .unwrap();

        // test changes
        let expected_changes = vec![
            StateChange::Update {
                key: b"begin_block".to_vec(),
                value: b"begin_block".to_vec(),
                previous_value: None,
            },
            StateChange::Update {
                key: b"ante_handler".to_vec(),
                value: b"ante".to_vec(),
                previous_value: None,
            },
            StateChange::Update {
                key: b"bob".to_vec(),
                value: b"0ixc".to_vec(),
                previous_value: None,
            },
        ];

        assert_eq!(snapshot_state.state_changes(), expected_changes);
    }
}
