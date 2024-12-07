use allocator_api2::{
    alloc::{Allocator, Global},
    vec::Vec,
};
use std::{borrow::Borrow, collections::HashMap};

use crate::Store;

pub struct Snapshot {
    index: usize,
}

pub struct SnapshotState<'a, S> {
    state: &'a S,
    changes: HashMap<Vec<u8>, Value>,
    changelog: Vec<StateChange>,
}

impl<'a, S> SnapshotState<'a, S> {
    pub fn new(state: &'a S) -> Self {
        Self {
            state,
            changes: Default::default(),
            changelog: Vec::new(),
        }
    }
}

impl<'a, S: Store> SnapshotState<'a, S> {
    pub fn get<A: Allocator>(&self, key: &Vec<u8>, allocator: A) -> Option<Vec<u8, A>> {
        // try to get from values
        match self.changes.get(key) {
            // get from disk db
            None => {
                let v = self.state.get(key).unwrap();
                let mut vec = Vec::new_in(allocator);
                vec.extend_from_slice(&v);
                Some(vec)
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
        let value = self.get(key, Global); //TODO: change from global allocator
        self.changes.insert(key.clone(), Value::Deleted);
        self.changelog.push(StateChange::Delete {
            key: key.clone(),
            old_value: value,
        });
    }

    /// Returns the state changes.
    #[allow(unused)] //TODO: will be used when committing state changes
    pub fn state_changes(self) -> Vec<StateChange> {
        self.changelog
    }

    pub fn snapshot(&mut self) -> Snapshot {
        Snapshot {
            index: self.changelog.len() - 1,
        }
    }

    pub fn revert_to_snapshot(&mut self, snapshot: Snapshot) -> Result<(), ()> {
        for _ in snapshot.index..self.changelog.len() {
            // pop in reverse
            let change = self.changelog.pop().unwrap();
            change.revert(&mut self.changes)
        }

        Ok(())
    }
}

/// Value is a enum that represents the if a value is deleted or updated.
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Deleted,
    Updated(Vec<u8>),
}

/// StateChange is a struct that represents a change in state.
#[derive(PartialEq, Debug, Clone)]
pub enum StateChange {
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
    pub fn revert(self, changes: &mut HashMap<Vec<u8>, Value>) {
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
    }
    #[test]
    fn test_flow() {
        let mut state = HashMap::<Vec<u8>, Vec<u8>>::new();
        let mut alice = Vec::new();
        alice.extend_from_slice(b"alice");
        let mut bob = Vec::new();
        bob.extend_from_slice(b"bob");
        let mut charlie_grant = Vec::new();
        charlie_grant.extend_from_slice(b"charlie_grant");
        let mut ixc = Vec::new();
        ixc.extend_from_slice(b"1ixc");
        let mut ixc2 = Vec::new();
        ixc2.extend_from_slice(b"2ixc");
        let mut ixc10 = Vec::new();
        ixc10.extend_from_slice(b"10ixc");
        state.insert(alice, ixc);
        state.insert(bob, ixc2);
        state.insert(charlie_grant, ixc10);

        let state = state;
        let mut snapshot_state = SnapshotState::new(&state);

        // set some values
        let mut v1 = Vec::new();
        v1.extend_from_slice(b"begin_block");
        snapshot_state.set(v1.clone(), v1.clone());

        snapshot_state.snapshot();

        let mut v2 = Vec::new();
        v2.extend_from_slice(b"ante_handler");
        let mut v3 = Vec::new();
        v3.extend_from_slice(b"ante");
        snapshot_state.set(v2.clone(), v3.clone());
        let mut v4 = Vec::new();
        v4.extend_from_slice(b"bob");

        let mut v5 = Vec::new();
        v5.extend_from_slice(b"0ixc");
        snapshot_state.set(v4.clone(), v5.clone());
        let mut v6 = Vec::new();
        v6.extend_from_slice(b"charlie_grant");
        snapshot_state.delete(&v6);

        let before_tx_exec_snapshot = snapshot_state.snapshot();
        let mut v7 = Vec::new();
        v7.extend_from_slice(b"alice");
        let mut v8 = Vec::new();
        v8.extend_from_slice(b"3ixc");
        snapshot_state.set(v7, v8);

        // test revert
        snapshot_state
            .revert_to_snapshot(before_tx_exec_snapshot)
            .unwrap();

        // test changes
        let mut expected_changes = Vec::<StateChange>::new();
        let changes = [
            StateChange::Update {
                key: v1.clone(),
                value: v1,
                previous_value: None,
            },
            StateChange::Update {
                key: v2,
                value: v3,
                previous_value: None,
            },
            StateChange::Update {
                key: v4,
                value: v5,
                previous_value: None,
            },
        ];
        expected_changes.push(changes[0].clone());
        expected_changes.push(changes[1].clone());
        expected_changes.push(changes[2].clone());

        assert_eq!(snapshot_state.state_changes(), expected_changes);
    }
}
