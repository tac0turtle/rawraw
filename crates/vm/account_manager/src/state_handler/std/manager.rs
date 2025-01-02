use allocator_api2::alloc::Allocator;
use ixc_message_api::{code::ErrorCode, AccountID};

/// The standard state manager trait which is the interface
/// that the storage layer must implement in order to be
/// wrapped by the standard state handler.
pub trait StdStateManager {
    /// Get the value of a key in storage.
    /// Keys are scoped to an account and optionally to a scope which is another account
    /// under which this storage key would be scoped.
    fn kv_get<'a>(
        &self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        allocator: &'a dyn Allocator,
    ) -> Result<Option<&'a [u8]>, ErrorCode>;
    /// Set the value of a key in storage.
    fn kv_set(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        value: &[u8],
    ) -> Result<(), ErrorCode>;
    /// Delete the value of a key in storage.
    fn kv_delete(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
    ) -> Result<(), ErrorCode>;
    /// Get the value of an accumulator in storage.
    fn accumulator_get(
        &self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
    ) -> Result<u128, ErrorCode>;
    /// Add to the value of an accumulator in storage.
    /// Adds are saturating and can never overflow or fail.
    /// Because of this, adds may be performed in an undefined order
    /// because addition is commutative.
    fn accumulator_add(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        value: u128,
    ) -> Result<(), ErrorCode>;
    /// Safely subtract from the value of an accumulator in storage.
    /// If the operation would cause the value to go below zero, the
    /// operation fails and returns false.
    fn accumulator_safe_sub(
        &mut self,
        account_id: AccountID,
        scope: Option<AccountID>,
        key: &[u8],
        value: u128,
    ) -> Result<bool, ErrorCode>;
    /// Begin a transaction.
    fn begin_tx(&mut self) -> Result<(), ErrorCode>;
    /// Commit a transaction.
    fn commit_tx(&mut self) -> Result<(), ErrorCode>;
    /// Rollback a transaction.
    fn rollback_tx(&mut self) -> Result<(), ErrorCode>;
    /// Initialize storage for a new account.
    fn create_account_storage(&mut self, account: AccountID) -> Result<(), ErrorCode>;
    /// Delete all of an account's storage. Keys scoped under the account
    /// by another account, however, must not be deleted.
    fn delete_account_storage(&mut self, account: AccountID) -> Result<(), ErrorCode>;
    /// Emit an event. The sender of the event is the account that emitted it.
    fn emit_event(
        &mut self,
        sender: AccountID,
        type_selector: u64,
        data: &[u8],
    ) -> Result<(), ErrorCode>;
}
