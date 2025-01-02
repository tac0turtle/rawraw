//! A state transition function that can be used to execute transactions and query state.
mod info;

use crate::info::Info;
use std::marker::PhantomData;

use allocator_api2::alloc::Allocator;
use ixc_account_manager::id_generator::IDGenerator;
use ixc_account_manager::state_handler::StateHandler;
use ixc_account_manager::AccountManager;
use ixc_message_api::gas::GasTracker;
use ixc_message_api::handler::{HostBackend, InvokeParams, RawHandler};
use ixc_message_api::message::{Message, MessageSelector, Request, Response};
use ixc_message_api::{code::ErrorCode, AccountID};
use ixc_vm_api::VM;

/// A transaction that can be used to execute a message .
pub trait Transaction {
    /// Get the sender of the transaction.
    fn sender(&self) -> AccountID;
    /// Get the recipient of the transaction.
    fn recipient(&self) -> AccountID;
    /// Get the message of the transaction.
    fn msg(&self) -> &Message;
    /// Returns the Gas Limit.
    fn gas_limit(&self) -> u64;
}

pub trait TxValidator {
    fn validate_tx<Vm: VM, SH: StateHandler, IDG: IDGenerator, Tx: Transaction + ?Sized>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        idg: &mut IDG,
        tx: &Tx,
        alloc: &dyn Allocator,
    ) -> Result<(), ErrorCode>;
}

pub trait AfterTxApplyHandler {
    fn after_tx_apply<Vm: VM, SH: StateHandler, IDG: IDGenerator, Tx: Transaction + ?Sized>(
        am: &AccountManager<Vm>,
        sh: &SH,
        idg: &mut IDG,
        tx: &Tx,
        tx_result: &Result<Response, ErrorCode>,
    ) -> Result<(), ErrorCode>;
}

pub trait BeginBlocker {
    fn begin_blocker<Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        idg: &mut IDG,
    );
}

pub trait EndBlocker {
    fn end_blocker<Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        idg: &mut IDG,
    );
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

impl<Btx: TxValidator, PTx: AfterTxApplyHandler, Bb: BeginBlocker, Eb: EndBlocker>
    STF<Btx, PTx, Bb, Eb>
{
    pub const ACCOUNT_TO_HANDLER_PREFIX: u8 = 0;
    pub const fn new() -> Self {
        Self(PhantomData, PhantomData, PhantomData, PhantomData)
    }

    pub fn apply_block<Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        idg: &mut IDG,
        block: Vec<&dyn Transaction>,
        allocator: &dyn Allocator,
    ) {
        Bb::begin_blocker(am, sh, idg);

        // TODO: when tx fails what do we do
        for tx in block {
            Self::apply_tx(am, sh, idg, tx, allocator).unwrap();
        }
    }

    pub fn apply_tx<'a, Vm: VM, SH: StateHandler, IDG: IDGenerator, Tx: Transaction + ?Sized>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        id_generator: &mut IDG,
        tx: &Tx,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        // before tx execution flow
        Btx::validate_tx(am, sh, id_generator, tx, allocator)?;

        // handle msg
        let gas_tracker = GasTracker::limited(tx.gas_limit());
        let invoke_params = InvokeParams::new(allocator, Some(&gas_tracker));
        let resp = am.invoke_msg(sh, id_generator, tx.sender(), tx.msg(), &invoke_params);

        // after execution of the msg we pass it in to the post handler.
        PTx::after_tx_apply(am, sh, id_generator, tx, &resp)?;

        resp
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_stf() {}
}
