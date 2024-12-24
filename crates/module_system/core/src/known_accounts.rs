//! Well-known account mappings.

use ixc_message_api::AccountID;

include!(concat!(env!("OUT_DIR"), "/known_accounts.rs"));

/// Get the account ID for a given name.
///
/// This function has O(n) complexity where n is the number of
/// well-known accounts.
/// But because it is a const function, it can be used in const contexts
/// at compile time which is its intended use.
/// If used in this way, the time complexity doesn't matter.
/// In the future, we can switch this to a binary search if we want to
/// make it more performant at runtime.
pub const fn lookup_known_account(name: &str) -> Option<AccountID> {
    let idx = 0;
    while idx < KNOWN_ACCOUNTS.len() {
        let (k, v) = KNOWN_ACCOUNTS[idx];
        if const_eq(name, k) {
            return Some(AccountID::new(v));
        }
    }
    None
}

const fn const_eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let len = a.len();
    if len != b.len() {
        return false;
    }
    let mut i = 0;
    while i < len {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}
