//! A state transition function that can be used to execute transactions and query state.
mod info;

use crate::info::Info;
use std::marker::PhantomData;

use allocator_api2::alloc::Allocator;
use ixc_account_manager::id_generator::IDGenerator;
use ixc_account_manager::state_handler::StateHandler;
use ixc_account_manager::AccountManager;
use ixc_message_api::handler::{HostBackend, RawHandler};
use ixc_message_api::message::{Message, MessageSelector, Request};
use ixc_message_api::{code::ErrorCode, AccountID};
use ixc_vm_api::VM;

pub struct BlockReq<T: Transation> {
    pub height: u64,
    pub time: u64,
    pub transactions: Vec<T>,
}

/// A transaction that can be used to execute a message .
pub trait Transation {
    /// Get the sender of the transaction.
    fn sender(&self) -> AccountID;
    /// Get the recipient of the transaction.
    fn recipient(&self) -> AccountID;
    /// Get the message of the transaction.
    fn msg(&self) -> &[u8];
    /// Get the message selector of the transaction.
    fn selector(&self) -> MessageSelector;
}

pub trait BeforeTxApply {
    fn before_tx_apply<Vm: VM, SH: StateHandler, IDG: IDGenerator, Tx: Transation>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        idg: &mut IDG,
        tx: &Tx,
        alloc: &dyn Allocator,
    ) -> Result<(), ErrorCode>;
}

pub trait AfterTxApply {
    fn after_tx_apply<Vm: VM, SH: StateHandler, IDG: IDGenerator, Tx: Transation>(
        am: &AccountManager<Vm>,
        sh: &SH,
        idg: &mut IDG,
        tx: &Tx,
        msg_result: &Result<&[u8], ErrorCode>,
    ) -> Result<(), ErrorCode>;
}

pub trait BeginBlocker {
    fn begin_blocker<Vm: VM, SH: StateHandler>();
}

pub trait EndBlocker {
    fn end_blocker<Vm: VM, SH: StateHandler>();
}

/// A state transition function that can be used to execute transactions and query state.
pub struct STF<BeforeTxApply, PostTxApply, BeginBlocker, EndBlocker>(
    PhantomData<BeforeTxApply>,
    PhantomData<PostTxApply>,
    PhantomData<BeginBlocker>,
    PhantomData<EndBlocker>,
);

/// TODO: this would be used to whoever is unwrapping the error to know exactly at which stage the tx execution
/// failed.
pub enum TxFailure {
    BeforeTx(ErrorCode),
    ApplyTx(ErrorCode),
    PostTx(ErrorCode),
}

impl<Btx: BeforeTxApply, PTx: AfterTxApply, Bb: BeginBlocker, Eb: EndBlocker>
    STF<Btx, PTx, Bb, Eb>
{
    pub const ACCOUNT_TO_HANDLER_PREFIX: u8 = 0;
    pub const fn new() -> Self {
        Self(PhantomData, PhantomData, PhantomData, PhantomData)
    }

    pub fn begin_block<Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        id_generator: &mut IDG,
    ) {
    }

    pub fn end_block() {}

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
        am.invoke_msg(sh, id_generator, tx.sender(), allocator)?;

        let resp = Self::response_from_message_packet(&message_packet);

        PTx::after_tx_apply(am, sh, id_generator, tx, &resp)?;

        Ok(todo!("impl"))
    }

    pub fn new_message_packet<'a>(
        tx: &impl Transation,
        alloc: &dyn Allocator,
    ) -> MessagePacket<'a> {
        todo!()
    }

    fn response_from_message_packet<'a>(
        packet: &'a MessagePacket<'a>,
    ) -> Result<&'a [u8], ErrorCode> {
        todo!()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_stf() {}
}
