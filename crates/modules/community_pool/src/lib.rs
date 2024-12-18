//! CommunityPool is a module that allows users to deposit and spend tokens.

/// # Resources
///
/// ## PoolBalance
///
/// Tracks balances per denom.
///
/// ## PoolParams
///
/// Stores the parameters of the community pool.
///
/// ## Admin
///
/// Stores the account ID of the admin.
///
/// ## SpendHooks
///
/// Stores the spend hooks per denom.
#[ixc::handler(CommunityPool)]
pub mod community_pool {
    use ixc::*;

    /// CommunityPool is a module that allows users to deposit and spend tokens.
    #[derive(Resources)]
    pub struct CommunityPool {
        #[state(prefix = 1)]
        pool_balance: AccumulatorMap<Str>, // Tracks balances per denom
        #[state(prefix = 2)]
        params: Item<PoolParams>,
        #[state(prefix = 3)]
        admin: Item<AccountID>,
        #[state(prefix = 4)]
        spend_hooks: Map<Str, AccountID>, // Optional hooks per denom
    }

    /// PoolParams is a struct that represents the parameters of the community pool.
    #[derive(SchemaValue, Clone, Default)]
    #[sealed]
    pub struct PoolParams {
        /// SpendEnabled is a boolean that determines whether spending is enabled.
        pub spend_enabled: bool,
        /// MinProposalAmount is the minimum amount of tokens that can be spent.
        pub min_proposal_amount: u128,
    }

