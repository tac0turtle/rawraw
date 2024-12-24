//! Well-known account mappings.

use ixc_message_api::AccountID;

include!(concat!(env!("OUT_DIR"), "/known_accounts.rs"));

/// Get the account ID for a given name.
pub const fn lookup_known_account(name: &str) -> Option<AccountID> {
    let name_bytes = name.as_bytes();
    let idx = KNOWN_ACCOUNTS
        .binary_search_by_key(&name_bytes, |(k, _)| k.as_bytes())
        .ok()?;
    let (_, value) = KNOWN_ACCOUNTS[idx];
    Some(AccountID::new(value))
}
