//! The map module contains the `Map` struct, which represents a key-value map in storage.
use crate::prefix::Prefix;
use crate::store_client::KVStoreClient;
use allocator_api2::vec::Vec;
use core::borrow::Borrow;
use core::marker::PhantomData;
use ixc_core::resource::{InitializationError, StateObjectResource};
use ixc_core::result::ClientResult;
use ixc_core::Context;
use ixc_message_api::handler::Allocator;
use ixc_schema::encoding::Encoding;
use ixc_schema::fields::FieldTypes;
use ixc_schema::list::List;
use ixc_schema::state_object::{
    decode_object_value, encode_object_key, encode_object_value, ObjectKey, ObjectValue,
    StateObjectDescriptor,
};

pub(crate) const MAX_SIZE: usize = 7;

/// A key-value map.
pub struct Map<K, V> {
    _phantom: (PhantomData<K>, PhantomData<V>),
    prefix: Prefix,
}

impl<K, V> Map<K, V> {
    /// Creates a new map with the given prefix.
    pub(crate) const fn new(prefix: Prefix) -> Self {
        Self {
            _phantom: (PhantomData, PhantomData),
            prefix,
        }
    }
}

impl<K: ObjectKey, V: ObjectValue> Map<K, V> {
    /// Gets the value of the map at the given key.
    pub fn get<'a, 'b, L>(&self, ctx: &'a Context, key: L) -> ClientResult<Option<V::Out<'a>>>
    where
        L: Borrow<K::In<'b>>,
    {
        let key_bz =
            encode_object_key::<K>(self.prefix.as_slice(), key.borrow(), ctx.memory_manager())?;

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
        let key_bz =
            encode_object_key::<K>(self.prefix.as_slice(), key.borrow(), ctx.memory_manager())?;
        let value_bz = encode_object_value::<V>(value.borrow(), ctx.memory_manager())?;
        unsafe { KVStoreClient.set(ctx, key_bz, value_bz) }
    }

    /// Deletes the value of the map at the given key.
    pub fn delete<'a, L>(&self, ctx: &mut Context, key: L) -> ClientResult<()>
    where
        L: Borrow<K::In<'a>>,
    {
        let key_bz =
            encode_object_key::<K>(self.prefix.as_slice(), key.borrow(), ctx.memory_manager())?;
        unsafe { KVStoreClient.delete(ctx, key_bz) }
    }
}

unsafe impl<K: ObjectKey, V: ObjectValue> StateObjectResource for Map<K, V> {
    unsafe fn new(scope: &[u8], prefix: u8) -> core::result::Result<Self, InitializationError> {
        let prefix = Prefix::new(scope, prefix)?;
        Ok(Self {
            _phantom: (PhantomData, PhantomData),
            prefix,
        })
    }

    #[cfg(feature = "std")]
    fn descriptor<'a>(
        allocator: &'a dyn Allocator,
        collection_name: &'a str,
        key_names: &[&'a str],
        value_names: &[&'a str],
    ) -> StateObjectDescriptor<'a> {
        let mut desc = StateObjectDescriptor::default();
        desc.name = collection_name;
        desc.encoding = Encoding::NativeBinary;
        if key_names.len() != K::FieldTypes::N {
            panic!("Expected {} key names for map \"{}\", but got {:?}. This generally means that you haven't specified name(...) correctly in #[state]. Ex. #[state(name(key1, key2)].",
                K::FieldTypes::N, collection_name, key_names);
        }
        if value_names.len() != V::FieldTypes::N {
            panic!("Expected {} value names for map \"{}\", but got {:?}. This generally means that you haven't specified name(...) correctly in #[state]. Ex. #[state(name(value1, value2)].",
                V::FieldTypes::N, collection_name, value_names);
        }
        let mut key_fields = Vec::new_in(allocator);
        for i in 0..K::FieldTypes::N {
            let mut field = K::FieldTypes::FIELDS[i];
            field.name = key_names[i];
            key_fields.push(field);
        }
        desc.key_fields = List::Owned(key_fields);
        let mut value_fields = Vec::new_in(allocator);
        for i in 0..V::FieldTypes::N {
            let mut field = V::FieldTypes::FIELDS[i];
            field.name = value_names[i];
            value_fields.push(field);
        }
        desc.value_fields = List::Owned(value_fields);
        desc
    }
}