    /// Coin is a struct that represents a coin.
    #[derive(SchemaValue, Clone, Default)]
    #[sealed]
    pub struct Coin<'a> {
        /// The denom of the coin.
        pub denom: &'a str,
        /// The amount of the coin.
        pub amount: u128,
    }

    /// EventDeposit is emitted when a deposit is executed.
    #[derive(SchemaValue, Clone, Default)]
    #[sealed]
    pub struct EventDeposit<'a> {
        /// EventSpend is emitted when a spend is executed.
        pub from: AccountID,
        /// The coin that was deposited.
        pub coin: Coin<'a>,
    }

    /// EventSpend is emitted when a spend is executed.
    #[derive(SchemaValue, Clone, Default)]
    #[sealed]
    pub struct EventSpend<'a> {
        /// The account that was spent to.
        pub to: AccountID,
        /// The coin that was spent.
        pub coin: Coin<'a>,
        /// The proposal ID that was spent.
        pub proposal_id: u64,
    }

    /// SpendHook is a trait that allows modules to hook into the spending process.
    #[handler_api]
    pub trait SpendHook {
        /// BeforeSpend is called before a spend is executed.
        fn before_spend<'a>(
            &self,
            ctx: &mut Context<'a>,
            to: AccountID,
            denom: &str,
            amount: u128,
            proposal_id: u64,
        ) -> Result<()>;
    }

    /// CommunityPool is a module that allows users to deposit and spend tokens.
    impl CommunityPool {
        /// Create is called when the community pool is created.
        #[on_create]
        pub fn create(&self, ctx: &mut Context) -> Result<()> {
            // Initialize with default params
            self.params.set(
                ctx,
                PoolParams {
                    spend_enabled: true,
                    min_proposal_amount: 1000,
                },
            )?;
            // Set creator as admin
            self.admin.set(ctx, ctx.caller())?;
            Ok(())
        }

        /// SetParams is used to set the community pool parameters.
        #[publish]
        pub fn set_params(&self, ctx: &mut Context, new_params: PoolParams) -> Result<()> {
            ensure!(self.admin.get(ctx)? == ctx.caller(), "not authorized");
            self.params.set(ctx, new_params)?;
            Ok(())
        }

        /// SetSpendHook is used to set a spend hook for a specific denom.
        #[publish]
        pub fn set_spend_hook(
            &self,
            ctx: &mut Context,
            denom: &str,
            hook: AccountID,
        ) -> Result<()> {
            ensure!(self.admin.get(ctx)? == ctx.caller(), "not authorized");
            self.spend_hooks.set(ctx, denom, hook)?;
            Ok(())
        }

        /// Deposit is used to add tokens to the community pool.
        #[publish]
        pub fn deposit<'a>(
            &self,
            ctx: &mut Context,
            denom: &'a str,
            amount: u128,
            mut evt: EventBus<EventDeposit<'a>>,
        ) -> Result<()> {
            // Add to pool balance
            self.pool_balance.add(ctx, denom, amount)?;

            // Emit deposit event
            evt.emit(
                ctx,
                &EventDeposit {
                    from: ctx.caller(),
                    coin: Coin { denom, amount },
                },
            )?;

            Ok(())
        }

        /// Spend is used to send tokens from the community pool to another account.
        #[publish]
        pub fn spend<'a>(
            &self,
            ctx: &mut Context,
            to: AccountID,
            denom: &'a str,
            amount: u128,
            proposal_id: u64,
            mut evt: EventBus<EventSpend<'a>>,
        ) -> Result<()> {
            // Verify spend is enabled
            let params = self.params.get(ctx)?;
            ensure!(params.spend_enabled, "spending is disabled");
            ensure!(amount >= params.min_proposal_amount, "amount below minimum");

            // Check if there's a spend hook and execute it
            if let Some(hook) = self.spend_hooks.get(ctx, denom)? {
                let hook_client = <dyn SpendHook>::new_client(hook);
                hook_client.before_spend(ctx, to, denom, amount, proposal_id)?;
            }

            // Verify sufficient balance and subtract
            self.pool_balance.safe_sub(ctx, denom, amount)?;

            // Emit spend event
            evt.emit(
                ctx,
                &EventSpend {
                    to,
                    coin: Coin { denom, amount },
                    proposal_id,
                },
            )?;

            Ok(())
        }

        /// GetBalance is used to get the balance of a specific denom.
        #[publish]
        pub fn get_balance(&self, ctx: &Context, denom: &str) -> Result<u128> {
            Ok(self.pool_balance.get(ctx, denom)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::community_pool::*;
    use ixc_core::account_api::ROOT_ACCOUNT;
    use ixc_testing::*;

    #[test]
    fn test_community_pool_basic() {
        let app = TestApp::default();
        app.register_handler::<CommunityPool>().unwrap();

        // Initialize with root account
        let mut root = app.client_context_for(ROOT_ACCOUNT);
        let pool = create_account::<CommunityPool>(&mut root, CommunityPoolCreate {}).unwrap();

        // Test deposit
        pool.deposit(&mut root, "atom", 1000).unwrap();
        assert_eq!(pool.get_balance(&root, "atom").unwrap(), 1000);

        // Test spend
        let mut alice = app.new_client_context().unwrap();
        let alice_id = alice.self_account_id();

        // Only admin can spend
        let result = pool.spend(&mut alice, alice_id, "atom", 500, 1);
        assert!(result.is_err());

        // Admin can spend
        pool.spend(&mut root, alice_id, "atom", 500, 1).unwrap();
        assert_eq!(pool.get_balance(&root, "atom").unwrap(), 500);
    }

    #[test]
    fn test_spend_hooks() {
        let app = TestApp::default();
        app.register_handler::<CommunityPool>().unwrap();

        let mut root = app.client_context_for(ROOT_ACCOUNT);
        let pool = create_account::<CommunityPool>(&mut root, CommunityPoolCreate {}).unwrap();

        // Set up mock spend hook
        let mut mock_spend_hook = MockSpendHook::new();
        mock_spend_hook
            .expect_before_spend()
            .times(1)
            .returning(|_, _, _, _, _| Ok(()));

        let mut mock = MockHandler::new();
        mock.add_handler::<dyn SpendHook>(Box::new(mock_spend_hook));
        let mock_id = app.add_mock(mock).unwrap();

        // Set spend hook
        pool.set_spend_hook(&mut root, "atom", mock_id).unwrap();

        // Deposit and spend to trigger hook
        pool.deposit(&mut root, "atom", 1000).unwrap();
        let alice = app.new_client_context().unwrap();
        pool.spend(&mut root, alice.self_account_id(), "atom", 500, 1)
            .unwrap();
    }
}
