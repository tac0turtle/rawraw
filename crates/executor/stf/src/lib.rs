//! A state transition function that can be used to execute transactions and query state.
mod info;
mod examples;

use std::marker::PhantomData;
use crate::info::Info;

use allocator_api2::alloc::Allocator;
use ixc_account_manager::state_handler::gas::GasMeter;
use ixc_account_manager::state_handler::StateHandler;
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
    fn before_tx_apply<Vm: VM, SH: StateHandler, Tx: Transation>(
        vm: &Vm,
        sh: &mut SH,
        tx: &Tx,
        handler: &dyn RawHandler,
        alloc: &dyn Allocator,
    ) -> Result<(), ErrorCode>;
}

pub trait AfterTxApply {
    fn after_tx_apply<Vm: VM, SH: StateHandler, Tx: Transation>(
        vm: Vm,
        sh: SH,
        tx: &Tx,
        msg_result: &[u8],
    ) -> Result<(), ErrorCode>;
}

/// A state transition function that can be used to execute transactions and query state.
pub struct STF<BeforeTxApply, PostTxApply>(PhantomData<BeforeTxApply>, PhantomData<PostTxApply>);

pub enum Failure {
    BeforeTx(ErrorCode),
    ApplyMsg(ErrorCode),
    PostTx(ErrorCode),
}

impl<Btx: BeforeTxApply, PTx: AfterTxApply> STF<Btx, PTx> {
    pub const ACCOUNT_TO_HANDLER_PREFIX: u8 = 0;
    pub const fn new() -> Self {
        Self(PhantomData, PhantomData)
    }

    pub fn apply_tx<Vm: VM, SH: StateHandler, Tx: Transation>(
        vm: &Vm,
        sh: &mut SH,
        tx: &Tx,
        allocator: &dyn Allocator,
    ) -> Result<Vec<u8>, ErrorCode> {
        let handler = Self::get_handler_for_sender(tx.sender(), vm, sh, allocator)?;

        let mut gas_meter = GasMeter::new(0);

        // before tx handle
        sh.begin_tx(&mut gas_meter)?;
        Btx::before_tx_apply(vm, sh, tx, handler, allocator)?;
        sh.commit_tx(&mut gas_meter)?;

        // apply msg
        sh.begin_tx(&mut gas_meter)?;
        let resp = handler.handle_msg(vm, sh, tx)?;
        sh.commit_tx(&mut gas_meter)?;

        // post tx handle
        sh.begin_tx(&mut gas_meter)?;
        PTx::after_tx_apply(vm, sh, tx, &[])?;
        sh.commit_tx(&mut gas_meter)?;

        Ok(todo!("impl"))
    }

    fn get_handler_for_sender<'a, 'b, SH: StateHandler, Vm: VM>(
        sender: &AccountID,
        vm: &Vm,
        sh: &SH,
        alloc: &'b dyn Allocator,
    ) -> Result<&'a dyn RawHandler, ErrorCode> {
        todo!("impl")
    }

    fn new_host_backend() -> HostBackendImpl {
        todo!()
    }
}

struct HostBackendImpl;

impl HostBackend for HostBackendImpl {
    fn invoke_msg(
        &mut self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        todo!()
    }

    fn invoke_query(
        &self,
        message_packet: &mut MessagePacket,
        allocator: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_stf() {

    }
}
