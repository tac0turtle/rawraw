//! A state transition function that can be used to execute transactions and query state.
mod examples;
mod info;

use crate::info::Info;
use std::marker::PhantomData;

use allocator_api2::alloc::Allocator;
use ixc_account_manager::authz::AuthorizationMiddleware;
use ixc_account_manager::id_generator::IDGenerator;
use ixc_account_manager::state_handler::gas::GasMeter;
use ixc_account_manager::state_handler::StateHandler;
use ixc_account_manager::AccountManager;
use ixc_message_api::handler::{HostBackend, RawHandler};
use ixc_message_api::{code::ErrorCode, header::MessageSelector, packet::MessagePacket, AccountID};
use ixc_vm_api::VM;

pub struct BlockReq<T: Transation> {
    pub height: u64,
    pub time: u64,
    pub transactions: Vec<T>,
}

/// A transaction that can be used to execute a message .
pub trait Transation {
    /// Get the sender of the transaction.
    fn sender(&self) -> &AccountID;
    /// Get the recipient of the transaction.
    fn recipient(&self) -> &AccountID;
    /// Get the message of the transaction.
    fn msg(&self) -> &[u8];
    /// Get the message selector of the transaction.
    fn selector(&self) -> &MessageSelector;
}

pub trait BeforeTxApply {
    fn before_tx_apply<Vm: VM, SH: StateHandler, IDG: IDGenerator, Tx: Transation>(
        am: &AccountManager<Vm>,
        idg: &mut IDG,
        sh: &mut SH,
        tx: &Tx,
        alloc: &dyn Allocator,
    ) -> Result<(), ErrorCode>;
}

pub trait AfterTxApply {
    fn after_tx_apply<Vm: VM, SH: StateHandler, IDG: IDGenerator, Tx: Transation>(
        am: &AccountManager<Vm>,
        idg: &mut IDG,
        sh: &SH,
        tx: &Tx,
        msg_result: &Result<&[u8], ErrorCode>,
    ) -> Result<(), ErrorCode>;
}

/// A state transition function that can be used to execute transactions and query state.
pub struct STF<BeforeTxApply, PostTxApply>(PhantomData<BeforeTxApply>, PhantomData<PostTxApply>);

/// TODO: this would be used to whoever is unwrapping the error to know exactly at which stage the tx execution
/// failed.
pub enum TxFailure {
    BeforeTx(ErrorCode),
    ApplyTx(ErrorCode),
    PostTx(ErrorCode),
}

impl<Btx: BeforeTxApply, PTx: AfterTxApply> STF<Btx, PTx> {
    pub const ACCOUNT_TO_HANDLER_PREFIX: u8 = 0;
    pub const fn new() -> Self {
        Self(PhantomData, PhantomData)
    }

    pub fn apply_tx<Vm: VM, SH: StateHandler, IDG: IDGenerator, Tx: Transation>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        id_generator: &mut IDG,
        tx: &Tx,
        allocator: &dyn Allocator,
    ) -> Result<Vec<u8>, ErrorCode> {
        Btx::before_tx_apply(am, sh, id_generator, tx, allocator)?;
        let mut message_packet = Self::new_message_packet(tx, allocator);

        // handle msg

        let mut msg = Self::new_message_packet(tx, allocator);
        am.invoke_msg(sh, id_generator, &NoOpAuthorizer, &mut msg, allocator)?;

        let resp = Self::response_from_message_packet(&msg);

        PTx::after_tx_apply(am, sh, id_generator, tx, &resp)?;


        Ok(todo!("impl"))
    }

    pub fn new_message_packet<'a>(
        tx: &impl Transation,
        alloc: &dyn Allocator,
    ) -> MessagePacket<'a> {
        todo!()
    }

    fn response_from_message_packet(packet: &MessagePacket<'_>) -> Result<&'_ [u8], ErrorCode> {
        todo!()
    }
}
struct NoOpAuthorizer;

impl AuthorizationMiddleware for NoOpAuthorizer {
    fn authorize(&self, real_caller: AccountID, msg: &MessagePacket) -> Result<(), ErrorCode> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_stf() {}
}
