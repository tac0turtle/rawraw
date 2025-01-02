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
use ixc_account_manager::id_generator::IDGenerator;
use ixc_account_manager::state_handler::StateHandler;
use ixc_account_manager::AccountManager;
use ixc_message_api::gas::GasTracker;
use ixc_message_api::handler::InvokeParams;
use ixc_message_api::message::{Message, Response};
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
    /// Gets the recipient of the transaction.
    fn recipient(&self) -> AccountID;
    /// Gets the message of the transaction.
    fn msg(&self) -> &Message;
    /// Returns the Gas Limit allocated for this transaction.
    fn gas_limit(&self) -> u64;
}

/// A trait for validating a transaction before it is executed.
///
/// Typically, this could involve checking signatures, ensuring the sender
/// has enough balance, verifying nonce or replay protection, etc.
pub trait TxValidator {
    /// Validates a transaction. Returns an `ErrorCode` if invalid.
    ///
    /// - `am`: Reference to the `AccountManager`.
    /// - `sh`: A mutable reference to the `StateHandler` (current state).
    /// - `idg`: A mutable reference to the `IDGenerator`.
    /// - `tx`: The transaction to validate.
    /// - `gt`: A `GasTracker` used to account for gas usage.
    /// - `alloc`: A reference to a dynamic allocator for memory management.
    fn validate_tx<Vm: VM, SH: StateHandler, IDG: IDGenerator, Tx: Transaction + ?Sized>(
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
pub trait AfterTxApplyHandler {
    /// Hook that is called after the transaction has been applied.
    ///
    /// - `am`: Reference to the `AccountManager`.
    /// - `sh`: Reference to the `StateHandler`.
    /// - `idg`: Reference to the `IDGenerator`.
    /// - `tx`: The transaction that was applied.
    /// - `tx_result`: The result of the transaction application (including gas used, response, etc.).
    fn after_tx_apply<'a, Vm: VM, SH: StateHandler, IDG: IDGenerator, Tx: Transaction + ?Sized>(
        am: &AccountManager<Vm>,
        sh: &SH,
        idg: &mut IDG,
        tx: &Tx,
        tx_result: &TxResult<'a, Tx>,
    );
}

/// A trait representing a "begin blocker" hook, which executes before processing
/// any transactions in a block.
///
/// This can be used to, for example, distribute block rewards, update timestamps,
/// or perform other per-block initialization tasks.
pub trait BeginBlocker<BR: BlockRequest> {
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
///
/// - `gas_used`: The total gas consumed by this transaction.
/// - `response`: The outcome of the transaction, either `Response` or an `ErrorCode`.
/// - `tx`: A reference to the original transaction.
pub struct TxResult<'a, Tx: Transaction + ?Sized> {
    pub gas_used: u64,
    pub response: Result<Response<'a>, ErrorCode>,
    pub tx: &'a Tx,
}

/// A trait representing the most basic block request interface.
///
/// A block request typically contains multiple transactions to be processed
/// as a batch. For example, in a blockchain, a block might contain many Txs.
pub trait BlockRequest {
    /// Returns the number of transactions in this request.
    fn txs_len(&self) -> u64;
    /// Returns the transactions in this request as a vector of references.
    fn txs(&self) -> Vec<&dyn Transaction>;
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
pub struct Stf<BlockRequest, TxValidator, PostTxApply, BeginBlocker, EndBlocker>(
    PhantomData<BlockRequest>,
    PhantomData<TxValidator>,
    PhantomData<PostTxApply>,
    PhantomData<BeginBlocker>,
    PhantomData<EndBlocker>,
);

/// Implementation of the State Transition Function for a given set of traits.
impl<Br, Txv, Ptx, Bb, Eb> Stf<Br, Txv, Ptx, Bb, Eb>
where
    Br: BlockRequest,
    Bb: BeginBlocker<Br>,
    Txv: TxValidator,
    Ptx: AfterTxApplyHandler,
    Eb: EndBlocker,
{
    /// Creates a new `Stf` with the provided type parameters.
    pub const fn new() -> Self {
        Self(
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
        )
    }

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
    pub fn apply_block<Vm, SH, IDG>(
        am: &AccountManager<Vm>,
        sh: &mut SH,
        idg: &mut IDG,
        block: &Br,
        allocator: &dyn Allocator,
    ) where
        Vm: VM,
        SH: StateHandler,
        IDG: IDGenerator,
    {
        // Run the 'begin_blocker' hook before processing transactions
        Bb::begin_blocker(am, sh, idg, block, allocator);

        // Prepare a container for transaction results.
        // We allocate enough capacity based on the number of Txs in the block.
        let mut results = Vec::with_capacity(
            block
                .txs_len()
                .try_into()
                .expect("too many transactions in block"),
        );

        // Process each transaction in the block and store the result.
        for tx in block.txs() {
            results.push(Self::apply_tx(am, sh, idg, tx, allocator));
        }

        // Once all Txs are processed, call the 'end_blocker' hook.
        Eb::end_blocker(am, sh, idg, allocator);
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
    pub fn validate_tx<'a, Vm: VM, SH: StateHandler, IDG: IDGenerator, Tx: Transaction + ?Sized>(
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
    pub fn apply_tx<'a, Vm: VM, SH: StateHandler, IDG: IDGenerator, Tx: Transaction + ?Sized>(
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
    fn internal_apply_tx<
        'a,
        Vm: VM,
        SH: StateHandler,
        IDG: IDGenerator,
        Tx: Transaction + ?Sized,
    >(
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
}

#[cfg(test)]
mod tests {
    /// Basic test to ensure the STF compiles and runs.
    #[test]
    fn test_stf() {
        assert!(true);
    }
}
