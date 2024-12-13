//! Basic functionality for creating and managing account lifecycle.

use crate::context::Context;
use crate::error::ClientError;
use crate::handler::{Handler, HandlerResources, InitMessage, Service};
use crate::result::ClientResult;
use core::str::from_utf8;
use ixc_core_macros::message_selector;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::code::SystemCode::EncodingError;
use ixc_message_api::message::{Message, Request};
use ixc_message_api::AccountID;
use ixc_schema::codec::Codec;

/// Creates a new account for the specified handler.
pub fn create_account<H: Handler>(
    ctx: &mut Context,
    init: H::Init<'_>,
) -> ClientResult<<H as Service>::Client> {
    let cdc = <<H as Handler>::Init<'_> as InitMessage<'_>>::Codec::default();
    let init_bz = cdc.encode_value(&init, ctx.memory_manager())?;

    let account_id = do_create_account(ctx, <H as HandlerResources>::NAME, init_bz)?;
    Ok(<H as Service>::new_client(account_id))
}

/// Creates a new account for the named handler with opaque initialization data.
pub fn create_account_raw(ctx: &mut Context, name: &str, init: &[u8]) -> ClientResult<AccountID> {
    do_create_account(ctx, name, init)
}

/// Creates a new account for the named handler with opaque initialization data.
fn do_create_account(ctx: &mut Context, name: &str, init: &[u8]) -> ClientResult<AccountID> {
    unsafe {
        let message = ixc_message_api::message::Message::new(
            ROOT_ACCOUNT,
            Request::new2(CREATE_SELECTOR, name.into(), init.into()),
        );
        let res = ctx.dynamic_invoke_msg(&message)?;
        let id = res.outputs[0].expect_account_id()?;
        Ok(id)
    }
}

/// Gets the handler ID of the account.
pub fn get_handler_id<'a>(ctx: &Context<'a>, account_id: AccountID) -> ClientResult<&'a str> {
    unsafe {
        let message = ixc_message_api::message::Message::new(
            ROOT_ACCOUNT,
            Request::new1(GET_HANDLER_ID_SELECTOR, account_id.into()),
        );
        let res = ctx.dynamic_invoke_query(&message)?;
        let handler_id = res.outputs[0].expect_string()?;
        Ok(handler_id)
    }
}

/// Migrates the account to the new handler with the specified ID.
pub fn migrate(ctx: &mut Context, new_handler_id: &str) -> ClientResult<()> {
    unsafe {
        let msg = Message::new(
            ROOT_ACCOUNT,
            Request::new1(MIGRATE_SELECTOR, new_handler_id.into()),
        );
        ctx.dynamic_invoke_msg(&msg)?;
        Ok(())
    }
}

/// Self-destructs the account.
///
/// # Safety
/// This function is unsafe because it can be used to destroy the account and all its state.
pub unsafe fn self_destruct(ctx: &mut Context) -> ClientResult<()> {
    let msg = Message::new(
        ROOT_ACCOUNT,
        Request::new(SELF_DESTRUCT_SELECTOR),
    );
    ctx.dynamic_invoke_msg(&msg)?;
    Ok(())
}

const CREATE_SELECTOR: u64 = message_selector!("ixc.account.v1.create");

const GET_HANDLER_ID_SELECTOR: u64 = message_selector!("ixc.account.v1.get_handler_id");

const MIGRATE_SELECTOR: u64 = message_selector!("ixc.account.v1.migrate");

const SELF_DESTRUCT_SELECTOR: u64 = message_selector!("ixc.account.v1.self_destruct");

/// The ID of the root account which creates and manages accounts.
pub const ROOT_ACCOUNT: AccountID = AccountID::new(1);

/// The message selector for the on_create message.
pub const ON_CREATE_SELECTOR: u64 = message_selector!("ixc.account.v1.on_create");

/// The message selector for the on_migrate message.
pub const ON_MIGRATE_SELECTOR: u64 = message_selector!("ixc.account.v1.on_migrate");

// TODO:
// // #[ixc_schema_macros::handler_api]
// /// The API for converting between native addresses and account IDs.
// /// Native addresses have both a byte representation and a string representation.
// /// The mapping between addresses and account IDs is assumed to be stateful.
// pub trait AddressAPI {
//     /// Convert an account ID to a byte representation.
//     fn to_bytes<'a>(&self, ctx: &'a Context, account_id: AccountID) -> crate::Result<&'a [u8]>;
//     /// Convert a byte representation to an account ID.
//     fn from_bytes<'a>(&self, ctx: &'a Context, address_bytes: &[u8]) -> crate::Result<AccountID>;
//     /// Convert an account ID to a string representation.
//     fn to_string<'a>(&self, ctx: &'a Context, account_id: AccountID) -> crate::Result<&'a str>;
//     /// Convert a string representation to an account ID.
//     fn from_string<'a>(&self, ctx: &'a Context, address_string: &str) -> crate::Result<AccountID>;
// }
