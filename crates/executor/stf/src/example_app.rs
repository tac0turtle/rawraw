use std::marker::PhantomData;

// --------------------------------------------------------------------------
// 1. Bring in the STF traits and structures (update the imports to your module paths).
// --------------------------------------------------------------------------
use crate::{
    AfterTxApplyHandler, BeginBlocker, BlockRequest, EndBlocker, Stf, Transaction, TxResult,
    TxValidator,
};

use allocator_api2::alloc::Allocator;
use ixc_account_manager::{id_generator::IDGenerator, state_handler::StateHandler, AccountManager};
use ixc_message_api::gas::GasTracker;
use ixc_message_api::handler::InvokeParams;
use ixc_message_api::message::{Message, Response};
use ixc_message_api::{code::ErrorCode, AccountID};
use ixc_vm_api::VM;

// --------------------------------------------------------------------------
// 2. Implement a simple Transaction
// --------------------------------------------------------------------------
/// A basic transaction that carries a sender, recipient, a message payload, and a gas limit.
pub struct MyTransaction<'a> {
    sender: AccountID,
    recipient: AccountID,
    msg: Message<'a>,
    gas_limit: u64,
}

impl<'a> Transaction for MyTransaction<'a> {
    fn sender(&self) -> AccountID {
        self.sender.clone()
    }

    fn msg(&self) -> &Message {
        &self.msg
    }

    fn gas_limit(&self) -> u64 {
        self.gas_limit
    }
}

// --------------------------------------------------------------------------
// 3. Implement a simple BlockRequest
// --------------------------------------------------------------------------
/// A block request that stores a batch of transactions in a `Vec`.
pub struct MyBlockRequest<'a> {
    transactions: Vec<MyTransaction<'a>>,
}

impl<'a> BlockRequest<MyTransaction<'a>> for MyBlockRequest<'a> {
    fn txs(&self) -> &[MyTransaction<'a>] {
        self.transactions.as_slice()
    }
}

// --------------------------------------------------------------------------
// 4. Implement a TxValidator (pretend to validate signatures, funds, etc.)
// --------------------------------------------------------------------------
pub struct MyTxValidator;

impl<'a> TxValidator<MyTransaction<'a>> for MyTxValidator {
    fn validate_tx<Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        _am: &AccountManager<Vm>,
        _sh: &mut SH,
        _idg: &mut IDG,
        tx: &MyTransaction<'a>,
        _gt: &GasTracker,
        _alloc: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        // For example, reject if gas limit is zero.
        if tx.gas_limit() == 0 {
            return Err(ErrorCode::Unknown(0));
        }
        // Otherwise, validation passed.
        Ok(())
    }
}

// --------------------------------------------------------------------------
// 5. Implement an AfterTxApplyHandler
// --------------------------------------------------------------------------
pub struct MyAfterTxApply;

impl<'a> AfterTxApplyHandler<MyTransaction<'a>> for MyAfterTxApply {
    fn after_tx_apply<'x, Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        _am: &AccountManager<Vm>,
        _sh: &SH,
        _idg: &mut IDG,
        tx: &MyTransaction<'a>,
        tx_result: &TxResult<'x, MyTransaction<'a>>,
    ) {
        // Here you could log the result, emit events, etc.
    }
}

// --------------------------------------------------------------------------
// 6. Implement a BeginBlocker
// --------------------------------------------------------------------------
pub struct MyBeginBlocker;

impl<'a> BeginBlocker<MyTransaction<'a>, MyBlockRequest<'a>> for MyBeginBlocker {
    fn begin_blocker<Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        _am: &AccountManager<Vm>,
        _sh: &mut SH,
        _idg: &mut IDG,
        block_request: &MyBlockRequest<'a>,
        _allocator: &dyn Allocator,
    ) {
        println!("[BeginBlocker] BeginBlock called");
    }
}

// --------------------------------------------------------------------------
// 7. Implement an EndBlocker
// --------------------------------------------------------------------------
pub struct MyEndBlocker;

impl EndBlocker for MyEndBlocker {
    fn end_blocker<Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        _am: &AccountManager<Vm>,
        _sh: &mut SH,
        _idg: &mut IDG,
        _allocator: &dyn Allocator,
    ) {
        // Post-block logic (e.g., finalize block, distribute rewards, etc.)
        println!("[EndBlocker] Finishing block.");
    }
}

// --------------------------------------------------------------------------
// 8. Putting it all together
// --------------------------------------------------------------------------
/// Create our STF with the chosen transaction, block request, tx validator, hooks, etc.
pub type App<'a> = Stf<
    MyTransaction<'a>,  // Tx
    MyBlockRequest<'a>, // BlockRequest
    MyTxValidator,      // TxValidator
    MyAfterTxApply,     // AfterTxApplyHandler
    MyBeginBlocker,     // BeginBlocker
    MyEndBlocker,
>;

impl<'a> App<'a> {
    pub fn genesis() {

    }
}

#[cfg(test)]
mod tests {
    use allocator_api2::alloc::Global;
    use ixc_account_manager::AccountManager;
    use ixc_account_manager::id_generator::IncrementingIDGenerator;
    use super::{App, MyBlockRequest};
    use ixc_account_manager::native_vm::NativeVMImpl;
    use ixc_testing::store::VersionedMultiStore;
    use ixc_account_manager::state_handler::std::{StdStateHandler};

    #[test]
    fn test() {

        let vm = NativeVMImpl::default();
        let am = AccountManager::new(&vm);

        let storage = VersionedMultiStore::default();
        let mut tx = storage.new_transaction();
        let mut state = StdStateHandler::new(&mut tx, Default::default());

        let mut idg = IncrementingIDGenerator::default();

        let block = MyBlockRequest{
            transactions: vec![],
        };

        let resp = App::apply_block(
            &am,
            &mut state,
            &mut idg,
            &block,
            &Global,
        );

    }
}
