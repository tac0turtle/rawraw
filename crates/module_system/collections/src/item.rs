//! The item module contains the `Item` struct, which represents a single item in storage.

use crate::Map;
use core::borrow::Borrow;
use ixc_core::resource::{InitializationError, StateObjectResource};
use ixc_core::result::ClientResult;
use ixc_core::Context;
use ixc_schema::state_object::ObjectValue;

/// A single item in storage.
pub struct Item<V> {
    map: Map<(), V>,
}

impl<V: ObjectValue> Item<V>
where
    for<'a> V::Out<'a>: Default,
{
    /// Gets the value of the item.
    pub fn get<'value>(&self, ctx: &'value Context) -> ClientResult<V::Out<'value>> {
        let v = self.map.get(ctx, ())?;
        Ok(v.unwrap_or_default())
    }

    /// Sets the value of the item.
    pub fn set<'value, U>(&self, ctx: &'value mut Context, value: U) -> ClientResult<()>
    where
        U: Borrow<V::In<'value>>,
    {
        self.map.set(ctx, (), value)
    }
}

unsafe impl<T> StateObjectResource for Item<T> {
    unsafe fn new(scope: &[u8], prefix: u8) -> core::result::Result<Self, InitializationError> {
        Ok(Self {
            map: Map::new(scope, prefix)?,
        })
    }
}
