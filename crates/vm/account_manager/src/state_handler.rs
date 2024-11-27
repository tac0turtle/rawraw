use allocator_api2::alloc::Allocator;
use ixc_message_api::AccountID;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::packet::MessagePacket;
use crate::{ROOT_ACCOUNT};
use crate::id_generator::IDGenerator;

/// A transaction.
pub trait StateHandler {
    fn begin_tx(&mut self) -> Result<(), ()>;
    fn commit_tx(&mut self) -> Result<(), ()>;
    fn rollback_tx(&mut self) -> Result<(), ()>;

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

    fn raw_kv_get(&self, account_id: AccountID, key: &[u8]) -> Option<Vec<u8>>;
    fn raw_kv_set(&mut self, account_id: AccountID, key: &[u8], value: &[u8]);
    fn raw_kv_delete(&mut self, account_id: AccountID, key: &[u8]);

    fn create_account_storage(&mut self, account: AccountID) -> Result<(), ()>;
    fn delete_account_storage(&mut self, account: AccountID) -> Result<(), ()>;
}

pub(crate) fn get_account_handler_id<ST: StateHandler>(
    state_handler: &ST,
    account_id: AccountID,
) -> Option<Vec<u8>> {
    let id: u128 = account_id.into();
    let key = format!("h:{}", id);
    state_handler.raw_kv_get(ROOT_ACCOUNT, key.as_bytes())
}

pub(crate) fn init_next_account<ST: StateHandler, IDG: IDGenerator>(
    id_generator: &mut IDG,
    state_handler: &mut ST,
    handler_id: &[u8],
) -> Result<AccountID, ()> {
    let id: u128 = id_generator.new_account_id()?.into();
    state_handler.init_account_storage(AccountID::new(id))?;
    state_handler.raw_kv_set(
        ROOT_ACCOUNT,
        // TODO choose a different layout for the key
        format!("h:{}", id).as_bytes(),
        handler_id,
    );
    Ok(AccountID::new(id))
}

pub(crate) fn destroy_account_data<ST: StateHandler>(state_handler: &mut ST, account: AccountID) -> Result<(), ()> {
    let current_account = state_handler.active_account();
    let id: u128 = current_account.into();
    let key = format!("h:{}", id);
    state_handler.raw_kv_delete(ROOT_ACCOUNT, key.as_bytes());
    state_handler.self_destruct_account()
}

