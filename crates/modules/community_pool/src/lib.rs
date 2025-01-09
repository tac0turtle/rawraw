//! CommunityPool is a module that allows users to deposit and spend tokens.
#![allow(missing_docs)]

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
    use ixc_bank::bank::Coin;
    use ixc_core::handler::{Client, Service};
    use mockall::automock;
    use num_enum::{IntoPrimitive, TryFromPrimitive};
    use thiserror::Error;

    /// CommunityPool is a module that allows users to deposit and spend tokens.
    #[derive(Resources)]
    pub struct CommunityPool {
        #[state(prefix = 1)]
        pool_balance: AccumulatorMap<Str>, // Tracks balances per denom
        #[state(prefix = 2)]
        params: Item<PoolParams>,
        #[state(prefix = 3)]
        admin: Item<AccountID>,
        #[client(6656)]
        bank_client: <dyn BankAPI as Service>::Client,
    }

    /// PoolParams is a struct that represents the parameters of the community pool.
    #[derive(SchemaValue, Clone, Default)]
    #[sealed]
    pub struct PoolParams {
        /// SpendEnabled is a boolean that determines whether spending is enabled.
        pub spend_enabled: bool,
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

    #[handler_api]
    #[automock]
    pub trait BankAPI {
        fn send<'a>(
            &self,
            ctx: &mut Context<'a>,
            to: AccountID,
            amount: &[Coin<'a>],
        ) -> Result<(), SendError>;
    }

    #[derive(Clone, Debug, IntoPrimitive, TryFromPrimitive, Error, SchemaValue, Default, Copy)]
    #[repr(u8)]
    #[non_exhaustive]
    pub enum SendError {
        #[default]
        #[error("insufficient funds")]
        InsufficientFunds,

        #[error("send blocked")]
        SendBlocked,
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

        /// Deposit is used to add tokens to the community pool.
        #[publish]
        pub fn deposit<'a>(
            &self,
            ctx: &mut Context<'a>,
            denom: &'a str,
            amount: u128,
            mut evt: EventBus<EventDeposit<'a>>,
        ) -> Result<()> {
            let coins = [Coin { denom, amount }];
            // transfer funds from caller to community pool
            self.bank_client.send(ctx, ctx.self_account_id(), &coins)?;

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

            ensure!(self.admin.get(ctx)? == ctx.caller(), "not authorized");

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
    use ixc_bank::bank::Coin;
    use ixc_core::account_api::ROOT_ACCOUNT;
    use ixc_testing::*;

    #[test]
    fn test_community_pool_basic() {
        let app = TestApp::default();
        // initialize bank mock
        let mut bank_mock = MockBankAPI::new();
        // expect send to be called once with the correct coins
        let coins = vec![Coin {
            denom: "foo",
            amount: 1000,
        }];
        let expected_coins = coins.clone();
        bank_mock
            .expect_send()
            .times(1)
            .returning(move |_, _, coins| {
                assert_eq!(coins, expected_coins);
                Ok(())
            });
        let bank_id = app
            .add_mock(MockHandler::of::<dyn BankAPI>(Box::new(bank_mock)))
            .unwrap();
        let mut bank_ctx = app.client_context_for(bank_id);
        app.register_handler_with_bindings::<CommunityPool>(&[("bank", bank_id)])
            .unwrap();

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
}
