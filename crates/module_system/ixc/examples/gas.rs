#![allow(missing_docs)]
#[ixc::handler(Gas1)]
mod gas1 {
    use ixc::*;

    #[derive(Resources)]
    pub struct Gas1 {}

    impl Gas1 {
        #[on_create]
        fn create(&self, _ctx: &mut Context) -> Result<()> { Ok(()) }

        #[publish]
        fn consume_gas(&self, ctx: &mut Context) -> Result<()> {
            ctx.consume_gas(100)?;
            Ok(())
        }
    }
}

#[ixc::handler(Gas2)]
mod gas2 {
    use ixc::*;
    use ixc_core::low_level::dynamic_invoke_msg_with_gas;
    use crate::gas1::{Gas1, Gas1ConsumeGas};

    #[derive(Resources)]
    pub struct Gas2 {}

    impl Gas2 {
        #[on_create]
        fn create(&self, _ctx: &mut Context) -> Result<()> { Ok(()) }

        #[publish]
        fn call_consume_gas(&self, ctx: &mut Context, gas_eater: AccountID, limit: Option<u64>) -> Result<Option<u64>> {
            let res = dynamic_invoke_msg_with_gas(ctx, gas_eater, Gas1ConsumeGas {}, limit);
            // if res.is_err() {
            //     match ctx.gas_left() {
            //         Some(gas_left) => {
            //             if gas_left > 0 {
            //                 return Ok(Some(gas_left));
            //             }
            //         }
            //         None => Ok(None),
            //     }
            // }
            // Ok(ctx.gas_left())
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::gas1::*;
    use ixc_core::account_api::create_account;
    use ixc_core::handler::Client;
    use ixc_core::low_level::dynamic_invoke_msg_with_gas;
    use ixc_message_api::code::{ErrorCode, SystemCode};
    use ixc_testing::*;

    #[test]
    fn test_gas() {
        let app = TestApp::default();
        app.register_handler::<Gas1>().unwrap();
        let mut alice = app.new_client_context().unwrap();
        let gas1_client = create_account::<Gas1>(&mut alice, Gas1Create {}).unwrap();
        let res = dynamic_invoke_msg_with_gas(&mut alice, gas1_client.account_id(), Gas1ConsumeGas {}, None);
        assert!(res.is_ok());
        let res = dynamic_invoke_msg_with_gas(&mut alice, gas1_client.account_id(), Gas1ConsumeGas {}, Some(50));
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().code, ErrorCode::SystemCode(SystemCode::OutOfGas));
        // assert_eq!(gas.consumed(), 100);
        // let res = gas1_client.consume_gas(&mut alice);
        // assert!(res.is_err());
        // assert_eq!(res.unwrap_err().code, ErrorCode::SystemCode(SystemCode::OutOfGas));
    }
}

fn main() {}