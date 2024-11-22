extern crate alloc;

use alloc::collections::BTreeMap;
use ixc_core_macros::message_selector;
use ixc_message_api::AccountID;
use ixc_message_api::header::MessageSelector;
use serde::{Deserialize, Serialize};
use crate::stf::{Account, AccountCodes, Context, State, Tx};

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
    fn sender(&self) -> &AccountID {
        &self.sender
    }

    fn recipient(&self) -> &AccountID {
       &self.recipient
    }

    fn msg(&self) -> &[u8] {
       &self.msg
    }

    fn selector(&self) -> &MessageSelector {
        &self.selector
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
    fn get_code_for_account(&self, account: &AccountID, _state: &dyn State) -> Result<&dyn Account, String> {
        self.accounts
            .get(account)
            .ok_or_else(|| alloc::format!("No code found for account: {:?}", account))
            .map(|ac| ac as &dyn Account)
    }
}

pub struct MockTokenAccount;

impl MockTokenAccount {
    pub const SEND_SELECTOR: MessageSelector = message_selector!("send");
    pub const BALANCE_SELECTOR: MessageSelector = message_selector!("balance");
}

#[derive(Serialize, Deserialize)]
pub struct MsgSend {
    pub to: AccountID,
    pub amount: u128
}

#[derive(Serialize, Deserialize)]
pub struct MsgSendResponse {
}

#[derive(Serialize, Deserialize)]
pub struct QueryBalance {
    pub account: AccountID,
}

#[derive(Serialize, Deserialize)]
pub struct QueryBalanceResponse {
    pub amount: u128
}



impl Account for MockTokenAccount {
    fn execute(&self, ctx: &mut dyn Context) -> Result<Vec<u8>, String> {
        match ctx.selector() {
            &Self::SEND_SELECTOR => {
                let send = serde_json::from_slice::<MsgSend>(ctx.raw_request_msg()).unwrap();
                Ok(serde_json::to_vec(&MsgSendResponse{

                }).unwrap())
            }
            _ => Err("unknown exec request".to_string()),
        }
    }

    fn query(&self, ctx: &dyn Context) -> Result<Vec<u8>, String> {
       match ctx.selector() {
           &Self::BALANCE_SELECTOR => {
               let req  = serde_json::from_slice::<QueryBalance>(ctx.raw_request_msg()).unwrap();
               panic!("ok")
           }
           _ => Err("unknown query request".to_string()),
       }
    }
}

struct KVStore;

impl KVStore {
    pub fn get(ctx: &dyn Context) -> Result<Option<Vec<u8>>, String> {
        panic!("ok")
    }

    pub fn set(ctx: &mut dyn Context, key: Vec<u8>, value: Vec<u8>) -> Result<(), String> {
        todo!()
    }
}

struct MockEOAccount;

