use crate::State;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub enum KeyTrace {
    Found { key: Vec<u8> },
    NotFound { key: Vec<u8> },
}

pub struct StateTracer<S> {
    db: S,
    traces: RefCell<Vec<KeyTrace>>,
}

impl<S> StateTracer<S> {
    pub fn new(db: S) -> Self {
        Self {
            db,
            traces: RefCell::new(Vec::new()),
        }
    }

    pub fn into_traces(self) -> Vec<KeyTrace> {
        self.traces.into_inner()
    }
}

impl<S: State> State for StateTracer<S> {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, String> {
        let result = self.db.get(key)?;
        match result {
            None => {
                self.traces
                    .borrow_mut()
                    .push(KeyTrace::NotFound { key: key.to_vec() });
                Ok(None)
            }
            Some(value) => {
                self.traces
                    .borrow_mut()
                    .push(KeyTrace::Found { key: key.to_vec() });
                Ok(Some(value))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sled::{DbChange, Sled};
    use crate::tracer::{KeyTrace, StateTracer};
    use crate::State;

    #[test]
    fn test_state() {
        let mut db = Sled::new("cookies").unwrap();
        db.commit_changes(
            0,
            vec![DbChange::Insert {
                key: b"cookies".to_vec(),
                value: vec![],
            }],
        )
        .unwrap();

        let state = db.load_latest_state().unwrap();

        let tracer = StateTracer::new(state);
        tracer.get(b"cookies").unwrap();
        tracer.get(b"not found").unwrap();

        let traces = tracer.into_traces();
        assert_eq!(traces.len(), 2);
        assert_eq!(
            traces,
            vec![
                KeyTrace::Found {
                    key: b"cookies".to_vec()
                },
                KeyTrace::NotFound {
                    key: b"not found".to_vec()
                }
            ]
        );
    }
}
