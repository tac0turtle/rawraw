//! The item module contains the `Item` struct, which represents a single item in storage.

use crate::prefix::Prefix;
use crate::Map;
use allocator_api2::alloc::Allocator;
use core::borrow::Borrow;
use ixc_core::resource::{InitializationError, StateObjectResource};
use ixc_core::result::ClientResult;
use ixc_core::Context;
use ixc_schema::state_object::{ObjectValue, StateObjectDescriptor};

/// A single item in storage.
pub struct Item<V> {
    map: Map<(), V>,
}

impl<V> Item<V> {
    /// Creates a new item with the given prefix.
    pub(crate) const fn new(prefix: Prefix) -> Self {
        Self {
            map: Map::new(prefix),
        }
    }
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

unsafe impl<T: ObjectValue> StateObjectResource for Item<T> {
    unsafe fn new(scope: &[u8], prefix: u8) -> core::result::Result<Self, InitializationError> {
        let prefix = Prefix::new(scope, prefix)?;
        Ok(Self {
            map: Map::new(prefix),
        })
    }

    #[cfg(feature = "std")]
    fn descriptor<'a>(
        allocator: &'a dyn Allocator,
        collection_name: &'a str,
        key_names: &[&'a str],
        value_names: &[&'a str],
    ) -> StateObjectDescriptor<'a> {
        if value_names.is_empty() {
            // we have a special default case where the item is named by the name of the collection
            Map::<(), T>::descriptor(allocator, collection_name, key_names, &[collection_name])
        } else {
            Map::<(), T>::descriptor(allocator, collection_name, key_names, value_names)
        }
    }
}
