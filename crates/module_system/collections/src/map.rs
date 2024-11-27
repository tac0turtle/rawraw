//! The map module contains the `Map` struct, which represents a key-value map in storage.
use crate::store_client::KVStoreClient;
use core::borrow::Borrow;
use ixc_core::resource::{InitializationError, StateObjectResource};
use ixc_core::result::ClientResult;
use ixc_core::Context;
use ixc_schema::state_object::{
    decode_object_value, encode_object_key, encode_object_value, ObjectKey, ObjectValue,
};

/// A key-value map.
pub struct Map<K, V> {
    _k: core::marker::PhantomData<K>,
    _v: core::marker::PhantomData<V>,
    prefix: Vec<u8>,
}

impl<K, V> Map<K, V> {
    /// Creates a new map with the given prefix.
    pub fn new(scope: &[u8], prefix: u8) -> Self {
        let mut prefix_vec = Vec::with_capacity(scope.len() + 1);
        prefix_vec.push(prefix); // Add the prefix
        prefix_vec.extend_from_slice(scope); // Add the scope if needed
        Self {
            _k: core::marker::PhantomData,
            _v: core::marker::PhantomData,
            prefix: prefix_vec,
        }
    }
}

impl<K: ObjectKey, V: ObjectValue> Map<K, V> {
    /// Gets the value of the map at the given key.
    pub fn get<'a, 'b, L>(&self, ctx: &'a Context, key: L) -> ClientResult<Option<V::Out<'a>>>
    where
        L: Borrow<K::In<'b>>,
    {
        let key_bz = encode_object_key::<K>(&self.prefix, key.borrow(), ctx.memory_manager())?;

        let value_bz = KVStoreClient.get(ctx, key_bz)?;
        let value_bz = match value_bz {
            None => return Ok(None),
            Some(value_bz) => value_bz,
        };

        let value = decode_object_value::<V>(value_bz, ctx.memory_manager())?;
        Ok(Some(value))
    }

    /// Sets the value of the map at the given key.
    pub fn set<'a, L, U>(&self, ctx: &mut Context, key: L, value: U) -> ClientResult<()>
    where
        L: Borrow<K::In<'a>>,
        U: Borrow<V::In<'a>>,
    {
        let key_bz = encode_object_key::<K>(&self.prefix, key.borrow(), ctx.memory_manager())?;
        let value_bz = encode_object_value::<V>(value.borrow(), ctx.memory_manager())?;
        unsafe { KVStoreClient.set(ctx, key_bz, value_bz) }
    }

    /// Deletes the value of the map at the given key.
    pub fn delete<'a, L>(&self, ctx: &mut Context, key: L) -> ClientResult<()>
    where
        L: Borrow<K::In<'a>>,
    {
        let key_bz = encode_object_key::<K>(&self.prefix, key.borrow(), ctx.memory_manager())?;
        unsafe { KVStoreClient.delete(ctx, key_bz) }
    }
}

unsafe impl<K, V> StateObjectResource for Map<K, V> {
    unsafe fn new(scope: &[u8], prefix: u8) -> core::result::Result<Self, InitializationError> {
        let mut prefix_vec = Vec::with_capacity(scope.len() + 1);
        prefix_vec.push(prefix); // Add the prefix
        prefix_vec.extend_from_slice(scope); // Add the scope if needed
        Ok(Self {
            _k: core::marker::PhantomData,
            _v: core::marker::PhantomData,
            prefix: prefix_vec,
        })
    }
}
