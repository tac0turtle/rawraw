//! ID generator trait.

use allocator_api2::alloc::Allocator;
use allocator_api2::vec::Vec;
use ixc_message_api::AccountID;
use ixc_message_api::code::ErrorCode;
use ixc_vm_api::ReadonlyStore;
use crate::state_handler::StateHandler;

/// ID generator trait.
pub trait IDGenerator {
    /// Generates a new account ID.
    fn new_account_id<S: Store>(&self, store: &mut S) -> Result<AccountID, ErrorCode>;
    /// Generates a new unique ID which can be used in any context.
    fn new_unique_id<S: Store>(&self, store: &mut S) -> Result<u128, ErrorCode>;
}

/// Store trait used by the ID generator.
pub trait Store {
    /// Get the value of the key.
    fn get(&self, key: &[u8]) -> Result<Option<&[u8]>, ErrorCode>;
    /// Set the value of the key.
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), ErrorCode>;
}

/// An ID generates IDs sequentially by incrementing an u128 value.
/// IDs start at 65536 and increment by 1 for each new ID.
#[derive(Default)]
pub struct IncrementingIDGenerator {}

impl IDGenerator for IncrementingIDGenerator {
    fn new_account_id<S: Store>(&self, store: &mut S) -> Result<AccountID, ErrorCode> {
        self.new_unique_id(store).map(|id| AccountID::new(id))
    }

    fn new_unique_id<S: Store>(&self, store: &mut S) -> Result<u128, ErrorCode> {
        if let Some(id) = store.get(b"I")? {
            let id = u128::from_le_bytes(id.try_into().unwrap());
            let new_id = id + 1;
            store.set(b"I", &new_id.to_le_bytes())?;
            Ok(new_id)

        } else {
            let id:u128 = 65536;
            store.set(b"I", &id.to_le_bytes())?;
            Ok(id)
        }
    }
}
