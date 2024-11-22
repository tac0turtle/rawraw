#![allow(unused)]
use std::collections::HashMap;
use crate::stf::State;

enum Value {
    Deleted,
    Updated(Vec<u8>),
}

#[derive(PartialEq, Debug)]
pub enum StateChange {
    Delete {
        key: Vec<u8>,
        old_value: Option<Vec<u8>>,
    },
    Update {
        key: Vec<u8>,
        value: Vec<u8>,
    },
}

impl StateChange {
    fn revert(self, changes: &mut HashMap<Vec<u8>, Value>) {
        match self {
            StateChange::Update { key, value } => {
                changes.remove(&key);
            }

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

pub struct Checkpoint {
    index: usize,
}

pub struct SnapshotState<S> {
    state: S,
    changes: HashMap<Vec<u8>, Value>,
    changelog: Vec<StateChange>,
}

impl<S> SnapshotState<S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            changes: Default::default(),
            changelog: vec![],
        }
    }
}

impl<S: State> SnapshotState<S> {
    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.changes
            .insert(key.clone(), Value::Updated(value.clone()));
        self.changelog.push(StateChange::Update { key, value });
    }

    pub fn delete(&mut self, key: &[u8]) {
        self.changes.insert(key.to_vec(), Value::Deleted);
        self.changelog.push(StateChange::Delete {
            key: key.to_vec(),
            old_value: self.get(key),
        });
    }

    pub fn state_changes(self) -> Vec<StateChange> {
        self.changelog
    }

    pub fn checkpoint(&mut self) -> Checkpoint {
        Checkpoint {
            index: self.changelog.len() - 1,
        }
    }

    pub fn go_to_checkpoint(&mut self, snapshot: Checkpoint) -> Result<(), ()> {
        for i in snapshot.index..self.changelog.len() {
            // pop in reverse
            let change = self.changelog.pop().unwrap();
            change.revert(&mut self.changes)
        }

        Ok(())
    }
}

impl<S: State> State for SnapshotState<S> {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        // try to get from values
        match self.changes.get(key) {
            // get from disk db
            None => self.state.get(key),
            // found in change list
            Some(value) => match value {
                Value::Updated(data) => Some(data.clone()),
                Value::Deleted => None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // implement in memory disk db
    impl State for HashMap<Vec<u8>, Vec<u8>> {
        fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
            self.get(key).cloned()
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

        let before_ante_handler_snapshot = snapshot_state.checkpoint();

        snapshot_state.set(b"ante_handler".to_vec(), b"ante".to_vec());
        snapshot_state.set(b"bob".to_vec(), b"0ixc".to_vec());
        snapshot_state.delete(b"charlie_grant");

        let before_tx_exec_snapshot = snapshot_state.checkpoint();
        snapshot_state.set(b"alice".to_vec(), b"3ixc".to_vec());

        // test revert
        snapshot_state
            .go_to_checkpoint(before_tx_exec_snapshot)
            .unwrap();

        // test changes
        let expected_changes = vec![
            StateChange::Update {
                key: b"begin_block".to_vec(),
                value: b"begin_block".to_vec(),
            },
            StateChange::Update {
                key: b"ante_handler".to_vec(),
                value: b"ante".to_vec(),
            },
            StateChange::Update {
                key: b"bob".to_vec(),
                value: b"0ixc".to_vec(),
            },
        ];

        assert_eq!(snapshot_state.state_changes(), expected_changes);
    }
}
