use crate::{AfterTxApply, BeforeTxApply, Transation};
use allocator_api2::alloc::Allocator;
use ixc_account_manager::state_handler::StateHandler;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::handler::RawHandler;
use ixc_vm_api::VM;
use std::marker::PhantomData;

pub struct BeforeTxApplyChain<N1: BeforeTxApply, N2: BeforeTxApply>(PhantomData<(N1, N2)>);

impl<N1: BeforeTxApply, N2: BeforeTxApply> BeforeTxApply for BeforeTxApplyChain<N1, N2> {
    fn before_tx_apply<Vm: VM, SH: StateHandler, Tx: Transation>(
        vm: &Vm,
        sh: &mut SH,
        tx: &Tx,
        handler: &dyn RawHandler,
        alloc: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        N1::before_tx_apply(vm, sh, tx, handler, alloc)?;
        N2::before_tx_apply(vm, sh, tx, handler, alloc)
    }
}

pub struct NoOpPostTxApply;

impl AfterTxApply for NoOpPostTxApply {
    fn after_tx_apply<Vm: VM, SH: StateHandler, Tx: Transation>(
        vm: &Vm,
        sh: &SH,
        tx: &Tx,
        msg_result: &[u8],
    ) -> Result<(), ErrorCode> {
        todo!()
    }
}

pub struct NoOpAuthenticator;

impl BeforeTxApply for NoOpAuthenticator {
    fn before_tx_apply<Vm: VM, SH: StateHandler, Tx: Transation>(
        vm: &Vm,
        sh: &mut SH,
        tx: &Tx,
        handler: &dyn RawHandler,
        alloc: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        todo!()
    }
}

pub struct DeductFees;

impl BeforeTxApply for DeductFees {
    fn before_tx_apply<Vm: VM, SH: StateHandler, Tx: Transation>(
        vm: &Vm,
        sh: &mut SH,
        tx: &Tx,
        handler: &dyn RawHandler,
        alloc: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        todo!()
    }
}

struct ChargeDevTax;

impl BeforeTxApply for ChargeDevTax {
    fn before_tx_apply<Vm: VM, SH: StateHandler, Tx: Transation>(
        vm: &Vm,
        sh: &mut SH,
        tx: &Tx,
        handler: &dyn RawHandler,
        alloc: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::STF;

    #[test]
    fn test() {
        let stf: STF<
            BeforeTxApplyChain<NoOpAuthenticator, BeforeTxApplyChain<DeductFees, ChargeDevTax>>,
            NoOpPostTxApply,
        > = STF::new();
    }
}
