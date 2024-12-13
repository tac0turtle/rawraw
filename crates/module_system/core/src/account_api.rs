//! Basic functionality for creating and managing account lifecycle.

use crate::context::Context;
use crate::error::ClientError;
use crate::handler::{Handler, InitMessage, NamedResources, Service};
use crate::low_level::create_packet;
use crate::result::ClientResult;
use core::str::from_utf8;
use ixc_core_macros::message_selector;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::code::SystemCode::EncodingError;
use ixc_message_api::AccountID;
use ixc_schema::codec::Codec;

/// Creates a new account for the specified handler.
pub fn create_account<H: Handler>(
    ctx: &mut Context,
    init: H::Init<'_>,
) -> ClientResult<<H as Service>::Client> {
    let cdc = <<H as Handler>::Init<'_> as InitMessage<'_>>::Codec::default();
    let init_bz = cdc.encode_value(&init, ctx.memory_manager())?;

    let account_id = do_create_account(ctx, <H as NamedResources>::NAME, init_bz)?;
    Ok(<H as Service>::new_client(account_id))
}

/// Creates a new account for the named handler with opaque initialization data.
pub fn create_account_raw(ctx: &mut Context, name: &str, init: &[u8]) -> ClientResult<AccountID> {
    do_create_account(ctx, name, init)
}

/// Creates a new account for the named handler with opaque initialization data.
fn do_create_account(ctx: &mut Context, name: &str, init: &[u8]) -> ClientResult<AccountID> {
    let mut packet = create_packet(
        ctx.account,
        ctx.memory_manager(),
        ROOT_ACCOUNT,
        CREATE_SELECTOR,
    )?;

    unsafe {
        packet.header_mut().in_pointer1.set_slice(name.as_bytes());
        packet.header_mut().in_pointer2.set_slice(init);

        ctx.dynamic_invoke_msg(&mut packet)?;

        let res = packet.header().in_pointer1.get(&packet);
        if res.len() != size_of::<u128>() {
            return Err(ClientError::new(
                ErrorCode::SystemCode(EncodingError),
                "invalid account ID".into(),
            ));
        }

        Ok(AccountID::new(u128::from_le_bytes(res.try_into().unwrap())))
    }
}

/// Gets the handler ID of the account.
pub fn get_handler_id<'a>(ctx: &Context<'a>, account_id: AccountID) -> ClientResult<&'a str> {
    let mut packet = create_packet(
        ctx.self_account_id(),
        ctx.memory_manager(),
        ROOT_ACCOUNT,
        GET_HANDLER_ID_SELECTOR,
    )?;
    unsafe {
        let id: u128 = account_id.into();
        packet
            .header_mut()
            .in_pointer1
            .set_slice(&id.to_le_bytes());
        ctx.dynamic_invoke_query(&mut packet)?;
        let res = packet.header().out_pointer1.get(&packet);
        from_utf8(res).map_err(|_| {
            ClientError::new(
                ErrorCode::SystemCode(EncodingError),
                "invalid handler ID".into(),
            )
        })
    }
}

/// Migrates the account to the new handler with the specified ID.
pub fn migrate(ctx: &mut Context, new_handler_id: &str) -> ClientResult<()> {
    let mut packet = create_packet(
        ctx.self_account_id(),
        ctx.memory_manager(),
        ROOT_ACCOUNT,
        MIGRATE_SELECTOR,
    )?;
    unsafe {
        packet
            .header_mut()
            .in_pointer1
            .set_slice(new_handler_id.as_bytes());
        ctx.dynamic_invoke_msg(&mut packet)?;
    }
    Ok(())
}

/// Self-destructs the account.
///
/// # Safety
/// This function is unsafe because it can be used to destroy the account and all its state.
pub unsafe fn self_destruct(ctx: &mut Context) -> ClientResult<()> {
    let mut packet = create_packet(
        ctx.self_account_id(),
        ctx.memory_manager(),
        ROOT_ACCOUNT,
        SELF_DESTRUCT_SELECTOR,
    )?;
    ctx.dynamic_invoke_msg(&mut packet)?;
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
