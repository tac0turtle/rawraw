//! The map module contains the `Map` struct, which represents a key-value map in storage.

use alloc::vec::Vec;
use core::borrow::Borrow;
use ixc_core::low_level::create_packet;
use ixc_core::resource::{InitializationError, StateObjectResource};
use ixc_core::result::ClientResult;
use ixc_core::Context;
use ixc_core_macros::message_selector;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::header::MessageSelector;
use ixc_message_api::AccountID;
use ixc_schema::state_object::{
    decode_object_value, encode_object_key, encode_object_value, ObjectKey, ObjectValue,
};

/// A key-value map.
pub struct Map<K, V> {
    _k: core::marker::PhantomData<K>,
    _v: core::marker::PhantomData<V>,
    #[cfg(feature = "std")]
    prefix: Vec<u8>,
    // TODO no_std prefix
}

impl<K: ObjectKey, V: ObjectValue> Map<K, V> {
    // /// Checks if the map contains the given key.
    // pub fn has<'key>(&self, ctx: &Context<'key>, key: K::Value<'key>) -> Result<bool> {
    //     todo!()
    // }

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

const STATE_ACCOUNT: AccountID = AccountID::new(2);

const GET_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.get");
const SET_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.set");
const DELETE_SELECTOR: MessageSelector = message_selector!("ixc.store.v1.delete");

struct KVStoreClient;

impl KVStoreClient {
    pub fn get<'a>(&self, ctx: &'a Context, key: &[u8]) -> ClientResult<Option<&'a [u8]>> {
        let mut packet = create_packet(ctx, STATE_ACCOUNT, GET_SELECTOR)?;
        let header = packet.header_mut();
        unsafe {
            header.in_pointer1.set_slice(key);
            match ctx
                .host_backend()
                .invoke(&mut packet, &ctx.memory_manager())
            {
                Err(ErrorCode::HandlerCode(0)) => {
                    return Ok(None);
                }
                _ => {}
            }
        }
        let res_bz = unsafe { packet.header().out_pointer1.get(&packet) };
        Ok(Some(res_bz))
    }

    pub unsafe fn set(&self, ctx: &Context, key: &[u8], value: &[u8]) -> ClientResult<()> {
        let mut packet = create_packet(ctx, STATE_ACCOUNT, SET_SELECTOR)?;
        let header = packet.header_mut();
        unsafe {
            header.in_pointer1.set_slice(key);
            header.in_pointer2.set_slice(value);
            ctx.host_backend()
                .invoke(&mut packet, &ctx.memory_manager())?;
        }
        Ok(())
    }

    pub unsafe fn delete(&self, ctx: &Context, key: &[u8]) -> ClientResult<()> {
        let mut packet = create_packet(ctx, STATE_ACCOUNT, DELETE_SELECTOR)?;
        let header = packet.header_mut();
        unsafe {
            header.in_pointer1.set_slice(key);
            ctx.host_backend()
                .invoke(&mut packet, &ctx.memory_manager())?;
        }
        Ok(())
    }
}

unsafe impl<K, V> StateObjectResource for Map<K, V> {
    unsafe fn new(scope: &[u8], p: u8) -> core::result::Result<Self, InitializationError> {
        let mut prefix = Vec::from(scope);
        prefix.push(p);
        Ok(Self {
            _k: core::marker::PhantomData,
            _v: core::marker::PhantomData,
            #[cfg(feature = "std")]
            prefix,
        })
    }
}
