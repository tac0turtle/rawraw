use allocator_api2::alloc::Allocator;
use ixc_message_api::AccountID;

pub trait StdStateManager<A: Allocator> {
    fn kv_get(
        &self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        allocator: A,
    ) -> Result<Option<Vec<u8, A>>, StdStateError>;
    fn kv_set(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        value: &[u8],
    ) -> Result<(), StdStateError>;
    fn kv_delete(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
    ) -> Result<(), StdStateError>;
    fn accumulator_get(
        &self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        allocator: A,
    ) -> Result<u128, StdStateError>;
    fn accumulator_add(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        value: u128,
    ) -> Result<(), StdStateError>;
    fn accumulator_safe_sub(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        value: u128,
    ) -> Result<bool, StdStateError>;
    fn begin_tx(&mut self) -> Result<(), StdStateError>;
    fn commit_tx(&mut self) -> Result<(), StdStateError>;
    fn rollback_tx(&mut self) -> Result<(), StdStateError>;
    fn init_account_storage(&mut self, account: AccountID) -> Result<(), StdStateError>;
    fn delete_account_storage(&mut self, account: AccountID) -> Result<(), StdStateError>;
}

pub enum StdStateError {
    FatalExecutionError,
}

