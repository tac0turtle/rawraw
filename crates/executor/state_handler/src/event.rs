#![allow(unused)]
use allocator_api2::vec::Vec;
use ixc_message_api::AccountID;

/// Represents event data with associated account information
#[derive(Clone, Debug)]
pub struct EventData {
    pub data: Vec<u8>,
    pub type_selector: u64,
    pub sender: AccountID,
}

/// Manages event state with support for snapshots and reversions
#[derive(Default)]
pub struct EventState {
    // Vector of event vectors, where each inner vector represents events for a transaction level
    events: Vec<Vec<EventData>>,
}

impl EventState {
    /// Creates a new EventState instance
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Takes a snapshot of the current event state
    /// Returns the current transaction level (events.len())
    pub fn snapshot(&mut self) -> usize {
        // Start a new transaction level by adding a new vector for events
        self.events.push(Vec::new());
        self.events.len()
    }

    /// Adds an event at the current transaction level
    pub fn emit_event(&mut self, sender: AccountID, type_selector: u64, data: Vec<u8>) {
        if self.events.is_empty() {
            // If no transaction level exists, create one
            self.events.push(Vec::new());
        }

        let current_level = self
            .events
            .last_mut()
            .expect("Events vector cannot be empty");
        current_level.push(EventData {
            data,
            type_selector,
            sender,
        });
    }

    /// Reverts the state to a previous snapshot level
    pub fn revert_to_snapshot(&mut self, snapshot_level: usize) {
        // Remove all events after the snapshot level
        self.events.truncate(snapshot_level);
    }

    /// Commits the current transaction level by merging events with the previous level
    pub fn commit(&mut self) {
        if self.events.len() >= 2 {
            let current_events = self.events.pop().expect("Events vector cannot be empty");
            let previous_level = self.events.last_mut().expect("Previous level must exist");
            previous_level.extend(current_events);
        }
    }

    /// Returns all events for the current transaction level
    pub fn get_current_events(&self) -> Option<&Vec<EventData>> {
        self.events.last()
    }

    /// Returns all events across all transaction levels
    pub fn get_all_events(&self) -> Vec<&EventData> {
        self.events.iter().flat_map(|level| level.iter()).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use allocator_api2::{alloc::Global, vec::Vec};

    fn create_test_data(data: &[u8]) -> Vec<u8> {
        let mut vec = Vec::new_in(Global);
        vec.extend_from_slice(data);
        vec
    }

    #[test]
    fn test_event_emission_and_reversion() {
        let mut event_state = EventState::new();

        // Take initial snapshot - level 0
        let snapshot1 = event_state.snapshot();

        // Emit some events at level 0
        event_state.emit_event(AccountID::new(1), 0, create_test_data(b"event1"));
        event_state.emit_event(AccountID::new(1), 0, create_test_data(b"event2"));

        // Take another snapshot - level 1
        let _ = event_state.snapshot();

        // Emit events at level 1
        event_state.emit_event(AccountID::new(2), 0, create_test_data(b"event3"));
        event_state.emit_event(AccountID::new(2), 0, create_test_data(b"event4"));

        // Verify level 1 events
        let current_events = event_state.get_current_events().unwrap();
        assert_eq!(current_events.len(), 2);
        assert_eq!(current_events[0].data, create_test_data(b"event3"));
        assert_eq!(current_events[1].data, create_test_data(b"event4"));

        // Take another snapshot - level 2
        let _ = event_state.snapshot();

        // Emit events at level 2
        event_state.emit_event(AccountID::new(3), 0, create_test_data(b"event5"));

        // Revert to snapshot1 (level 0)
        event_state.revert_to_snapshot(snapshot1);

        // Verify level 0 events remain
        let current_events = event_state.get_current_events().unwrap();
        assert_eq!(current_events.len(), 2);
        assert_eq!(current_events[0].data, create_test_data(b"event1"));
        assert_eq!(current_events[1].data, create_test_data(b"event2"));

        // Verify total number of levels
        assert_eq!(event_state.events.len(), 1);
    }

    #[test]
    fn test_event_commit() {
        let mut event_state = EventState::new();

        // First transaction level
        let _ = event_state.snapshot();
        event_state.emit_event(AccountID::new(1), 0, create_test_data(b"event1"));

        // Second transaction level
        let _ = event_state.snapshot();
        event_state.emit_event(AccountID::new(2), 0, create_test_data(b"event2"));

        // Commit second level to first
        event_state.commit();

        // Verify events are merged
        let current_events = event_state.get_current_events().unwrap();
        assert_eq!(current_events.len(), 2);
        assert_eq!(current_events[0].data, create_test_data(b"event1"));
        assert_eq!(current_events[1].data, create_test_data(b"event2"));
    }
}
