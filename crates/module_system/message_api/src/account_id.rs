/// Account ID is a unique integer identifier for an account.
/// Every account has one and only one account identifier.
/// This is distinct from an account's "address".
/// An account may actually have multiple addresses in
/// different "address spaces" from the point of view of
/// an external user, but an account always has one unique account ID.
/// The account ID zero is reserved for the "null account" meaning
/// that the account is not valid or does not exist.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
}

impl Into<u128> for AccountID {
    fn into(self) -> u128 {
        self.0
    }
}

impl From<u128> for AccountID {
    fn from(value: u128) -> Self {
        AccountID(value)
    }
}
