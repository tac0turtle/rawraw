use std::marker::PhantomData;
use crate::{BeforeTxApply, Transation};
use allocator_api2::alloc::Allocator;
use ixc_account_manager::state_handler::StateHandler;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::handler::RawHandler;
use ixc_vm_api::VM;

pub trait Authenticator {
    fn authenticate<Vm: VM>(vm: &Vm) -> Result<(), ErrorCode>;
}

pub struct BeforeTxApplyAuthenticator<A: Authenticator>(PhantomData<A>);

impl<A: Authenticator> BeforeTxApply for BeforeTxApplyAuthenticator<A> {
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
