use ixc_message_api::AccountID;

pub trait IDGenerator {
    fn new_account_id(&mut self) -> Result<AccountID, ()>;
    fn new_unique_id(&mut self) -> Result<u128, ()>;
}

