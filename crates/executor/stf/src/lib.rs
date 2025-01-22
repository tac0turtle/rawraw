//! A state transition function that can be used to execute transactions and query state.
//!
//! # Overview
//!
//! This module defines a generic State Transition Function (STF) that processes
//! blocks of transactions in a blockchain-like setting. The STF supports a flow
//! with:
//! - A `BeginBlocker` that runs before any transactions in a block.
//! - Transaction validation and execution through a `TxValidator` and transaction application methods.
//! - An `AfterTxApplyHandler` that runs post-transaction.
//! - An `EndBlocker` that runs after all transactions in a block.
//!
//! The `Stf` struct ties together these components, enabling modular handling of
//! how transactions are validated, applied, and how the system state is updated.
//! It also supports features like gas usage tracking and response encoding.

mod block_info;
#[cfg(test)]
mod example_app;

use allocator_api2::alloc::Allocator;
use ixc::schema::binary::NativeBinaryCodec;
use ixc::schema::codec::Codec;
use ixc_account_manager::id_generator::IDGenerator;
use ixc_account_manager::state_handler::StateHandler;
use ixc_account_manager::AccountManager;
use ixc_core::handler::Handler;
use ixc_message_api::gas::GasTracker;
use ixc_message_api::handler::InvokeParams;
use ixc_message_api::message::{Message, Param, Request, Response};
use ixc_message_api::{code::ErrorCode, AccountID};
use ixc_vm_api::VM;
use std::marker::PhantomData;

/// A trait representing a transaction.
///
/// This trait defines the minimal interface for a transaction:
/// - `sender()`: The account initiating the transaction.
/// - `recipient()`: The target account or contract.
/// - `msg()`: The message payload containing the call data or instruction.
/// - `gas_limit()`: The maximum gas units the sender is willing to spend.
pub trait Transaction {
    /// Gets the sender of the transaction.
    fn sender(&self) -> AccountID;
    /// Gets the message of the transaction.
    fn msg(&self) -> &Message;
    /// Returns the Gas Limit allocated for this transaction.
    fn gas_limit(&self) -> u64;
}

/// A trait for validating a transaction before it is executed.
///
/// Typically, this could involve checking signatures, ensuring the sender
/// has enough balance, verifying nonce or replay protection, etc.
pub trait TxValidator<Tx: Transaction> {
    /// Validates a transaction. Returns an `ErrorCode` if invalid.
    ///
    /// - `am`: Reference to the `AccountManager`.
    /// - `sh`: A mutable reference to the `StateHandler` (current state).
    /// - `idg`: A mutable reference to the `IDGenerator`.
    /// - `tx`: The transaction to validate.
    /// - `gt`: A `GasTracker` used to account for gas usage.
    /// - `alloc`: A reference to a dynamic allocator for memory management.
    fn validate_tx<Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        idg: &mut IDG,
        tx: &Tx,
        gt: &GasTracker,
        alloc: &dyn Allocator,
    ) -> Result<(), ErrorCode>;
}

/// A trait for handling events after a transaction has been applied.
///
/// This can be used to perform cleanup, logging, or other side effects that
/// only occur after the transaction has been executed (e.g., updating block
/// explorers, event emission, etc.).
pub trait AfterTxApplyHandler<Tx: Transaction> {
    /// Hook that is called after the transaction has been applied.
    ///
    /// - `am`: Reference to the `AccountManager`.
    /// - `sh`: Reference to the `StateHandler`.
    /// - `idg`: Reference to the `IDGenerator`.
    /// - `tx`: The transaction that was applied.
    /// - `tx_result`: The result of the transaction application (including gas used, response, etc.).
    fn after_tx_apply<Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        am: &AccountManager<Vm>,
        sh: &SH,
        idg: &mut IDG,
        tx: &Tx,
        tx_result: &TxResult<'_, Tx>,
    );
}

/// A trait representing a "begin blocker" hook, which executes before processing
/// any transactions in a block.
///
/// This can be used to, for example, distribute block rewards, update timestamps,
/// or perform other per-block initialization tasks.
pub trait BeginBlocker<Tx: Transaction, BR: BlockRequest<Tx>> {
    /// Called before any transactions of the block are processed.
    ///
    /// - `am`: Reference to the `AccountManager`.
    /// - `sh`: A mutable reference to the `StateHandler`.
    /// - `idg`: A mutable reference to the `IDGenerator`.
    /// - `block_request`: The block data (including transactions).
    /// - `allocator`: A reference to a memory allocator.
    fn begin_blocker<Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        idg: &mut IDG,
        block_request: &BR,
        allocator: &dyn Allocator,
    );
}

