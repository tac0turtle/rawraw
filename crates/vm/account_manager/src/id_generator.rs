use ixc_message_api::AccountID;

pub trait IDGenerator {
    fn new_account_id(&mut self) -> Result<AccountID, ()>;
    fn new_unique_id(&mut self) -> Result<u128, ()>;
}

#[derive(Default)]
pub struct IncrementingIDGenerator {
    next_id: u128,
}

impl IDGenerator for IncrementingIDGenerator {
    fn new_account_id(&mut self) -> Result<AccountID, ()> {
        let id = self.next_id;
        self.next_id += 1;
        Ok(AccountID::new(id))
    }

    fn new_unique_id(&mut self) -> Result<u128, ()> {
        let id = self.next_id;
        self.next_id += 1;
        Ok(id)
    }
}
