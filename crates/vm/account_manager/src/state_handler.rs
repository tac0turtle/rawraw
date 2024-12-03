use allocator_api2::alloc::Allocator;
use allocator_api2::vec::Vec;
use ixc_message_api::AccountID;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::packet::MessagePacket;
use crate::{ROOT_ACCOUNT};
use crate::id_generator::IDGenerator;

pub trait Store<A: Allocator> {
    fn kv_get(&self, account_id: AccountID, key: &[u8], gas: &mut Gas, allocator: A) -> Result<Option<Vec<u8, A>>, ErrorCode>;
    fn kv_set(&mut self, account_id: AccountID, key: &[u8], value: &[u8], gas: &mut Gas) -> Result<(), ErrorCode>;
    fn kv_delete(&mut self, account_id: AccountID, key: &[u8], gas: &mut Gas) -> Result<(), ErrorCode>;
}

pub struct Gas(u64);

/// A transaction.
pub trait StateHandler<A: Allocator>: Store<A> {
    fn begin_tx(&mut self) -> Result<(), ErrorCode>;
    fn commit_tx(&mut self) -> Result<(), ErrorCode>;
    fn rollback_tx(&mut self) -> Result<(), ErrorCode>;

    fn handle_exec(
        &mut self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode>;

    fn handle_query(
        &self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode>;

    fn create_account_storage(&mut self, account: AccountID) -> Result<(), ErrorCode>;
    fn delete_account_storage(&mut self, account: AccountID) -> Result<(), ErrorCode>;
}

pub(crate) fn get_account_handler_id<A: Allocator, ST: Store<A>>(
    state_handler: &ST,
    account_id: AccountID,
    gas: &mut Gas,
    allocator: A,
) -> Result<Option<Vec<u8, A>>, ErrorCode> {
    let id: u128 = account_id.into();
    let key = format!("h:{}", id);
    state_handler.kv_get(ROOT_ACCOUNT, key.as_bytes(), gas, allocator)
}

pub(crate) fn init_next_account<A: Allocator, ST: StateHandler<A>, IDG: IDGenerator>(
    id_generator: &mut IDG,
    state_handler: &mut ST,
    handler_id: &[u8],
    gas: &mut Gas,
) -> Result<AccountID, ErrorCode> {
    let id: u128 = id_generator.new_account_id()?.into();
    state_handler.create_account_storage(AccountID::new(id))?;
    state_handler.kv_set(
        ROOT_ACCOUNT,
        // TODO choose a different layout for the key
        format!("h:{}", id).as_bytes(),
        handler_id,
        gas,
    )?;
    Ok(AccountID::new(id))
}

pub(crate) fn destroy_account_data<A: Allocator, ST: StateHandler<A>>(state_handler: &mut ST, account: AccountID, gas: &mut Gas) -> Result<(), ErrorCode> {
    let id: u128 = account.into();
    let key = format!("h:{}", id);
    state_handler.kv_delete(ROOT_ACCOUNT, key.as_bytes(), gas)?;
    state_handler.delete_account_storage(account)
}

