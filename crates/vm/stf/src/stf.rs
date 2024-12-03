use ixc_core_macros::message_selector;
use ixc_message_api::header::MessageSelector;
use ixc_message_api::AccountID;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

use crate::state::SnapshotState;

pub trait Tx {
    fn sender(&self) -> &AccountID;
    fn recipient(&self) -> &AccountID;
    fn msg(&self) -> &[u8];
    fn selector(&self) -> &MessageSelector;
}

pub trait Context {
    fn selector(&self) -> &MessageSelector;
    fn msg(&self) -> &[u8];
    fn whoami(&self) -> &AccountID;
    fn sender(&self) -> &AccountID;
    fn invoke(
        &mut self,
        recipient: &AccountID,
        selector: &MessageSelector,
        msg: &[u8],
    ) -> Result<Vec<u8>, String>;
    fn query(
        &self,
        recipient: &AccountID,
        selector: &MessageSelector,
        msg: &[u8],
    ) -> Result<Vec<u8>, String>;
}

pub trait Account {
    fn execute(&self, ctx: &mut dyn Context) -> Result<Vec<u8>, String>;
    fn query(&self, ctx: &dyn Context) -> Result<Vec<u8>, String>;
}

pub trait State {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
}

pub trait AccountCodes {
    fn get_code_for_account(
        &self,
        account: &AccountID,
        state: &dyn State,
    ) -> Result<&dyn Account, String>;
}

// New context implementation
struct ExecutionContext<'a> {
    whoami: &'a AccountID,
    sender: &'a AccountID,
    msg: &'a [u8],
    selector: &'a MessageSelector,
    account_codes: &'a dyn AccountCodes,
    state: Rc<RefCell<SnapshotState<DynStateWrapper>>>,
}

impl<'a> ExecutionContext<'a> {
    const SEND_SELECTOR: MessageSelector = message_selector!("send");
    const BALANCE_SELECTOR: MessageSelector = message_selector!("balance");
    fn new(
        whoami: &'a AccountID,
        sender: &'a AccountID,
        msg: &'a [u8],
        selector: &'a MessageSelector,
        account_codes: &'a dyn AccountCodes,
        state: Rc<RefCell<SnapshotState<DynStateWrapper>>>,
    ) -> Self {
        Self {
            whoami,
            sender,
            msg,
            selector,
            account_codes,
            state,
        }
    }

    fn handle_storage_mutable(
        &mut self,
        _sender: &AccountID,
        _msg: &[u8],
    ) -> Result<Vec<u8>, String> {
        todo!()
    }

    fn handle_storage_readonly(
        &self,
        sender: &AccountID,
        selector: &MessageSelector,
        msg: &[u8],
    ) -> Result<Vec<u8>, String> {
        match selector {
            &STORE_GET_SELECTOR => {
                let req = serde_json::from_slice::<StoreGetRequest>(self.msg).unwrap();
                let mut key = sender.bytes().to_vec();
                key.extend(req.key);
                let value = self.state.borrow().get(&key);
                Ok(serde_json::to_vec(&StoreGetResponse { value }).unwrap())
            }
            _ => Err(format!("invalid request: {:?}", selector)),
        }
    }
}

pub const STORE_SET_SELECTOR: MessageSelector = message_selector!("store.set");
pub const STORE_REMOVE_SELECTOR: MessageSelector = message_selector!("store.remove");
pub const STORE_GET_SELECTOR: MessageSelector = message_selector!("store.get");
pub const STORAGE_ACCOUNT_ID: AccountID = AccountID::new(u128::MAX);

impl<'a> Context for ExecutionContext<'a> {
    fn selector(&self) -> &MessageSelector {
        self.selector
    }

    fn msg(&self) -> &[u8] {
        self.msg
    }

    fn whoami(&self) -> &AccountID {
        self.whoami
    }

    fn sender(&self) -> &AccountID {
        self.sender
    }

