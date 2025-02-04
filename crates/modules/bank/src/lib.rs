//! Bank module that allows for the transfer of tokens between accounts.
#![allow(missing_docs)] //TODO remove when docs are added to macros
#![allow(clippy::needless_lifetimes)]
/// The bank module used for transfering tokens between accounts.
#[ixc::handler(Bank)]
pub mod bank {
    use ixc::*;
    use ixc_core::handler::Service;
    use mockall::automock;

    /// The bank module used for transfering tokens between accounts.
    #[derive(Resources)]
    pub struct Bank {
        /// The balances of accounts.
        #[state(prefix = 1, key(address, denom), value(amount))]
        pub(crate) balances: AccumulatorMap<(AccountID, Str)>,
        /// The total supply of tokens.
        #[state(prefix = 2, key(denom), value(total))]
        pub(crate) supply: AccumulatorMap<Str>,
        /// The super admin account.
        #[state(prefix = 3)]
        super_admin: Item<AccountID>,
        /// The global send hook account.
        #[state(prefix = 4)]
        global_send_hook: Item<AccountID>,
        /// The denom admins.
        #[state(prefix = 5)]
        denom_admins: Map<Str, AccountID>,
        /// The denom send hooks.
        #[state(prefix = 6)]
        denom_send_hooks: Map<Str, AccountID>,
        /// The denom burn hooks.
        #[state(prefix = 6)]
        denom_burn_hooks: Map<Str, AccountID>,
        /// The denom recieve hooks.
        #[state(prefix = 7)]
        denom_recieve_hooks: Map<AccountID, AccountID>,
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
            to: AccountID,
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

    /// The bank module used for transfering tokens between accounts.
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
            // Check if denom already exists
            let v = self.denom_admins.get(ctx, denom)?;
            if v.is_some() {
                return Err(error!("denom already exists"));
            }

            // Set the denom admin
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

        /// Set the denom send hook.
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

        /// Set the denom recieve hook.
        #[publish]
        pub fn set_denom_recieve_hook(&self, ctx: &mut Context, hook: AccountID) -> Result<()> {
            // Only denom admin can set recieve hooks
            let caller = ctx.caller();
            self.denom_recieve_hooks.set(ctx, caller, hook)?;
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

    /// The API of the bank module.
    #[publish]
    impl BankAPI for Bank {
        /// Get the balance of an account.
        fn get_balance(&self, ctx: &Context, account: AccountID, denom: &str) -> Result<u128> {
            let amount = self.balances.get(ctx, (account, denom))?;
            Ok(amount)
        }

        /// Send tokens from one account to another.
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

                if let Some(hook) = self.denom_recieve_hooks.get(ctx, to)? {
                    let hook_client = <dyn ReceiveHook>::new_client(hook);
                    hook_client.on_receive(ctx, from, to, coin.denom, coin.amount)?;
                }

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

        /// Mint tokens.
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

            if let Some(hook) = self.denom_recieve_hooks.get(ctx, to)? {
                let hook_client = <dyn ReceiveHook>::new_client(hook);
                hook_client.on_receive(ctx, ctx.caller(), to, denom, amount)?;
            }
            evt.emit(
                ctx,
                &EventMint {
                    to,
                    coin: Coin { denom, amount },
                },
            )?;
            Ok(())
        }

        /// Burn tokens.
        fn burn<'a>(
            &self,
            ctx: &mut Context,
            denom: &'a str,
            amount: u128,
            mut evt: EventBus<EventBurn<'a>>,
        ) -> Result<()> {
            // Check if the caller is authorized to burn

            // Check if there are any burn hooks and execute them
            if let Some(hook) = self.denom_burn_hooks.get(ctx, denom)? {
                let hook_client = <dyn BurnHook>::new_client(hook);
                hook_client.on_burn(ctx, ctx.caller(), denom, amount)?;
            }

            // Verify sufficient balance and reduce it
            self.balances.safe_sub(ctx, (ctx.caller(), denom), amount)?;

            // Reduce total supply
            self.supply.safe_sub(ctx, denom, amount)?;

            // Emit burn event
            evt.emit(
                ctx,
                &EventBurn {
                    from: ctx.caller(),
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

        // Test burn by token holder
        bank_client.burn(&mut bob, "foo", 300).unwrap();

        // Verify balance
        let bob_balance = bank_client.get_balance(&bob, bob_id, "foo").unwrap();
        assert_eq!(bob_balance, 700);

        // Verify total supply
        app.exec_in(&bank_client, |bank, ctx| {
            let foo_supply = bank.supply.get(ctx, "foo").unwrap();
            assert_eq!(foo_supply, 700);
        });

        // Test burn by admin
        let res = bank_client.burn(&mut alice, "foo", 200);
        assert!(res.is_err());

        // Verify final balance and supply
        let bob_balance = bank_client.get_balance(&bob, bob_id, "foo").unwrap();
        assert_eq!(bob_balance, 700);

        app.exec_in(&bank_client, |bank, ctx| {
            let foo_supply = bank.supply.get(ctx, "foo").unwrap();
            assert_eq!(foo_supply, 700);
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
}
