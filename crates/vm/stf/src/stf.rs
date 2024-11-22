use ixc_core_macros::message_selector;
use ixc_message_api::header::MessageSelector;
use ixc_message_api::AccountID;
use crate::state::SnapshotState;

pub trait Tx {
    fn get_sender(&self) -> &AccountID;
    fn get_recipient(&self) -> &AccountID;
    fn get_msg(&self) -> Vec<u8>;
    fn get_selector(&self) -> &MessageSelector;
}

pub trait Context {
    fn selector(&self) -> &MessageSelector;
    fn raw_request_msg(&self) -> &[u8];
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
    ) -> Result<Box<dyn Account>, String>;
}

// New context implementation
struct ExecutionContext<'a> {
    whoami: &'a AccountID,
    sender: &'a AccountID,
    msg: &'a [u8],
    selector: &'a MessageSelector,
    account_codes: &'a dyn AccountCodes,
    state: SnapshotState<DynStateWrapper<'a>>,
}

impl<'a> ExecutionContext<'a> {
    fn new(
        whoami: &'a AccountID,
        sender: &'a AccountID,
        msg: &'a [u8],
        selector: &'a MessageSelector,
        account_codes: &'a dyn AccountCodes,
        state: &'a dyn State,
    ) -> Self {
        let state = SnapshotState::new(DynStateWrapper::new(state));
        Self {
            whoami,
            sender,
            msg,
            selector,
            account_codes,
            state,
        }
    }

    fn handle_storage_mutable(&mut self, sender: &AccountID, msg: &[u8]) -> Result<Vec<u8>, String> {
        todo!()
    }

    fn handle_storage_readonly(&self, sender: &AccountID, msg: &[u8]) -> Result<Vec<u8>, String> {
        todo!()
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

    fn raw_request_msg(&self) -> &[u8] {
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
            .get_code_for_account(recipient, &self.state)?;

        // we need to deconstruct and we will reconstruct later.
        // Create new context for the invocation where:
        // - current whoami becomes the sender
        // - recipient becomes the new whoami
        let mut new_context = ExecutionContext::new(
            recipient,
            self.whoami,
            msg,
            selector,
            self.account_codes,
            self.state,
        );

        // Execute with new context
        code.execute(&mut new_context)
    }

    fn query(
        &self,
        recipient: &AccountID,
        selector: &MessageSelector,
        msg: &[u8],
    ) -> Result<Vec<u8>, String> {
        if recipient == &STORAGE_ACCOUNT_ID {
            return self.handle_storage_readonly(self.whoami, msg)
        }
        // Get the code for the recipient account
        let code = self
            .account_codes
            .get_code_for_account(recipient, &self.state)?;

        // Create new context for the query
        let new_context = ExecutionContext::new(
            recipient,
            self.whoami,
            msg,
            selector,
            self.account_codes,
            self.state,
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
        state: impl State,
    ) -> Result<(), String> {
        let sender = tx.get_sender();
        let recipient = tx.get_recipient();
        let msg = tx.get_msg();

        // do authentication
        // TODO:

        self.run_msg(
            sender,
            recipient,
            &msg,
            tx.get_selector(),
            &account_codes,
            &state,
        )
    }

    fn run_msg(
        &self,
        sender: &AccountID,
        recipient: &AccountID,
        msg: &[u8],
        selector: &MessageSelector,
        account_codes: &dyn AccountCodes,
        state: &dyn State,
    ) -> Result<(), String> {
        // Get the code for the recipient account
        let code = account_codes.get_code_for_account(recipient, state)?;

        // Create initial execution context
        let mut context =
            ExecutionContext::new(recipient, sender, msg, selector, account_codes, state);

        // Execute the message with context
        code.execute(&mut context)?;

        Ok(())
    }
}

struct DynStateWrapper<'a> {
    state: &'a dyn State,
}

impl<'a> DynStateWrapper<'a> {
    fn new(state: &'a dyn State) -> Self {
        Self { state }
    }
}

impl<'a> State for DynStateWrapper<'a> {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.state.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::{MockAccountCodes, MockState, MockTokenAccount, MockTx};
    #[test]
    fn test_stf() {
        let token = AccountID::new(1);
        let alice = AccountID::new(2);

        let stf = Stf::new();
        let state = MockState::new();
        let mock_codes = MockAccountCodes::builder()
            .with_account(token, MockTokenAccount)
            .build();

        let tx = MockTx::new(alice, token);

        stf.apply_tx(

        ).unwrap();
    }
}
