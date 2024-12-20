use arrayvec::ArrayVec;
use core::cell::RefCell;
use ixc_message_api::code::ErrorCode;
use ixc_message_api::gas::Gas;
use ixc_message_api::AccountID;

#[derive(Debug)]
pub(crate) struct CallStack<const CALL_STACK_LIMIT: usize> {
    call_stack: RefCell<ArrayVec<Frame, CALL_STACK_LIMIT>>,
    pub(crate) gas: Gas,
}

#[derive(Debug)]
pub(crate) struct Frame {
    active_account: AccountID,
    gas_start: u64,
    gas_max: Option<u64>,
}

impl<const CALL_STACK_LIMIT: usize> CallStack<CALL_STACK_LIMIT> {
    pub(crate) fn new(active_account: AccountID, gas_limit: Option<u64>) -> Self {
        let gas_meter = Gas::limited(gas_limit.unwrap_or(0));
        let res = Self {
            call_stack: RefCell::new(Default::default()),
            gas: gas_meter,
        };
        res.push(active_account, gas_limit).unwrap();
        res
    }

    pub(crate) fn push(
        &self,
        account_id: AccountID,
        gas_limit: Option<u64>,
    ) -> Result<(), ErrorCode> {
        let gas_start = self.gas.consumed();
        let mut gas_max = None;
        if let Some(gas_limit) = gas_limit {
            let mut scope_gas_max = gas_limit + gas_start;
            if let Some(cur_gas_max) = self.gas_max() {
                scope_gas_max = scope_gas_max.min(cur_gas_max);
            }
            gas_max = Some(scope_gas_max);
        };
        let frame = Frame {
            active_account: account_id,
            gas_start,
            gas_max,
        };
        self.call_stack.borrow_mut().try_push(frame).map_err(|_| {
            ErrorCode::System(ixc_message_api::code::SystemCode::CallStackOverflow)
        })
    }

    /// Pops the top frame from the call stack and returns the gas consumed
    /// since the frame was pushed.
    pub(crate) fn pop(&self) -> u64 {
        let frame = self.call_stack.borrow_mut().pop();
        let gas_start = frame.map(|f| f.gas_start).unwrap_or(0);
        self.gas.consumed() - gas_start
    }

    fn gas_max(&self) -> Option<u64> {
        if let Some(frame) = self.call_stack.borrow().last() {
            frame.gas_max
        } else {
            self.gas.limit()
        }
    }

    pub(crate) fn consume_gas(&self, gas: u64) -> Result<(), ErrorCode> {
        self.gas.consume(gas)?;
        let consumed = self.gas.consumed();
        if let Some(Some(gas_max)) = self.call_stack.borrow().last().map(|f| f.gas_max) {
            if consumed > gas_max {
                return Err(ErrorCode::System(
                    ixc_message_api::code::SystemCode::OutOfGas,
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn caller(&self) -> Result<AccountID, ErrorCode> {
        let call_stack = self.call_stack.borrow();
        let len = call_stack.len();
        call_stack
            .get(len - 2)
            .map(|f| f.active_account)
            .ok_or(ErrorCode::System(
                ixc_message_api::code::SystemCode::FatalExecutionError,
            ))
    }

    pub(crate) fn active_account(&self) -> Result<AccountID, ErrorCode> {
        self.call_stack
            .borrow()
            .last()
            .map(|f| f.active_account)
            .ok_or(ErrorCode::System(
                ixc_message_api::code::SystemCode::FatalExecutionError,
            ))
    }
}
