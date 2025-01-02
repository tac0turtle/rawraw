extern crate alloc;

#[ixc::handler(Scheduler)]
pub mod scheduler {
    use ixc::*;
    #[derive(ixc::Resources)]
    pub struct Scheduler {
        #[state(prefix = 1)]
        /// Admin defines the admin scheduler.
        pub(crate) admin: Item<AccountID>,
        /// Maps schedule identifier to the accounts that need to be scheduled.
        /// Eg: begin_block => ("mint", "staking", ...)
        #[state(prefix = 2, key(denom), value(admin))]
        pub(crate) scheduling_map: Map<Str, Vec<AccountID>>,
    }

    #[handler_api]
    pub trait ScheduledAccount {
        fn schedule<'a>(
            &self,
            ctx: &mut Context<'a>,
            identifier: &str
        ) -> Result<()>;
    }

    impl Scheduler {
        /// lifetime failure on &str
        #[on_create]
        pub fn create(&self, ctx: &mut Context, scheduling_map: Vec<(&str, Vec<AccountID>)>) -> Result<()> {
            self.update_scheduling_map(ctx, scheduling_map)
        }

        /// lifetime failure on &str, cannot add lifetime tho for
        #[publish]
        pub fn update_scheduling_map(&self, ctx: &mut Context, scheduling_map: Vec<(&str, Vec<AccountID>)>) -> Result<()> {
            ensure!(ctx.caller() == self.admin.get(ctx)?, "unauthorized");
            for (k, v) in scheduling_map {
                self.scheduling_map.set(ctx, k, v)?;
            }
            Ok(())
        }
        #[publish]
        pub fn schedule(&self, ctx: &mut Context, schedule_identifier: &str) -> Result<()> {
            ensure!(ctx.caller() == self.admin.get(ctx)?, "unauthorized");
            let accounts = self.scheduling_map.get(ctx, schedule_identifier)?;
            match accounts {
                Some(accounts) => {
                    for account in accounts {
                        self.schedule_account(ctx, account, schedule_identifier);
                    }
                }
                None => (),
            };
            Ok(())
        }

        fn schedule_account(&self, ctx: &mut Context, account: AccountID, schedule_identifier: &str) {
            let scheduled = <dyn ScheduledAccount>::new_client(account);
            // ignore if the account fails
            // TODO: logging
            match scheduled.schedule(ctx, schedule_identifier) {
                Ok(_) => (),
                Err(_) => (),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_scheduler() {
    }
}