#![allow(missing_docs)]
#[ixc::handler(Delegator)]
mod delegator {
    use ixc::*;
    use ixc_core::handler::{Client, Service};
    use mockall::automock;
    use ixc_core::low_level::dynamic_invoke_msg_packet;
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
            let msg_packet = Message::new(msg.account, Request::new1(msg.selector, msg.bytes.as_slice().into()));
            dynamic_invoke_msg_packet(ctx, &msg_packet, None)?;
            Ok(())
        }
    }
}

fn main() {}
