use ixc_schema::SchemaValue;
use crate::result::ClientResult;
use crate::{low_level, Context};

use ixc_schema::structs::StructSchema;

/// An event bus that can be used to emit events.
#[derive(Default, Copy, Clone)]
pub struct EventBus<E> {
    _marker: core::marker::PhantomData<E>,
}

impl<'a, E: StructSchema + SchemaValue<'a>> EventBus<E> {
    /// Emits an event to the event bus.
    pub fn emit(&mut self, ctx: &mut Context, event: &E) -> ClientResult<()> {
        low_level::emit_event(ctx, event)
    }
}

