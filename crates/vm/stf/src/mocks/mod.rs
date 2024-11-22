extern crate alloc;

use alloc::collections::BTreeMap;
use ixc_message_api::AccountID;
use ixc_message_api::header::MessageSelector;
use crate::stf::{Account, AccountCodes, State, Tx};

pub struct MockState {
    state: BTreeMap<Vec<u8>, Vec<u8>>
}

impl MockState {
    pub const fn new() -> Self {
        Self{
            state: BTreeMap::new()
        }
    }

    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.state.insert(key, value);
    }
}

impl State for MockState {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.state.get(key).cloned()
    }
}

#[derive(Debug)]
pub struct MockTx {
    sender: AccountID,
    recipient: AccountID,
    msg: Vec<u8>,
    selector: MessageSelector,
}

impl MockTx {
    pub fn new(sender: AccountID, recipient: AccountID, msg: Vec<u8>, selector: MessageSelector) -> Self {
        Self {
            sender,
            recipient,
            msg,
            selector: 0,
        }
    }
}

impl Tx for MockTx {
    fn get_sender(&self) -> &AccountID {
        &self.sender
    }

    fn get_recipient(&self) -> &AccountID {
        &self.recipient
    }

    fn get_msg(&self) -> Vec<u8> {
        self.msg.clone()
    }

    fn get_selector(&self) -> &MessageSelector {
        todo!()
    }
}

pub struct MockAccountCodes {
    accounts: BTreeMap<AccountID, Box<dyn Account>>,
}

impl MockAccountCodes {
    pub fn builder() -> MockAccountCodesBuilder {
        MockAccountCodesBuilder::new()
    }

    pub fn clear(&mut self) {
        self.accounts.clear();
    }
}

pub struct MockAccountCodesBuilder {
    accounts: BTreeMap<AccountID, Box<dyn Account>>,
}

impl MockAccountCodesBuilder {
    pub fn new() -> Self {
        Self {
            accounts: BTreeMap::new(),
        }
    }

    pub fn with_account(mut self, account_id: AccountID, code: impl Account) -> Self {
        self.accounts.insert(account_id, Box::new(code));
        self
    }

    pub fn build(self) -> MockAccountCodes {
        MockAccountCodes {
            accounts: self.accounts,
        }
    }
}

impl Default for MockAccountCodesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AccountCodes for MockAccountCodes {
    fn get_code_for_account(&self, account: &AccountID, _state: impl State) -> Result<Box<dyn Account>, String> {
        self.accounts
            .get(account)
            .cloned()
            .ok_or_else(|| alloc::format!("No code found for account: {:?}", account))
    }
}
