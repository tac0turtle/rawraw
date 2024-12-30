//! A u128 accumulator map.
use crate::prefix::Prefix;
use crate::{Item, Map};
use allocator_api2::alloc::Allocator;
use core::borrow::Borrow;
use ixc_core::error::{convert_client_error, ClientError};
use ixc_core::resource::{InitializationError, StateObjectResource};
use ixc_core::result::ClientResult;
use ixc_core::Context;
use ixc_message_api::code::ErrorCode;
use ixc_schema::schema::SchemaValue;
use ixc_schema::state_object::{ObjectKey, StateObjectDescriptor};
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// A 128-bit unsigned integer accumulator.
pub struct Accumulator {
    item: Item<u128>,
}

/// A map from keys to 128-bit unsigned integers that act as accumulators.
pub struct AccumulatorMap<K> {
    map: Map<K, u128>,
}

/// An error that can occur when performing a safe subtraction.
#[derive(Default, Debug, Clone, TryFromPrimitive, IntoPrimitive, SchemaValue, Copy)]
#[repr(u8)]
#[non_exhaustive]
pub enum SafeSubError {
    /// The subtraction would result in a negative value.
    #[default]
    Underflow,
}

impl Accumulator {
    /// Gets the current value, defaulting always to 0.
    pub fn get(&self, ctx: &Context) -> ClientResult<u128> {
        self.item.get(ctx)
    }

    /// Adds the given value to the current value.
    pub fn add(&self, ctx: &mut Context, value: u128) -> ClientResult<u128> {
        let current = self.item.get(ctx)?;
        let new_value = current.saturating_add(value);
        self.item.set(ctx, &new_value)?;
        Ok(new_value)
    }

    /// Subtracts the given value from the current value,
    /// returning an error if the subtraction would result in a negative value.
    pub fn safe_sub(&self, ctx: &mut Context, value: u128) -> ClientResult<u128, SafeSubError> {
        let current = self.item.get(ctx).map_err(convert_client_error)?;
        let new_value = current
            .checked_sub(value)
            .ok_or_else(|| ClientError::new(ErrorCode::HandlerCode(SafeSubError::Underflow)))?;
        self.item
            .set(ctx, &new_value)
            .map_err(convert_client_error)?;
        Ok(new_value)
    }
}

impl<K: ObjectKey> AccumulatorMap<K> {
    /// Gets the current value for the given key, defaulting always to 0.
    pub fn get<'a, L>(&self, ctx: &Context, key: L) -> ClientResult<u128>
    where
        L: Borrow<K::In<'a>>,
    {
        let value = self.map.get(ctx, key)?;
        Ok(value.unwrap_or_default())
    }

    /// Adds the given value to the current value for the given key.
    pub fn add<'a, L>(&self, ctx: &mut Context, key: L, value: u128) -> ClientResult<u128>
    where
        L: Borrow<K::In<'a>>,
    {
        let current = self.get(ctx, key.borrow())?;
        let new_value = current.saturating_add(value);
        self.map.set(ctx, key.borrow(), &new_value)?;
        Ok(new_value)
    }

    /// Subtracts the given value from the current value for the given key,
    /// returning an error if the subtraction would result in a negative value.
    pub fn safe_sub<'a, L>(
        &self,
        ctx: &mut Context,
        key: L,
        value: u128,
    ) -> ClientResult<u128, SafeSubError>
    where
        L: Borrow<K::In<'a>>,
    {
        let current = self.get(ctx, key.borrow()).map_err(convert_client_error)?;
        let new_value = current
            .checked_sub(value)
            .ok_or_else(|| ClientError::new(ErrorCode::HandlerCode(SafeSubError::Underflow)))?;
        self.map
            .set(ctx, key.borrow(), &new_value)
            .map_err(convert_client_error)?;
        Ok(new_value)
    }
}

unsafe impl StateObjectResource for Accumulator {
    unsafe fn new(scope: &[u8], prefix: u8) -> Result<Self, InitializationError> {
        let prefix = Prefix::new(scope, prefix)?;
        Ok(Accumulator {
            item: Item::new(prefix),
        })
    }

    #[cfg(feature = "std")]
    fn descriptor<'a>(
        allocator: &'a dyn Allocator,
        collection_name: &'a str,
        key_names: &[&'a str],
        value_names: &[&'a str],
    ) -> StateObjectDescriptor<'a> {
        let mut desc = Item::<u128>::descriptor(allocator, collection_name, key_names, value_names);
        desc.is_accumulator = true;
        desc
    }
}

unsafe impl<K: ObjectKey> StateObjectResource for AccumulatorMap<K> {
    unsafe fn new(scope: &[u8], prefix: u8) -> Result<Self, InitializationError> {
        let prefix = Prefix::new(scope, prefix)?;
        Ok(AccumulatorMap {
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
        let mut desc =
            Map::<K, u128>::descriptor(allocator, collection_name, key_names, value_names);
        desc.is_accumulator = true;
        desc
    }
}
