use std::marker::PhantomData;

// --------------------------------------------------------------------------
// 1. Import the STF traits and structures (adjust to your real module paths).
// --------------------------------------------------------------------------
use crate::Stf;
use crate::{
    AfterTxApplyHandler, BeginBlocker, BlockRequest, EndBlocker, Transaction, TxResult, TxValidator,
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
struct MyTransaction<'a> {
    sender: AccountID,
    recipient: AccountID,
    msg: Message<'a>,
    gas_limit: u64,
}

impl Transaction for MyTransaction {
    fn sender(&self) -> AccountID {
        self.sender.clone()
    }

    fn recipient(&self) -> AccountID {
        self.recipient.clone()
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
struct MyBlockRequest<'a> {
    // For simplicity, store transactions in a Vec. They could come from anywhere (e.g., network).
    transactions: Vec<MyTransaction<'a>>,
}

impl BlockRequest for MyBlockRequest {
    fn txs_len(&self) -> u64 {
        self.transactions.len() as u64
    }

    // Return each transaction as a reference to something that implements `Transaction`.
    fn txs(&self) -> Vec<&dyn Transaction> {
        self.transactions
            .iter()
            .map(|tx| tx as &dyn Transaction)
            .collect()
    }
}

// --------------------------------------------------------------------------
// 4. Implement a TxValidator (pretend to validate signatures, funds, etc.)
// --------------------------------------------------------------------------
struct MyTxValidator;

impl TxValidator for MyTxValidator {
    fn validate_tx<Vm: VM, SH: StateHandler, IDG: IDGenerator, Tx: Transaction + ?Sized>(
        _am: &AccountManager<Vm>,
        _sh: &mut SH,
        _idg: &mut IDG,
        tx: &Tx,
        _gt: &GasTracker,
        _alloc: &dyn Allocator,
    ) -> Result<(), ErrorCode> {
        // For example, reject if gas limit is 0.
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
struct MyAfterTxApply;

impl AfterTxApplyHandler for MyAfterTxApply {
    fn after_tx_apply<'a, Vm: VM, SH: StateHandler, IDG: IDGenerator, Tx: Transaction + ?Sized>(
        _am: &AccountManager<Vm>,
        _sh: &SH,
        _idg: &mut IDG,
        tx: &Tx,
        tx_result: &TxResult<'a, Tx>,
    ) {
        // do logging or event emission etc.
    }
}

// --------------------------------------------------------------------------
// 6. Implement a BeginBlocker
// --------------------------------------------------------------------------
struct MyBeginBlocker;

impl<BR: BlockRequest> BeginBlocker<BR> for MyBeginBlocker {
    fn begin_blocker<Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        _am: &AccountManager<Vm>,
        _sh: &mut SH,
        _idg: &mut IDG,
        block_request: &BR,
        _allocator: &dyn Allocator,
    ) {
        // For example, we might log something or distribute block rewards.
        // We'll just do a simple print here.
        println!(
            "BeginBlocker: starting new block with {} txs.",
            block_request.txs_len()
        );
    }
}

// --------------------------------------------------------------------------
// 7. Implement an EndBlocker
// --------------------------------------------------------------------------
struct MyEndBlocker;

impl EndBlocker for MyEndBlocker {
    fn end_blocker<Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        _am: &AccountManager<Vm>,
        _sh: &mut SH,
        _idg: &mut IDG,
        _allocator: &dyn Allocator,
    ) {
        // Post-block logic (e.g., finalize block, distribute staking rewards, etc.)
        println!("EndBlocker: finishing block.");
    }
}

// --------------------------------------------------------------------------
// 8. Putting it all together in main()
// --------------------------------------------------------------------------

const STF: Stf<MyBlockRequest, MyTxValidator, MyAfterTxApply, MyBeginBlocker, MyEndBlocker> =
    Stf::new();
fn main() {}
