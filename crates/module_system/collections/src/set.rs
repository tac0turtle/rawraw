//! The set module contains the `Set` struct, which represents a set of keys in storage.

use crate::Map;
use core::borrow::Borrow;
use ixc_core::{result::ClientResult, Context};
use ixc_schema::state_object::ObjectKey;

/// A set of keys in storage.
pub struct Set<K> {
    map: Map<K, ()>,
}

impl<K> Set<K> {
    pub fn new(scope: &[u8], prefix: u8) -> Self {
        Self {
            map: Map::new(scope, prefix),
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
        self.map.get(ctx, key).map(|_| true)
    }
    pub fn delete<'a, L>(&mut self, ctx: &mut Context, key: L) -> ClientResult<()>
    where
        L: Borrow<K::In<'a>>,
    {
        self.map.delete(ctx, key)
    }
}
