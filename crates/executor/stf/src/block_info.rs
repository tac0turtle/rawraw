use crate::{BeginBlocker, BlockRequest, Transaction};
use allocator_api2::alloc::Allocator;
use ixc_account_manager::id_generator::IDGenerator;
use ixc_account_manager::state_handler::StateHandler;
use ixc_account_manager::AccountManager;
use ixc_message_api::gas::GasTracker;
use ixc_vm_api::VM;

/// Defines a BlockRequest which exposes time and height information.
pub trait BlockRequestWithInfo<Tx: Transaction>: BlockRequest<Tx> {
    /// The time as unix nanoseconds.
    fn time_unix_ns(&self) -> u64;
    /// The height of the block being processed.
    fn height(&self) -> u64;
}

/// We implement a specialized BeginBlocker which stores block information.

pub struct BeginBlockerWithBlockInfo;

impl<Tx: Transaction, T: BlockRequestWithInfo<Tx>> BeginBlocker<Tx, T>
    for BeginBlockerWithBlockInfo
{
    fn begin_blocker<Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        _am: &AccountManager<Vm>,
        _sh: &mut SH,
        _idg: &mut IDG,
        _block_request: &T,
        _allocator: &dyn Allocator,
    ) {
        let _gt = GasTracker::unlimited();
    }
}
