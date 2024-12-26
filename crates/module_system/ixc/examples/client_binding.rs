#![allow(missing_docs)]

use crate::client_binding::{ClientBindingTest, ClientBindingTestCreate};
use ixc_core::create_account;
use ixc_core::known_accounts::ROOT_ACCOUNT;
use ixc_message_api::AccountID;
use ixc_testing::TestApp;
use std::str::FromStr;

#[ixc::handler(ClientBindingTest)]
mod client_binding {
    use ixc::*;
    use ixc_core::handler::Client;

    // just a dummy marker API
    #[handler_api]
    pub trait AnyAPI {}

    #[derive(Resources)]
    pub struct ClientBindingTest {
        #[client(foo)]
        foo_client: <dyn AnyAPI as Service>::Client,
        #[client(bar)]
        bar_client: <dyn AnyAPI as Service>::Client,
    }

    impl ClientBindingTest {
        #[on_create]
        pub fn create(&self, ctx: &mut Context) -> Result<()> {
            Ok(())
        }

        #[publish]
        pub fn who_is_foo(&self, ctx: &Context) -> Result<AccountID> {
            Ok(self.foo_client.account_id())
        }

        #[publish]
        pub fn who_is_bar(&self, ctx: &Context) -> Result<AccountID> {
            Ok(self.bar_client.account_id())
        }
    }
}

fn main() {
    let test_app = TestApp::default();
    test_app.register_handler::<ClientBindingTest>().unwrap();
    let mut alice = test_app.new_client_context().unwrap();
    let binding_test_client =
        create_account::<ClientBindingTest>(&mut alice, ClientBindingTestCreate {}).unwrap();
    let foo = binding_test_client.who_is_foo(&alice).unwrap();
    let bar = binding_test_client.who_is_bar(&alice).unwrap();
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        panic!("expected at least two arguments");
    }
    let expected_root = u128::from_str_radix(&args[1], 16).unwrap();
    assert_eq!(ROOT_ACCOUNT, AccountID::new(expected_root));
    let expected_foo = u128::from_str_radix(&args[2], 16).unwrap();
    let expected_bar = u128::from_str_radix(&args[3], 16).unwrap();
    assert_eq!(foo, AccountID::new(expected_foo));
    assert_eq!(bar, AccountID::new(expected_bar));
    println!("Successfully matched expected account IDs");
}