/// A trait representing an "end blocker" hook, which executes after processing
/// all transactions in a block.
///
/// This can be used to finalize the block, distribute staking rewards, or clean
/// up temporary state.
pub trait EndBlocker {
    /// Called after all transactions of the block are processed.
    ///
    /// - `am`: Reference to the `AccountManager`.
    /// - `sh`: A mutable reference to the `StateHandler`.
    /// - `idg`: A mutable reference to the `IDGenerator`.
    /// - `allocator`: A reference to a memory allocator.
    fn end_blocker<Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        idg: &mut IDG,
        allocator: &dyn Allocator,
    );
}

/// Encapsulates the result of applying a single transaction.
pub struct TxResult<'a, Tx: Transaction> {
    /// The total gas consumed by this transaction.
    pub gas_used: u64,
    /// The outcome of the transaction, either `Response` or an `ErrorCode`
    pub response: Result<Response<'a>, ErrorCode>,
    /// A reference to the original transaction.
    pub tx: &'a Tx,
}

/// A trait representing the most basic block request interface.
///
/// A block request typically contains multiple transactions to be processed
/// as a batch. For example, in a blockchain, a block might contain many Txs.
pub trait BlockRequest<Tx: Transaction> {
    /// Returns the transactions in this request as a vector of references.
    fn txs(&self) -> &[Tx];
}

/// A State Transition Function (STF) that coordinates how blocks and transactions
/// are processed.
///
/// `Stf` is parameterized by the following types:
/// - `BlockRequest`: The structure that contains transactions (e.g., a block).
/// - `BeforeTxApply`: A `TxValidator` for validating transactions.
/// - `PostTxApply`: An `AfterTxApplyHandler` for post-processing after Tx is applied.
/// - `BeginBlocker`: The block initialization trait.
/// - `EndBlocker`: The block finalization trait.
pub struct Stf<Tx, BlockRequest, TxValidator, PostTxApply, BeginBlocker, EndBlocker>(
    PhantomData<(
        Tx,
        BlockRequest,
        TxValidator,
        PostTxApply,
        BeginBlocker,
        EndBlocker,
    )>,
);

