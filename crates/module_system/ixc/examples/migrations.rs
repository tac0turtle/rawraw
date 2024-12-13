#![allow(missing_docs)]
#[ixc::handler(Handler1)]
mod handler1 {
    use ixc::*;
    use ixc_core::account_api;

    #[derive(Resources)]
    pub struct Handler1 {
        #[state(prefix = 0)]
        pub value: Item<u32>,
        #[state(prefix = 1)]
        owner: Item<AccountID>,
    }

    impl Handler1 {
        #[on_create]
        pub fn create(&self, ctx: &mut Context) -> Result<()> {
            self.value.set(ctx, 1)?;
            Ok(self.owner.set(ctx, &ctx.self_account_id())?)
        }

        #[publish]
        pub fn get(&self, ctx: &Context) -> Result<u32> {
            Ok(self.value.get(ctx)?)
        }

        #[publish]
        pub fn migrate(&self, ctx: &mut Context, new_handler_id: &str) -> Result<()> {
            ensure!(ctx.caller() == self.owner(ctx)?, "unauthorized caller");
            Ok(account_api::migrate(ctx, new_handler_id)?)
        }
    }
}

#[ixc::handler(Handler2)]
mod handler2 {
    use crate::handler1::Handler1;
    use ixc::*;
    use ixc_core::account_api;

    #[derive(Resources)]
    pub struct Handler2 {
        #[state(prefix = 0)]
        pub value: Item<u64>,
        #[state(prefix = 1)]
        owner: Item<AccountID>,
    }

    impl Handler2 {
        #[on_create]
        pub fn create(&self, ctx: &mut Context) -> Result<()> {
            Ok(self.owner.set(ctx, &ctx.self_account_id())?)
        }

        #[on_migrate]
        pub fn migrate1(&self, ctx: &mut Context, #[from] handler1: &Handler1) -> Result<()> {
            Ok(self.value.set(ctx, handler1.value.get(ctx)? as u64 * 2)?)
        }

        #[publish]
        pub fn get(&self, ctx: &Context) -> Result<u64> {
            Ok(self.value.get(ctx)?)
        }

        #[publish]
        pub fn migrate(&self, ctx: &mut Context, new_handler_id: &str) -> Result<()> {
            ensure!(ctx.caller() == self.owner(ctx)?, "unauthorized caller");
            Ok(account_api::migrate(ctx, new_handler_id)?)
        }
    }
}

#[ixc::handler(Handler3)]
mod handler3 {
    use crate::handler2::Handler2;
    use ixc::*;
    use ixc_core::handler::HandlerResources;

    #[derive(Resources)]
    pub struct Handler3 {
        #[state(prefix = 0)]
        value: Item<u128>,
        #[state(prefix = 1)]
        owner: Item<AccountID>,
    }

    impl Handler3 {
        #[on_create]
        pub fn create(&self, ctx: &mut Context) -> Result<()> {
            Ok(self.owner.set(ctx, &ctx.self_account_id())?)
        }

        #[on_migrate]
        pub fn migrate1(&self, ctx: &mut Context, #[from] handler1: &Handler1) -> Result<()> {
            Ok(self.value.set(ctx, handler1.value.get(ctx)? as u128 * 3)?)
        }

        #[on_migrate]
        pub fn migrate2(&self, ctx: &mut Context, #[from] handler2: &Handler2) -> Result<()> {
            Ok(self.value.set(ctx, handler2.value.get(ctx)? as u128 * 2)?)
        }

        #[publish]
        pub fn get(&self, ctx: &Context) -> Result<u128> {
            Ok(self.value.get(ctx)?)
        }
    }

    // here we show a simple way to implement the HandlerResources trait
    // so that we don't need all the old code from Handler1 to perform the migration
    // we just need this struct to read its state
    #[derive(Resources)]
    pub struct Handler1 {
        #[state(prefix = 0)]
        pub value: Item<u32>,
    }
    impl HandlerResources for Handler1 {
        const NAME: &'static str = "Handler1";
    }
}

#[cfg(test)]
mod tests {
    use crate::handler1::{Handler1, Handler1Create};
    use crate::handler2::Handler2;
    use crate::handler3::Handler3;
    use ixc::*;
    use ixc_core::account_api::get_handler_id;
    use ixc_core::handler::{Client, HandlerResources};
    use ixc_testing::*;

    #[test]
    fn test_migration() {
        let test_app = TestApp::default();
        test_app.register_handler::<Handler1>().unwrap();
        test_app.register_handler::<Handler2>().unwrap();
        test_app.register_handler::<Handler3>().unwrap();

        let mut bob = test_app.new_client_context().unwrap();
        let foo = create_account::<Handler1>(&mut bob, Handler1Create {}).unwrap();
        assert_eq!(
            get_handler_id(&bob, foo.account_id()).unwrap(),
            Handler1::NAME
        );
        let cur = foo.get(&bob).unwrap();
        assert_eq!(cur, 1);

        foo.migrate(&mut bob, Handler2::NAME).unwrap();
        assert_eq!(
            get_handler_id(&bob, foo.account_id()).unwrap(),
            Handler2::NAME
        );

        let foo = Handler2::new_client(foo.account_id());
        let cur = foo.get(&bob).unwrap();
        assert_eq!(cur, 2);

        foo.migrate(&mut bob, Handler3::NAME).unwrap();
        assert_eq!(
            get_handler_id(&bob, foo.account_id()).unwrap(),
            Handler3::NAME
        );

        let foo = Handler3::new_client(foo.account_id());
        let cur = foo.get(&bob).unwrap();
        assert_eq!(cur, 4);

        let bar = create_account::<Handler1>(&mut bob, Handler1Create {}).unwrap();
        assert_eq!(
            get_handler_id(&bob, bar.account_id()).unwrap(),
            Handler1::NAME
        );

        bar.migrate(&mut bob, Handler3::NAME).unwrap();
        assert_eq!(
            get_handler_id(&bob, bar.account_id()).unwrap(),
            Handler3::NAME
        );

        let bar = Handler3::new_client(bar.account_id());
        let cur = bar.get(&bob).unwrap();
        assert_eq!(cur, 3);
    }
}

fn main() {}