    fn invoke(
        &mut self,
        recipient: &AccountID,
        selector: &MessageSelector,
        msg: &[u8],
    ) -> Result<Vec<u8>, String> {
        if recipient == &STORAGE_ACCOUNT_ID {
            return self.handle_storage_mutable(recipient, msg);
        }

        // Get the code for the recipient account
        let code = self
            .account_codes
            .get_code_for_account(recipient, &*self.state.borrow())?;

        // Create new context for the invocation
        let checkpoint = self.state.borrow_mut().checkpoint();
        let mut new_context = ExecutionContext::new(
            recipient,
            self.whoami,
            msg,
            selector,
            self.account_codes,
            Rc::clone(&self.state),
        );

        // Execute with new context
        let res = code.execute(&mut new_context);
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                self.state
                    .borrow_mut()
                    .restore_checkpoint(checkpoint)
                    .unwrap();
                Err(e)
            }
        }
    }

    fn query(
        &self,
        recipient: &AccountID,
        selector: &MessageSelector,
        msg: &[u8],
    ) -> Result<Vec<u8>, String> {
        if recipient == &STORAGE_ACCOUNT_ID {
            return self.handle_storage_readonly(self.whoami, selector, msg);
        }
        // Get the code for the recipient account
        let code = self
            .account_codes
            .get_code_for_account(recipient, &*self.state.borrow())?;

        // Create new context for the query
        let new_context = ExecutionContext::new(
            recipient,
            self.whoami,
            msg,
            selector,
            self.account_codes,
            Rc::clone(&self.state),
        );

        // Execute query with new context
        code.query(&new_context)
    }
}

pub struct Stf;

impl Stf {
    pub const fn new() -> Self {
        Self
    }

    pub fn apply_tx(
        &self,
        tx: impl Tx,
        account_codes: impl AccountCodes,
        state: impl State + 'static,
    ) -> Result<(), String> {
        let sender = tx.sender();
        let recipient = tx.recipient();
        let msg = tx.msg();

        // TODO: do authentication

        let state_rc = Rc::new(RefCell::new(SnapshotState::new(DynStateWrapper::new(
            Rc::new(RefCell::new(state)),
        ))));

        self.run_msg(
            sender,
            recipient,
            msg,
            tx.selector(),
            &account_codes,
            state_rc,
        )
    }

    fn run_msg(
        &self,
        sender: &AccountID,
        recipient: &AccountID,
        msg: &[u8],
        selector: &MessageSelector,
        account_codes: &dyn AccountCodes,
        state: Rc<RefCell<SnapshotState<DynStateWrapper>>>,
    ) -> Result<(), String> {
        // Get the code for the recipient account
        let code = account_codes.get_code_for_account(recipient, &*state.borrow())?;

        // Create initial execution context
        let mut context =
            ExecutionContext::new(recipient, sender, msg, selector, account_codes, state);

        // Execute the message with context
        code.execute(&mut context)?;

        Ok(())
    }
}

struct DynStateWrapper {
    state: Rc<RefCell<dyn State>>,
}

impl DynStateWrapper {
    fn new(state: Rc<RefCell<dyn State>>) -> Self {
        Self { state }
    }
}

impl State for DynStateWrapper {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.state.borrow().get(key)
    }
}

#[derive(Serialize, Deserialize)]
pub struct StoreSetRequest {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct StoreSetResponse {}

#[derive(Serialize, Deserialize)]
pub struct StoreGetRequest {
    pub key: Vec<u8>,
}
#[derive(Serialize, Deserialize)]
pub struct StoreGetResponse {
    pub value: Option<Vec<u8>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::{MockAccountCodes, MockState, MockTokenAccount, MockTx, MsgSend};

    #[test]
    fn test_stf() {
        let token = AccountID::new(1);
        let alice = AccountID::new(2);
        let bob = AccountID::new(3);

        let stf = Stf::new();
        let mut state = MockState::new();
        let mock_codes = MockAccountCodes::builder()
            .with_account(token, MockTokenAccount)
            .build();

        let msg = serde_json::to_vec(&MsgSend {
            to: bob,
            amount: 100,
        })
        .unwrap();

        let tx = MockTx::new(alice, token, msg, MsgSend::selector());

        stf.apply_tx(tx, mock_codes, state).unwrap();
    }
}
