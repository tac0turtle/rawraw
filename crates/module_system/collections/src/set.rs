//! The set module contains the `Set` struct, which represents a set of keys in storage.

use crate::Map;
use core::borrow::Borrow;
use ixc_core::result::ClientResult;
use ixc_core::Context;
use ixc_schema::state_object::ObjectKey;
use std::ops::Deref;

/// A set of keys in storage.
pub struct Set<K> {
    map: Map<K, ()>,
}

impl<K> Set<K> {
    pub const fn new(prefix: u8) -> Self {
        Self {
            map: Map::new(prefix),
        }
    }
}

impl<K: ObjectKey> Set<K> {
    pub fn set<'a, L>(&mut self, ctx: &mut Context, key: L) -> ClientResult<()>
    where
        L: Borrow<K::In<'a>>,
    {
        self.map.set(ctx, key, ())
    }

    pub fn contains<'a, L>(&self, ctx: &mut Context, key: L) -> ClientResult<bool>
    where
        L: Borrow<K::In<'a>>,
    {
        self.map.get(ctx, key).map(|v| match v {
            None => false,
            Some(_) => true,
        })
    }

    pub fn delete<'a, L>(&mut self, ctx: &mut Context, key: L) -> ClientResult<()>
    where
        L: Borrow<K::In<'a>>
    {
        self.map.delete(ctx, key)
    }
}
