#![allow(missing_docs)]
#[ixc::handler(Delegator)]
mod delegator {
    use ixc::*;
    use ixc_core::handler::{Client, Service};
    use mockall::automock;
    use ixc_core::low_level::{dynamic_invoke_msg_packet, invoke_any_message};
    use ixc_message_api::message::{Message, Request};
    use ixc_schema::any::AnyMessage;

    #[derive(Resources)]
    pub struct Delegator {
        #[state(prefix = 1)]
        pub delegatee: Item<AccountID>,
    }

    impl Delegator {
        #[on_create]
        pub fn create(&self, ctx: &mut Context) -> Result<()> {
            self.delegatee.set(ctx, &ctx.caller())?;
            Ok(())
        }
    }

    #[handler_api]
    trait ExecAuthorized {
        fn exec_authorized(&self, ctx: &mut Context, msg: &AnyMessage<'_>) -> Result<()>;
    }

    impl ExecAuthorized for Delegator {
        fn exec_authorized(&self, ctx: &mut Context, msg: &AnyMessage<'_>) -> Result<()> {
            let delegatee = self.delegatee.get(ctx)?;
            ensure!(ctx.caller() == delegatee, "unauthorized caller");
            invoke_any_message(ctx, msg)?;
            Ok(())
        }
    }
}

#[ixc::handler(CallCounter)]
mod call_counter {
    use ixc::*;

    #[derive(Resources)]
    pub struct CallCounter {
        #[state(prefix = 1, key(account), value(call_count))]
        pub calls: AccumulatorMap<AccountID>,
    }

    impl CallCounter {
        #[on_create]
        pub fn create(&self, ctx: &mut Context) -> Result<()> {
            Ok(())
        }

        #[publish]
        pub fn call(&self, ctx: &mut Context) -> Result<()> {
            let account = ctx.caller();
            self.calls.add(ctx, account, 1)?;
            Ok(())
        }

        #[publish]
        pub fn get_call_count(&self, ctx: &Context, account: AccountID) -> Result<u128> {
            Ok(self.calls.get(ctx, account)?)
        }
    }
}


#[cfg(test)]
mod tests {
    use ixc_core::handler::Client;
    use ixc_testing::*;
    use super::delegator::*;
    use super::call_counter::*;
    use ixc_schema::json::decode_value;

    #[test]
    fn test_any_message() {
        let app = TestApp::default();
        app.register_handler::<Delegator>().unwrap();
        app.register_handler::<CallCounter>().unwrap();
        let mut bob = app.new_client_context().unwrap();
        let bob_id = bob.self_account_id();
        let delegator_client = create_account::<Delegator>(&mut bob, DelegatorCreate {}).unwrap();
        let delegator_id = delegator_client.target_account();
        // let msg = format!(r#"{{"type":"create_account","value":{{"handler_id":"call_counter","init_data":"
        todo!()
    }
}

fn main() {}
