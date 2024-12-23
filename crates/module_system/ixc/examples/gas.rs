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

#[cfg(test)]
mod tests {
    use super::gas1::*;
    use ixc_core::account_api::create_account;
    use ixc_core::handler::Client;
    use ixc_core::low_level::dynamic_invoke_msg_with_gas;
    use ixc_testing::*;

    #[test]
    fn test_gas() {
        let app = TestApp::default();
        app.register_handler::<Gas1>().unwrap();
        let mut alice = app.new_client_context().unwrap();
        let gas1_client = create_account::<Gas1>(&mut alice, Gas1Create {}).unwrap();
        // let gas = Gas::unlimited();
        // dynamic_invoke_msg_with_gas(&mut alice, gas1_client.account_id(), Gas1ConsumeGas {}, Some(&gas)).unwrap();
        // assert_eq!(gas.consumed(), 100);
        // let res = gas1_client.consume_gas(&mut alice);
        // assert!(res.is_err());
        // assert_eq!(res.unwrap_err().code, ErrorCode::SystemCode(SystemCode::OutOfGas));
    }
}

fn main() {}