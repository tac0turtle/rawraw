use crate::result::ClientResult;
use crate::Context;

use allocator_api2::vec::Vec;

/// An event bus that can be used to emit events.
pub struct EventBus<T> {
    events: Vec<T>,
}

impl<T> Default for EventBus<T> {
    fn default() -> Self {
        Self { events: Vec::new() }
    }
}

impl Clone for EventBus<u8> {
    fn clone(&self) -> Self {
        Self {
            events: self.events.clone(),
        }
    }
}

impl<T: Clone> EventBus<T> {
    /// Emits an event to the event bus.
    pub fn emit(&mut self, _ctx: &mut Context, event: &T) -> ClientResult<()> {
        self.events.push(event.clone());
        Ok(())
    }

    /// Returns all events that have been emitted.
    pub fn get_events(&self) -> &[T] {
        &self.events
    }

    /// Clears all events from the bus.
    pub fn clear(&mut self) {
        self.events.clear();
    }
}
