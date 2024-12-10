//! ID generator trait.
use ixc_message_api::AccountID;
use ixc_message_api::code::ErrorCode;

/// ID generator trait.
pub trait IDGenerator {
    /// Generates a new account ID.
    fn new_account_id(&mut self) -> Result<AccountID, ErrorCode>;
    /// Generates a new unique ID which can be used in any context.
    fn new_unique_id(&mut self) -> Result<u128, ErrorCode>;
}

/// An ID generates IDs sequentially by incrementing an u128 value.
#[derive(Default)]
pub struct IncrementingIDGenerator {
    next_id: u128,
}

impl IDGenerator for IncrementingIDGenerator {
    fn new_account_id(&mut self) -> Result<AccountID, ErrorCode> {
        let id = self.next_id;
        self.next_id += 1;
        Ok(AccountID::new(id))
    }

    fn new_unique_id(&mut self) -> Result<u128, ErrorCode> {
        let id = self.next_id;
        self.next_id += 1;
        Ok(id)
    }
}
