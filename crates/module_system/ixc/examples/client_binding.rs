#![allow(missing_docs)]

use ixc_core::create_account;
use ixc_testing::TestApp;
use crate::client_binding::{ClientBindingTest, ClientBindingTestCreate};

#[ixc::handler(ClientBindingTest)]
mod client_binding {
    use ixc::*;
    use ixc_core::handler::Client;

    // just a dummy marker API
    #[handler_api]
    pub trait FooAPI {}

    #[derive(Resources)]
    pub struct ClientBindingTest {
        #[client(foo)]
        foo_client: <dyn FooAPI as Service>::Client,
    }

    impl ClientBindingTest {
        #[on_create]
        pub fn create(&self, ctx: &mut Context) -> Result<()> { Ok(()) }

        #[publish]
        pub fn who_is_foo(&self, ctx: &mut Context) -> Result<AccountID> {
            Ok(self.foo_client.account_id())
        }
    }
}

fn main() {
    let test_app = TestApp::default();
    test_app.register_handler_with_bindings::<ClientBindingTest>(&[
        ("foo", 2.into()),
    ]).unwrap();
    let mut alice = test_app.new_client_context().unwrap();
    let foo_client = create_account::<ClientBindingTest>(&mut alice, ClientBindingTestCreate {}).unwrap();
    println!("foo is {:?}", foo_client.who_is_foo(&mut alice).unwrap());
}