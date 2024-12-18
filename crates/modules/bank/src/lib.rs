//! Bank moduleodule that allows for the transfer of tokens between accounts.
#![allow(clippy::needless_lifetimes)]
/// The bank module used for transfering tokens between accounts.
#[ixc::handler(Bank)]
pub mod bank {
    use ixc::*;
    use ixc_core::error::unimplemented_ok;
    use ixc_core::handler::Service;
    use mockall::automock;

    /// The bank module used for transfering tokens between accounts.
    #[derive(Resources)]
    pub struct Bank {
        #[state(prefix = 1, key(address, denom), value(amount))]
        pub(crate) balances: AccumulatorMap<(AccountID, Str)>,
        #[state(prefix = 2, key(denom), value(total))]
        pub(crate) supply: AccumulatorMap<Str>,
        #[state(prefix = 3)]
        super_admin: Item<AccountID>,
        #[state(prefix = 4)]
        global_send_hook: Item<AccountID>,
        #[state(prefix = 5)]
        denom_admins: Map<Str, AccountID>,
        #[state(prefix = 6)]
        denom_send_hooks: Map<Str, AccountID>,
        #[state(prefix = 6)]
        denom_burn_hooks: Map<Str, AccountID>,
    }

    /// A coin is a token with a denom and an amount.
    #[derive(SchemaValue, Clone, Default)]
    #[sealed]
    pub struct Coin<'a> {
        /// The denom of the coin.
        pub denom: &'a str,
        /// The amount of the coin.
        pub amount: u128,
    }

    /// The API of the bank module.
    #[handler_api]
    pub trait BankAPI {
        /// Get the balance of an account for a specific denom.
        fn get_balance(&self, ctx: &Context, account: AccountID, denom: &str) -> Result<u128>;
        /// Send coins to another account.
        fn send<'a>(
            &self,
            ctx: &'a mut Context,
            to: AccountID,
            amount: &[Coin<'a>],
            evt: EventBus<EventSend<'_>>,
        ) -> Result<()>;
        /// Mint coins for an account.
        fn mint(
            &self,
            ctx: &mut Context,
            to: AccountID,
            denom: &str,
            amount: u128,
            evt: EventBus<EventMint<'_>>,
        ) -> Result<()>;
        /// Burn coins from an account.
        fn burn(
            &self,
            ctx: &mut Context,
            from: AccountID,
            denom: &str,
            amount: u128,
            evt: EventBus<EventBurn<'_>>,
        ) -> Result<()>;
    }

    /// The send hook is called when a coin is sent to another account.
    #[handler_api]
    #[automock]
    pub trait SendHook {
        /// Called when a coin is sent to another account.
        fn on_send<'a>(
            &self,
            ctx: &mut Context<'a>,
            from: AccountID,
            to: AccountID,
            denom: &str,
            amount: u128,
        ) -> Result<()>;
    }

    /// The burn hook is called when a coin is burned from an account.
    #[handler_api]
    #[automock]
    pub trait BurnHook {
        /// Called when a coin is burned from an account.
        fn on_burn<'a>(
            &self,
            ctx: &mut Context<'a>,
            from: AccountID,
            denom: &str,
            amount: u128,
        ) -> Result<()>;
    }

    /// The receive hook is called when a coin is received by an account.
    #[handler_api]
    #[automock]
    pub trait ReceiveHook {
        /// Called when a coin is received by an account.
        fn on_receive<'a>(
            &self,
            ctx: &mut Context<'a>,
            from: AccountID,
            denom: &str,
            amount: u128,
        ) -> Result<()>;
    }

    /// An event emitted when a coin is sent to another account.
    #[derive(SchemaValue, Clone, Default)]
    #[non_exhaustive]
    pub struct EventSend<'a> {
        /// The account that sent the coin.
        pub from: AccountID,
        /// The account that received the coin.
        pub to: AccountID,
        /// The coin that was sent.
        pub coin: Coin<'a>,
    }

    /// An event emitted when a coin is minted for an account.
    #[derive(SchemaValue, Clone, Default)]
    #[non_exhaustive]
    pub struct EventMint<'a> {
        /// The account that minted the coin.
        pub to: AccountID,
        /// The coin that was minted.
        pub coin: Coin<'a>,
    }

    /// An event emitted when a coin is burned from an account.
    #[derive(SchemaValue, Default, Clone)]
    #[non_exhaustive]
    pub struct EventBurn<'a> {
        /// The account that burned the coin.
        pub from: AccountID,
        /// The coin that was burned.
        pub coin: Coin<'a>,
    }

    impl Bank {
        /// Called when the bank module is created.
        #[on_create]
        pub fn create(&self, ctx: &mut Context) -> Result<()> {
            self.super_admin.set(ctx, ctx.caller())?;
            Ok(())
        }

        /// Create a new denom.
        #[publish]
        pub fn create_denom(&self, ctx: &mut Context, denom: &str, admin: AccountID) -> Result<()> {
            ensure!(self.super_admin.get(ctx)? == ctx.caller(), "not authorized");
            self.denom_admins.set(ctx, denom, admin)?;
            Ok(())
        }

        /// Set the global send hook.
        #[publish]
        pub fn set_global_send_hook(&self, ctx: &mut Context, hook: AccountID) -> Result<()> {
            ensure!(self.super_admin.get(ctx)? == ctx.caller(), "not authorized");
            self.global_send_hook.set(ctx, hook)?;
            Ok(())
        }

        /// Set the denom burn hook.
        #[publish]
        pub fn set_denom_send_hook(
            &self,
            ctx: &mut Context,
            denom: &str,
            hook: AccountID,
        ) -> Result<()> {
            // Only denom admin can set send hooks
            let admin = self
                .denom_admins
                .get(ctx, denom)?
                .ok_or(error!("denom not defined"))?;
            ensure!(admin == ctx.caller(), "not authorized");
            self.denom_send_hooks.set(ctx, denom, hook)?;
            Ok(())
        }
        /// Set the denom burn hook.
        #[publish]
        pub fn set_denom_burn_hook(
            &self,
            ctx: &mut Context,
            denom: &str,
            hook: AccountID,
        ) -> Result<()> {
            // Only denom admin can set burn hooks
            let admin = self
                .denom_admins
                .get(ctx, denom)?
                .ok_or(error!("denom not defined"))?;
            ensure!(admin == ctx.caller(), "not authorized");
            self.denom_burn_hooks.set(ctx, denom, hook)?;
            Ok(())
        }
    }

    #[publish]
    impl BankAPI for Bank {
        fn get_balance(&self, ctx: &Context, account: AccountID, denom: &str) -> Result<u128> {
            let amount = self.balances.get(ctx, (account, denom))?;
            Ok(amount)
        }

        fn send<'a>(
            &self,
            ctx: &'a mut Context,
            to: AccountID,
            amount: &[Coin<'a>],
            mut evt: EventBus<EventSend<'a>>,
        ) -> Result<()> {
            let global_send = self.global_send_hook.get(ctx)?;
            for coin in amount {
                if !global_send.is_empty() {
                    let hook_client = <dyn SendHook>::new_client(global_send);
                    hook_client.on_send(ctx, ctx.caller(), to, coin.denom, coin.amount)?;
                }
                if let Some(hook) = self.denom_send_hooks.get(ctx, coin.denom)? {
                    let hook_client = <dyn SendHook>::new_client(hook);
                    hook_client.on_send(ctx, ctx.caller(), to, coin.denom, coin.amount)?;
                }
                let from = ctx.caller();
                let receive_hook = <dyn ReceiveHook>::new_client(to);
                unimplemented_ok(receive_hook.on_receive(ctx, from, coin.denom, coin.amount))?;
                self.balances
                    .safe_sub(ctx, (from, coin.denom), coin.amount)?;
                self.balances.add(ctx, (to, coin.denom), coin.amount)?;
                evt.emit(
                    ctx,
                    &EventSend {
                        from,
                        to,
                        coin: coin.clone(),
                    },
                )?;
            }
            Ok(())
        }

        fn mint<'a>(
            &self,
            ctx: &mut Context,
            to: AccountID,
            denom: &'a str,
            amount: u128,
            mut evt: EventBus<EventMint<'a>>,
        ) -> Result<()> {
            let admin = self
                .denom_admins
                .get(ctx, denom)?
                .ok_or(error!("denom not defined"))?;
            ensure!(admin == ctx.caller(), "not authorized");
            self.supply.add(ctx, denom, amount)?;
            self.balances.add(ctx, (to, denom), amount)?;
            evt.emit(
                ctx,
                &EventMint {
                    to,
                    coin: Coin { denom, amount },
                },
            )?;
            Ok(())
        }

        fn burn<'a>(
            &self,
            ctx: &mut Context,
            from: AccountID,
            denom: &'a str,
            amount: u128,
            mut evt: EventBus<EventBurn<'a>>,
        ) -> Result<()> {
            // Check if the caller is authorized to burn
            // Only denom admin or the token owner can burn
            let admin = self
                .denom_admins
                .get(ctx, denom)?
                .ok_or(error!("denom not defined"))?;
            ensure!(
                admin == ctx.caller() || from == ctx.caller(),
                "not authorized to burn tokens"
            );

            // Check if there are any burn hooks and execute them
            if let Some(hook) = self.denom_burn_hooks.get(ctx, denom)? {
                let hook_client = <dyn BurnHook>::new_client(hook);
                hook_client.on_burn(ctx, from, denom, amount)?;
            }

            // Verify sufficient balance and reduce it

            self.balances.safe_sub(ctx, (from, denom), amount)?;

            // Reduce total supply
            self.supply.safe_sub(ctx, denom, amount)?;

            // Emit burn event
            evt.emit(
                ctx,
                &EventBurn {
                    from,
                    coin: Coin { denom, amount },
                },
            )?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bank::*;
    use ixc_core::account_api::ROOT_ACCOUNT;
    use ixc_testing::*;

    #[test]
    fn test() {
        // initialize the app
        let app = TestApp::default();
        // register the Bank handler
        app.register_handler::<Bank>().unwrap();

        // create a new client context for the root account and initialize bank
        let mut root = app.client_context_for(ROOT_ACCOUNT);
        let bank_client = create_account::<Bank>(&mut root, BankCreate {}).unwrap();

        // register a mock global send hook to test that it is called
        let mut mock_global_send_hook = MockSendHook::new();
        // expect that the send hook is only called 1x in this test
        mock_global_send_hook
            .expect_on_send()
            .times(1)
            .returning(|_, _, _, _, _| Ok(()));
        let mut mock = MockHandler::new();
        mock.add_handler::<dyn SendHook>(Box::new(mock_global_send_hook));
        let mock_id = app.add_mock(mock).unwrap();
        bank_client
            .set_global_send_hook(&mut root, mock_id)
            .unwrap();

        // alice gets to manage the "foo" denom and mints herself 1000 foo coins
        let mut alice = app.new_client_context().unwrap();
        let alice_id = alice.self_account_id();
        bank_client
            .create_denom(&mut root, "foo", alice_id)
            .unwrap();
        bank_client.mint(&mut alice, alice_id, "foo", 1000).unwrap();

        // ensure alice has 1000 foo coins
        let alice_balance = bank_client.get_balance(&alice, alice_id, "foo").unwrap();
        assert_eq!(alice_balance, 1000);

        // alice sends 100 foo coins to bob
        let bob = app.new_client_context().unwrap();
        bank_client
            .send(
                &mut alice,
                bob.self_account_id(),
                &[Coin {
                    denom: "foo",
                    amount: 100,
                }],
            )
            .unwrap();

        // ensure alice has 900 foo coins and bob has 100 foo coins
        let alice_balance = bank_client
            .get_balance(&alice, alice.self_account_id(), "foo")
            .unwrap();
        assert_eq!(alice_balance, 900);
        let bob_balance = bank_client
            .get_balance(&bob, bob.self_account_id(), "foo")
            .unwrap();
        assert_eq!(bob_balance, 100);

        // look inside bank to check the balance of alice directly as well as the supply of foo
        app.exec_in(&bank_client, |bank, ctx| {
            let alice_balance = bank.balances.get(ctx, (alice_id, "foo")).unwrap();
            assert_eq!(alice_balance, 900);
            let foo_supply = bank.supply.get(ctx, "foo").unwrap();
            assert_eq!(foo_supply, 1000);
        })
    }

    #[test]
    fn test_burn() {
        let app = TestApp::default();
        app.register_handler::<Bank>().unwrap();

        // Initialize with root account
        let mut root = app.client_context_for(ROOT_ACCOUNT);
        let bank_client = create_account::<Bank>(&mut root, BankCreate {}).unwrap();

        // Set up Alice as denom admin
        let mut alice = app.new_client_context().unwrap();
        let alice_id = alice.self_account_id();
        bank_client
            .create_denom(&mut root, "foo", alice_id)
            .unwrap();

        // Mint tokens to Bob
        let mut bob = app.new_client_context().unwrap();
        let bob_id = bob.self_account_id();
        bank_client.mint(&mut alice, bob_id, "foo", 1000).unwrap();

        // Set up burn hook
        let mut mock_burn_hook = MockBurnHook::new();
        mock_burn_hook
            .expect_on_burn()
            .times(1)
            .returning(|_, _, _, _| Ok(()));
        let mut mock = MockHandler::new();
        mock.add_handler::<dyn BurnHook>(Box::new(mock_burn_hook));
        let mock_id = app.add_mock(mock).unwrap();

        // Set burn hook
        bank_client
            .set_denom_burn_hook(&mut alice, "foo", mock_id)
            .unwrap();

        // Test burn by token holder
        bank_client.burn(&mut bob, bob_id, "foo", 300).unwrap();

        // Verify balance
        let bob_balance = bank_client.get_balance(&bob, bob_id, "foo").unwrap();
        assert_eq!(bob_balance, 700);

        // Verify total supply
        app.exec_in(&bank_client, |bank, ctx| {
            let foo_supply = bank.supply.get(ctx, "foo").unwrap();
            assert_eq!(foo_supply, 700);
        });

        // Test burn by admin
        bank_client.burn(&mut alice, bob_id, "foo", 200).unwrap();

        // Verify final balance and supply
        let bob_balance = bank_client.get_balance(&bob, bob_id, "foo").unwrap();
        assert_eq!(bob_balance, 500);

        app.exec_in(&bank_client, |bank, ctx| {
            let foo_supply = bank.supply.get(ctx, "foo").unwrap();
            assert_eq!(foo_supply, 500);
        });
    }

    #[test]
    fn test_burn_unauthorized() {
        let app = TestApp::default();
        app.register_handler::<Bank>().unwrap();

        // Initialize bank
        let mut root = app.client_context_for(ROOT_ACCOUNT);
        let bank_client = create_account::<Bank>(&mut root, BankCreate {}).unwrap();

        // Set up Alice as denom admin
        let mut alice = app.new_client_context().unwrap();
        let alice_id = alice.self_account_id();
        bank_client
            .create_denom(&mut root, "foo", alice_id)
            .unwrap();

        // Mint tokens to Bob
        let bob = app.new_client_context().unwrap();
        let bob_id = bob.self_account_id();
        bank_client.mint(&mut alice, bob_id, "foo", 1000).unwrap();

        // Try to burn with unauthorized account (Charlie)
        let mut charlie = app.new_client_context().unwrap();
        let result = bank_client.burn(&mut charlie, bob_id, "foo", 100);
        assert!(result.is_err());
    }
    #[test]
    fn test_mint_with_hooks() {
        let app = TestApp::default();
        app.register_handler::<Bank>().unwrap();

        // Initialize bank with root account
        let mut root = app.client_context_for(ROOT_ACCOUNT);
        let bank_client = create_account::<Bank>(&mut root, BankCreate {}).unwrap();

        // Set up Alice as denom admin
        let mut alice = app.new_client_context().unwrap();
        let alice_id = alice.self_account_id();
        bank_client
            .create_denom(&mut root, "foo", alice_id)
            .unwrap();

        // Set up Bob's account for receiving mints
        let bob = app.new_client_context().unwrap();
        let bob_id = bob.self_account_id();

        // Set up a mock receive hook for Bob
        let mut mock_receive_hook = MockReceiveHook::new();
        mock_receive_hook
            .expect_on_receive()
            .times(2) // We'll mint twice
            .returning(|_, _, _, _| Ok(()));
        let mut mock = MockHandler::new();
        mock.add_handler::<dyn ReceiveHook>(Box::new(mock_receive_hook));

        // Create mock account and associate it with Bob's ID
        let mock_id = app.add_mock(mock).unwrap();
        let bob_with_hook = app.client_context_for(mock_id);

        // Test successful mint by denom admin
        bank_client.mint(&mut alice, mock_id, "foo", 500).unwrap();

        // Verify initial balance and supply
        let bob_balance = bank_client
            .get_balance(&bob_with_hook, mock_id, "foo")
            .unwrap();
        assert_eq!(bob_balance, 500);

        app.exec_in(&bank_client, |bank, ctx| {
            let foo_supply = bank.supply.get(ctx, "foo").unwrap();
            assert_eq!(foo_supply, 500);
        });

        // Test second mint
        bank_client.mint(&mut alice, mock_id, "foo", 300).unwrap();

        // Verify updated balance and supply
        let bob_balance = bank_client
            .get_balance(&bob_with_hook, mock_id, "foo")
            .unwrap();
        assert_eq!(bob_balance, 800);

        app.exec_in(&bank_client, |bank, ctx| {
            let foo_supply = bank.supply.get(ctx, "foo").unwrap();
            assert_eq!(foo_supply, 800);
        });
    }

    #[test]
    fn test_mint_unauthorized() {
        let app = TestApp::default();
        app.register_handler::<Bank>().unwrap();

        // Initialize bank
        let mut root = app.client_context_for(ROOT_ACCOUNT);
        let bank_client = create_account::<Bank>(&mut root, BankCreate {}).unwrap();

        // Set up Alice as denom admin
        let alice = app.new_client_context().unwrap();
        let alice_id = alice.self_account_id();
        bank_client
            .create_denom(&mut root, "foo", alice_id)
            .unwrap();

        // Try to mint with unauthorized account (Bob)
        let mut bob = app.new_client_context().unwrap();
        let bob_id = bob.self_account_id();

        // Bob tries to mint to himself
        let result = bank_client.mint(&mut bob, bob_id, "foo", 1000);
        assert!(result.is_err());

        // Verify no balance was created
        let bob_balance = bank_client.get_balance(&bob, bob_id, "foo").unwrap();
        assert_eq!(bob_balance, 0);

        // Verify supply wasn't affected
        app.exec_in(&bank_client, |bank, ctx| {
            let foo_supply = bank.supply.get(ctx, "foo").unwrap();
            assert_eq!(foo_supply, 0);
        });
    }

    #[test]
    fn test_mint_to_multiple_accounts() {
        let app = TestApp::default();
        app.register_handler::<Bank>().unwrap();

        // Initialize bank
        let mut root = app.client_context_for(ROOT_ACCOUNT);
        let bank_client = create_account::<Bank>(&mut root, BankCreate {}).unwrap();

        // Set up Alice as denom admin
        let mut alice = app.new_client_context().unwrap();
        let alice_id = alice.self_account_id();
        bank_client
            .create_denom(&mut root, "foo", alice_id)
            .unwrap();

        // Create multiple recipient accounts
        let bob = app.new_client_context().unwrap();
        let charlie = app.new_client_context().unwrap();
        let dave = app.new_client_context().unwrap();

        // Mint different amounts to different accounts
        bank_client
            .mint(&mut alice, bob.self_account_id(), "foo", 100)
            .unwrap();
        bank_client
            .mint(&mut alice, charlie.self_account_id(), "foo", 200)
            .unwrap();
        bank_client
            .mint(&mut alice, dave.self_account_id(), "foo", 300)
            .unwrap();

        // Verify individual balances
        assert_eq!(
            bank_client
                .get_balance(&bob, bob.self_account_id(), "foo")
                .unwrap(),
            100
        );
        assert_eq!(
            bank_client
                .get_balance(&charlie, charlie.self_account_id(), "foo")
                .unwrap(),
            200
        );
        assert_eq!(
            bank_client
                .get_balance(&dave, dave.self_account_id(), "foo")
                .unwrap(),
            300
        );

        // Verify total supply
        app.exec_in(&bank_client, |bank, ctx| {
            let foo_supply = bank.supply.get(ctx, "foo").unwrap();
            assert_eq!(foo_supply, 600);
        });
    }

    #[test]
    fn test_mint_events() {
        let app = TestApp::default();
        app.register_handler::<Bank>().unwrap();

        // Initialize bank
        let mut root = app.client_context_for(ROOT_ACCOUNT);
        let bank_client = create_account::<Bank>(&mut root, BankCreate {}).unwrap();

        // Set up Alice as denom admin
        let mut alice = app.new_client_context().unwrap();
        let alice_id = alice.self_account_id();
        bank_client
            .create_denom(&mut root, "foo", alice_id)
            .unwrap();

        // Create recipient account
        let bob = app.new_client_context().unwrap();
        let bob_id = bob.self_account_id();

        // Capture and verify mint event
        app.exec_in(&bank_client, |bank, mut ctx| {
            let mut events = EventBus::<EventMint>::default();
            bank.mint(&mut ctx, bob_id, "foo", 1000, events.clone())
                .unwrap();

            let emitted_events = events.get_events();
            assert_eq!(emitted_events.len(), 1);
            assert_eq!(emitted_events[0].to, bob_id);
            assert_eq!(emitted_events[0].coin.denom, "foo");
            assert_eq!(emitted_events[0].coin.amount, 1000);
        });
    }
}