/// Implementation of the State Transition Function for a given set of traits.
impl<Tx, Br, Txv, Ptx, Bb, Eb> Stf<Tx, Br, Txv, Ptx, Bb, Eb>
where
    Tx: Transaction,
    Br: BlockRequest<Tx>,
    Bb: BeginBlocker<Tx, Br>,
    Txv: TxValidator<Tx>,
    Ptx: AfterTxApplyHandler<Tx>,
    Eb: EndBlocker,
{
    /// Applies an entire block by:
    /// 1. Calling the `begin_blocker`.
    /// 2. Iterating over each transaction and applying it.
    /// 3. Calling the `end_blocker`.
    ///
    /// - `am`: Reference to the `AccountManager`.
    /// - `sh`: A mutable reference to the state handler.
    /// - `idg`: A mutable reference to the ID generator.
    /// - `block`: The block containing transactions.
    /// - `allocator`: A memory allocator reference.
    pub fn apply_block<'a, Vm, SH, IDG>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        idg: &mut IDG,
        block: &'a Br,
        allocator: &'a dyn Allocator,
    ) -> Vec<TxResult<'a, Tx>>
    where
        Vm: VM,
        SH: StateHandler,
        IDG: IDGenerator,
    {
        // Run the 'begin_blocker' hook before processing transactions
        Bb::begin_blocker(am, sh, idg, block, allocator);

        // Prepare a container for transaction results.
        // We allocate enough capacity based on the number of Txs in the block.
        let txs = block.txs();
        let mut results = Vec::with_capacity(txs.len());

        // Process each transaction in the block and store the result.
        for tx in block.txs() {
            results.push(Self::apply_tx(am, sh, idg, tx, allocator));
        }

        // Once all Txs are processed, call the 'end_blocker' hook.
        Eb::end_blocker(am, sh, idg, allocator);
        results
    }

    /// Validates a single transaction without executing it.
    ///
    /// - `am`: Reference to the `AccountManager`.
    /// - `sh`: A mutable reference to the state handler.
    /// - `idg`: A mutable reference to the ID generator.
    /// - `tx`: The transaction to validate.
    /// - `allocator`: A memory allocator reference.
    ///
    /// Returns a `TxResult` containing the outcome of the validation.
    pub fn validate_tx<'a, Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        idg: &mut IDG,
        tx: &'a Tx,
        allocator: &'a dyn Allocator,
    ) -> TxResult<'a, Tx> {
        // Create a default GasTracker for validation phase.
        let gt = GasTracker::default();

        // Run the validation logic from the TxValidator trait.
        let resp = Txv::validate_tx(am, sh, idg, tx, &gt, allocator);

        // Construct and return the TxResult. If validation succeeds, it includes an empty `Response`.
        TxResult {
            gas_used: gt.consumed.into_inner(),
            response: resp.map(|_| Response::new()),
            tx,
        }
    }

    /// Validates and then applies a single transaction.
    ///
    /// - `am`: Reference to the `AccountManager`.
    /// - `sh`: A mutable reference to the state handler.
    /// - `id_generator`: A mutable reference to the ID generator.
    /// - `tx`: The transaction to apply.
    /// - `allocator`: A memory allocator reference.
    ///
    /// Returns a `TxResult` containing the outcome of the transaction.
    pub fn apply_tx<'a, Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        id_generator: &mut IDG,
        tx: &'a Tx,
        allocator: &'a dyn Allocator,
    ) -> TxResult<'a, Tx> {
        // Create a GasTracker with a limit specified by the transaction.
        let gas_tracker = GasTracker::limited(tx.gas_limit());

        // Internally validate and execute the transaction.
        let resp = Self::internal_apply_tx(am, sh, id_generator, tx, &gas_tracker, allocator);

        // Build the transaction result.
        let tx_result = TxResult {
            gas_used: gas_tracker.consumed.into_inner(),
            response: resp,
            tx,
        };

        // Hook for any post-transaction processing.
        Ptx::after_tx_apply(am, sh, id_generator, tx, &tx_result);

        tx_result
    }

    /// Internal helper function that validates and executes the transaction.
    ///
    /// - `am`: Reference to the `AccountManager`.
    /// - `sh`: A mutable reference to the state handler.
    /// - `id_generator`: A mutable reference to the ID generator.
    /// - `tx`: The transaction to apply.
    /// - `gas_tracker`: A reference to the GasTracker for accounting gas usage.
    /// - `allocator`: A memory allocator reference.
    ///
    /// Returns a `Result` containing either a `Response` or an `ErrorCode`.
    fn internal_apply_tx<'a, Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        id_generator: &mut IDG,
        tx: &Tx,
        gas_tracker: &GasTracker,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        // First validate the transaction. If validation fails, return the error immediately.
        Txv::validate_tx(am, sh, id_generator, tx, gas_tracker, allocator)?;

        // If validation succeeds, invoke the message (execute it on the VM).
        let invoke_params = InvokeParams::new(allocator, Some(gas_tracker));
        am.invoke_msg(sh, id_generator, tx.sender(), tx.msg(), &invoke_params)
    }

    /// Allows any caller to execute something with sudo permissions
    /// It allows it to impersonate any account.
    pub fn sudo<'a, Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        id_generator: &mut IDG,
        sender: AccountID,
        msg: &Message,
        gas_limit: Option<u64>,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        let gt = match gas_limit {
            Some(gt) => GasTracker::limited(gt),
            None => GasTracker::unlimited(),
        };

        let invoke_params = InvokeParams::new(allocator, Some(&gt));
        am.invoke_msg(sh, id_generator, sender, msg, &invoke_params)
    }

    const CREATE_SELECTOR: u64 = 4843642167467229819; // hacked from expansion

    /// Creates an account. It is a SUDO operation.
    pub fn create_account<'a, Vm: VM, SH: StateHandler, IDG: IDGenerator, H: Handler>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        id_generator: &mut IDG,
        sender: Option<AccountID>, // if none defaults to root.
        msg: H::Init<'_>,
        gas_limit: Option<u64>,
        allocator: &'a dyn Allocator,
    ) -> Result<AccountID, ErrorCode> {
        let gt = match gas_limit {
            Some(gt) => GasTracker::limited(gt),
            None => GasTracker::unlimited(),
        };

        let msg_bytes = NativeBinaryCodec {}.encode_value(&msg, allocator)?;

        let invoke_params = InvokeParams::new(allocator, Some(&gt));

        let request = Request::new2(
            Self::CREATE_SELECTOR,
            Param::from(H::NAME),
            Param::from(msg_bytes),
        );
        let msg = Message::new(ixc_message_api::ROOT_ACCOUNT, request);
        am.invoke_msg(
            sh,
            id_generator,
            sender.unwrap_or(ixc_message_api::ROOT_ACCOUNT),
            &msg,
            &invoke_params,
        )
        .map(|r| r.out1().expect_account_id().unwrap())
    }

    /// Performs a readonly query on the account.
    pub fn query<'a, Vm: VM, SH: StateHandler, IDG: IDGenerator>(
        am: &AccountManager<Vm>,
        sh: &SH,
        msg: &Message,
        gas_limit: Option<u64>,
        allocator: &'a dyn Allocator,
    ) -> Result<Response<'a>, ErrorCode> {
        let gt = match gas_limit {
            Some(gt) => GasTracker::limited(gt),
            None => GasTracker::unlimited(),
        };
        let invoke_params = InvokeParams::new(allocator, Some(&gt));
        am.invoke_query(sh, msg, &invoke_params)
    }
}

#[cfg(test)]
mod tests {
    /// Basic test to ensure the STF compiles and runs.
    #[test]
    fn test_stf() {
        assert!(true);
    }
}
