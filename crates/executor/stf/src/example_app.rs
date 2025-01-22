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
use ixc_message_api::message::Message;
use ixc_message_api::{code::ErrorCode, AccountID};
use ixc_vm_api::VM;

// --------------------------------------------------------------------------
// 2. Implement a simple Transaction
// --------------------------------------------------------------------------
/// A basic transaction that carries a sender, recipient, a message payload, and a gas limit.
pub struct MyTransaction<'a> {
    sender: AccountID,
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
        _tx: &MyTransaction<'a>,
        _tx_result: &TxResult<'x, MyTransaction<'a>>,
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
        _block_request: &MyBlockRequest<'a>,
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
    pub fn genesis() {}
}

#[ixc::handler(Echo)]
mod echo_account {
    use ixc::*;

    #[derive(Resources)]
    pub struct Echo;
    impl Echo {
        #[on_create]
        pub fn create(&self, ctx: &mut Context) -> Result<()> {
            Ok(())
        }

        #[publish]
        pub fn echo(&self, ctx: &mut Context, msg: u64) -> Result<u64> {
            Ok(msg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{echo_account, App, MyBlockRequest, MyTransaction};
    use crate::example_app::echo_account::Echo;
    use allocator_api2::alloc::Global;
    use ixc::schema::binary::NativeBinaryCodec;
    use ixc::schema::codec::Codec;
    use ixc::schema::structs::StructSchema; // imported for type selector
    use ixc_account_manager::id_generator::IncrementingIDGenerator;
    use ixc_account_manager::native_vm::{NativeVM, NativeVMImpl};
    use ixc_account_manager::state_handler::std::StdStateHandler;
    use ixc_account_manager::AccountManager;
    use ixc_core::handler::HandlerResources;
    use ixc_core::resource::{ResourceScope, Resources};
    use ixc_message_api::message::{Message, Param, Request};
    use ixc_testing::store::VersionedMultiStore;

    #[test]
    fn test() {
        let scope = ResourceScope::default();

        let mut vm = NativeVMImpl::default();
        vm.register_handler(Echo::NAME, Box::new(unsafe { Echo::new(&scope).unwrap() }));

        let am = AccountManager::new(&vm);

        let storage = VersionedMultiStore::default();
        let mut tx = storage.new_transaction();
        let mut state = StdStateHandler::new(&mut tx, Default::default());

        let mut idg = IncrementingIDGenerator::default();

        let alice = App::create_account::<_, _, _, Echo>(
            &am,
            &mut state,
            &mut idg,
            None,
            echo_account::EchoCreate {},
            None,
            &Global,
        )
        .unwrap();
        let bob = App::create_account::<_, _, _, Echo>(
            &am,
            &mut state,
            &mut idg,
            None,
            echo_account::EchoCreate {},
            None,
            &Global,
        )
        .unwrap();

        let req = echo_account::EchoEcho { msg: 200u64 };
        let req_bytes = NativeBinaryCodec {}.encode_value(&req, &Global).unwrap();

        let block = MyBlockRequest {
            transactions: vec![MyTransaction {
                sender: alice,
                msg: Message::new(
                    bob,
                    Request::new1(
                        echo_account::EchoEcho::TYPE_SELECTOR,
                        Param::from(req_bytes),
                    ),
                ),
                gas_limit: 100_000,
            }],
        };

        let mut block_resp = App::apply_block(&am, &mut state, &mut idg, &block, &Global);
        let tx_exec_resp = u64::from_le_bytes(
            block_resp
                .pop()
                .unwrap()
                .response
                .unwrap()
                .out1()
                .expect_bytes() // why bytes???
                .unwrap()
                .try_into()
                .unwrap(),
        );
        assert_eq!(200, tx_exec_resp);
    }
}
