#![allow(missing_docs)]
#[ixc::handler(GasEater1)]
mod gas1 {
    use ixc::*;

    #[derive(Resources)]
    pub struct GasEater1 {}

    impl GasEater1 {
        #[on_create]
        fn create(&self, _ctx: &mut Context) -> Result<()> {
            Ok(())
        }

        #[publish]
        fn eat_some_gas(&self, ctx: &mut Context) -> Result<()> {
            ctx.consume_gas(100)?;
            Ok(())
        }
    }
}

#[ixc::handler(GasEater2)]
mod gas2 {
    use ixc::*;
    use ixc_core::low_level::dynamic_invoke_msg_with_gas_tracker;
    use ixc_core::result::ClientResult;
    use ixc_message_api::code::{ErrorCode, SystemCode};
    use ixc_message_api::gas::GasTracker;

    #[derive(Resources)]
    pub struct GasEater2 {}

    impl GasEater2 {
        #[on_create]
        fn create(&self, _ctx: &mut Context) -> Result<()> {
            Ok(())
        }

        #[publish]
        fn call_eat_gas(
            &self,
            ctx: &mut Context,
            gas_eater: AccountID,
            limit: Option<u64>,
        ) -> Result<u64> {
            let tracker = GasTracker::new(limit);
            let res = dynamic_invoke_msg_with_gas_tracker(
                ctx,
                gas_eater,
                crate::gas1::GasEater1EatSomeGas {},
                Some(&tracker),
            );
            check_if_really_out_of_gas(ctx, res.clone())?;
            if res.is_err() {
                Ok(0)
            } else {
                Ok(tracker.consumed.get())
            }
        }
    }

    fn check_if_really_out_of_gas(ctx: &mut Context, res: ClientResult<()>) -> Result<()> {
        match res {
            Err(e) => {
                if e.code == ErrorCode::SystemCode(SystemCode::OutOfGas) {
                    if ctx.out_of_gas()? {
                        Err(e)
                    } else {
                        Ok(())
                    }
                } else {
                    Err(e)
                }
            }
            _ => res,
        }?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::gas1::*;
    use crate::gas2::*;
    use ixc_core::account_api::create_account;
    use ixc_core::handler::Client;
    use ixc_core::low_level::dynamic_invoke_msg_with_gas_tracker;
    use ixc_message_api::code::{ErrorCode, SystemCode};
    use ixc_message_api::gas::GasTracker;
    use ixc_testing::*;

    #[test]
    fn test_gas() {
        let app = TestApp::default();
        app.register_handler::<GasEater1>().unwrap();
        let mut alice = app.new_client_context().unwrap();
        let gas1_client = create_account::<GasEater1>(&mut alice, GasEater1Create {}).unwrap();
        let tracker = GasTracker::unlimited();
        let res = dynamic_invoke_msg_with_gas_tracker(
            &mut alice,
            gas1_client.target_account(),
            GasEater1EatSomeGas {},
            Some(&tracker),
        );
        assert!(res.is_ok());
        assert_eq!(tracker.consumed.get(), 100);
        let tracker = GasTracker::new(Some(50));
        let res = dynamic_invoke_msg_with_gas_tracker(
            &mut alice,
            gas1_client.target_account(),
            GasEater1EatSomeGas {},
            Some(&tracker),
        );
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().code,
            ErrorCode::SystemCode(SystemCode::OutOfGas)
        );
        assert_eq!(tracker.consumed.get(), 100);

        app.register_handler::<GasEater2>().unwrap();
        let gas2_client = create_account::<GasEater2>(&mut alice, GasEater2Create {}).unwrap();
        let res = gas2_client.call_eat_gas(&mut alice, gas1_client.target_account(), None);
        assert_eq!(res.unwrap(), 100);

        let tracker = GasTracker::unlimited();
        let res = dynamic_invoke_msg_with_gas_tracker(
            &mut alice,
            gas2_client.target_account(),
            GasEater2CallEatGas {
                gas_eater: gas1_client.target_account(),
                limit: None,
            },
            Some(&tracker),
        );
        assert_eq!(res.unwrap(), 100);
        assert_eq!(tracker.consumed.get(), 100);

        let tracker = GasTracker::limited(200);
        let res = dynamic_invoke_msg_with_gas_tracker(
            &mut alice,
            gas2_client.target_account(),
            GasEater2CallEatGas {
                gas_eater: gas1_client.target_account(),
                limit: Some(50),
            },
            Some(&tracker),
        );
        assert_eq!(res.unwrap(), 0);
        assert_eq!(tracker.consumed.get(), 100);
    }
}

fn main() {}
