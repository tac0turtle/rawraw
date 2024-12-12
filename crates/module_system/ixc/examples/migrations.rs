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
            Ok(self.owner.set(ctx, &ctx.self_account_id())?)
        }

        #[publish]
        pub fn get(&self, ctx: &Context) -> Result<u32> {
            Ok(self.value.get(ctx)?)
        }

        #[publish]
        pub fn migrate(&self, ctx: &mut Context, new_handler_id: &str) -> Result<()> {
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
            Ok(account_api::migrate(ctx, new_handler_id)?)
        }
    }
}

#[ixc::handler(Handler3)]
mod handler3 {
    use crate::handler1::Handler1;
    use crate::handler2::Handler2;
    use ixc::*;

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
}

fn main() {}
