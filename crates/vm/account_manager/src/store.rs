use ixc_message_api::AccountID;

trait SimpleStore {
    fn get(&self, account_id: AccountID, key: &[u8]) -> Option<Vec<u8>>;
    fn set(&mut self, account_id: AccountID, key: &[u8], value: &[u8]);
    fn delete(&mut self, account_id: AccountID, key: &[u8]);
}

trait Store: SimpleStore {
    fn accumulator_get(&self, account_id: AccountID, key: &[u8]) -> u128;
    fn accumulator_add(&mut self, account_id: AccountID, key: &[u8], value: u128);
    fn accumulator_safe_sub(&mut self, account_id: AccountID, key: &[u8], value: u128) -> bool;
    fn account_scoped_get(&self, account_id: AccountID, scoped: AccountID, key: &[u8]) -> Option<Vec<u8>>;
    fn account_scoped_set(&mut self, account_id: AccountID, scoped: AccountID, key: &[u8], value: &[u8]);
    fn account_scoped_delete(&mut self, account_id: AccountID, scoped: AccountID, key: &[u8]);
}