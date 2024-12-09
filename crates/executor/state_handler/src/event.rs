/// EventState is a struct that represents the state of events.
use allocator_api2::alloc::Global;
use allocator_api2::vec::Vec;
pub struct EventState {
    events: Vec<Vec<Vec<u8>>>,
}

impl EventState {
    /// Creates a new event state.
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }
    /// Adds an event to the event state.
    pub fn add_event(&mut self, event: Vec<u8>) {
        let mut v = Vec::new_in(Global);
        v.push(event);
        self.events.push(v);
    }

    /// Returns the events.
    pub fn events(&self) -> &Vec<Vec<Vec<u8>>> {
        &self.events
    }

    pub fn revert_event(&mut self, snapshot: usize) -> Result<(), ()> {
        for _ in snapshot..self.events.len() {
            // pop in reverse
            let event = self.events.pop().unwrap();
        }

        Ok(())
    }
}
