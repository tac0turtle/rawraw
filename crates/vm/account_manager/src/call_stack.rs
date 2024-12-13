use core::cell::RefCell;
use arrayvec::ArrayVec;
use ixc_message_api::AccountID;
use ixc_message_api::code::ErrorCode;
use crate::gas::GasMeter;

#[derive(Debug)]
pub(crate) struct CallStack<const CALL_STACK_LIMIT: usize> {
    call_stack: RefCell<ArrayVec<Frame, CALL_STACK_LIMIT>>,
    gas_meter: GasMeter,
}

#[derive(Debug)]
pub(crate) struct Frame {
    active_account: AccountID,
}

impl Frame {
    fn new(active_account: AccountID) -> Self {
        Self { active_account }
    }
}

impl<const CALL_STACK_LIMIT: usize> CallStack<CALL_STACK_LIMIT> {
    pub(crate) fn new(active_account: AccountID) -> Self {
        let mut res = Self {
            call_stack: RefCell::new(Default::default()),
            gas_meter: GasMeter::Unlimited, // TODO
        };
        res.push(active_account).unwrap();
        res
    }

    pub(crate) fn push(&self, account_id: AccountID) -> Result<(), ErrorCode> {
        self.call_stack.borrow_mut().try_push(Frame::new(account_id))
            .map_err(|_| ErrorCode::SystemCode(ixc_message_api::code::SystemCode::CallStackOverflow))
    }

    pub(crate) fn pop(&self) {
        self.call_stack.borrow_mut().pop();
    }

    pub(crate) fn active_account(&self) -> Result<AccountID, ErrorCode> {
        self.call_stack.borrow().last().map(|f| f.active_account)
            .ok_or(ErrorCode::SystemCode(ixc_message_api::code::SystemCode::FatalExecutionError))
    }

    pub(crate) fn gas_meter(&self) -> &GasMeter {
        &self.gas_meter
    }
}

