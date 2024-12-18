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
    pub const EMPTY: AccountID = AccountID::new(0);

    /// Creates a new account ID from the given integer.
    pub const fn new(id: u128) -> Self {
        AccountID(id)
    }

    /// Returns true if the account ID is zero.
    /// The account ID zero is reserved for the "null account" meaning
    /// that the account is not valid or does not exist.
    pub fn is_empty(&self) -> bool {
        self == &Self::EMPTY
    }

    /// Returns the account ID as a big-endian byte array.
    pub fn to_bytes(&self) -> [u8; 16] {
        self.0.to_le_bytes()
    }
}

impl From<AccountID> for u128 {
    fn from(val: AccountID) -> Self {
        val.0
    }
}

impl From<u128> for AccountID {
    fn from(value: u128) -> Self {
        AccountID::new(value)
    }
}

impl From<[u8; 16]> for AccountID {
    fn from(value: [u8; 16]) -> Self {
        AccountID::new(u128::from_le_bytes(value))
    }
}

impl From<AccountID> for [u8; 16] {
    fn from(value: AccountID) -> Self {
        value.to_bytes()
    }
}