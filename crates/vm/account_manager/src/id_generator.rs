use ixc_message_api::AccountID;
use ixc_message_api::code::ErrorCode;

pub trait IDGenerator {
    fn new_account_id(&mut self) -> Result<AccountID, ErrorCode>;
    fn new_unique_id(&mut self) -> Result<u128, ErrorCode>;
}

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
