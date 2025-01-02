use crate::{BeginBlocker, BlockRequest};
use allocator_api2::alloc::Allocator;
use ixc_account_manager::id_generator::IDGenerator;
use ixc_account_manager::state_handler::StateHandler;
use ixc_account_manager::AccountManager;
use ixc_message_api::gas::GasTracker;
use ixc_vm_api::VM;

pub trait BlockRequestWithInfo: BlockRequest {
    fn time_unix_ns(&self) -> u64;
    fn height(&self) -> u64;
}

/// We implement a specialized BeginBlocker which stores block information.

pub struct BeginBlockerWithBlockInfo;

impl<T: BlockRequestWithInfo> BeginBlocker<T> for BeginBlockerWithBlockInfo {
    fn begin_blocker<Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        idg: &mut IDG,
        block_request: &T,
        allocator: &dyn Allocator,
    ) {
        let gt = GasTracker::unlimited();

    }
}
