use serde::{Deserialize, Serialize};

/// Account ID is a unique integer identifier for an account.
/// Every account has one and only one account identifier.
/// This is distinct from an account's "address".
/// An account may actually have multiple addresses in
/// different "address spaces" from the point of view of
/// an external user, but an account always has one unique account ID.
/// The account ID zero is reserved for the "null account" meaning
/// that the account is not valid or does not exist.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[repr(transparent)]
pub struct AccountID(u128);

impl AccountID {
    /// The empty account ID.
    pub const EMPTY: AccountID = AccountID(0);

    /// Creates a new account ID from the given integer.
    pub const fn new(id: u128) -> Self {
        AccountID(id)
    }

    /// Returns true if the account ID is zero.
    /// The account ID zero is reserved for the "null account" meaning
    /// that the account is not valid or does not exist.
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Bytes returns the bytes representation of the account ID.
    pub fn bytes(&self) -> [u8; 16] {
        self.0.to_be_bytes()
    }
}

impl From<AccountID> for u128 {
    fn from(val: AccountID) -> Self {
        val.0
    }
}

impl From<u128> for AccountID {
    fn from(value: u128) -> Self {
        AccountID(value)
    }
}
