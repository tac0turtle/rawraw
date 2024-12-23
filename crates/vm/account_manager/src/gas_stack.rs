use arrayvec::ArrayVec;
use core::cell::RefCell;
use ixc_message_api::code::ErrorCode;
use crate::gas::GasMeter;

#[derive(Debug)]
pub(crate) struct GasStack<const CALL_STACK_LIMIT: usize> {
    stack: RefCell<ArrayVec<Frame, CALL_STACK_LIMIT>>,
    root_limit: Option<u64>,
    gas: GasMeter,
}

impl<const CALL_STACK_LIMIT: usize> GasStack<CALL_STACK_LIMIT> {
    pub(crate) fn new(gas_limit: Option<u64>) -> Self {
        Self {
            stack: RefCell::new(Default::default()),
            root_limit: gas_limit,
            gas: GasMeter::new(gas_limit),
        }
    }
}

#[derive(Debug)]
struct Frame {
    gas_start: u64,
    gas_max: Option<u64>,
}

impl<const CALL_STACK_LIMIT: usize> GasStack<CALL_STACK_LIMIT> {
    pub(crate) fn push(&self, gas_limit: Option<u64>) -> Result<(), ErrorCode> {
        if let Some(gas_limit) = gas_limit {
            // first get the amount of gas that has been consumed
            let gas_start = self.gas.consumed.get();
            // this is the new proposed gas limit, which we get by adding the gas start to the scoped gas limit
            let proposed_gas_max = gas_start + gas_limit;
            // this is the current gas limit based on the current scope or the root limit
            let cur_gas_max = if let Some(frame) = self.stack.borrow().last() {
                frame.gas_max
            } else {
                self.root_limit
            };
            // the new limit is the minimum of these
            let new_limit = if let Some(cur_gas_max) = cur_gas_max {
                proposed_gas_max.min(cur_gas_max)
            } else {
                proposed_gas_max
            };
            self.gas.limit.set(new_limit);
            //
            // let mut scope_gas_max = gas_limit + gas_start;
            // if let Some(cur_gas_max) = self.gas_max() {
            //     scope_gas_max = scope_gas_max.min(cur_gas_max);
            // }
            // gas_max = Some(scope_gas_max);
        };

        todo!();

        // let frame = Frame { gas_start, gas_max };
        // self.stack.borrow_mut().try_push(frame).map_err(|_| {
        //     ErrorCode::SystemCode(ixc_message_api::code::SystemCode::CallStackOverflow)
        // })
    }

    pub(crate) fn pop(&self) -> Result<(), ErrorCode> {
        // let frame = self.stack.borrow_mut().pop();
        // let gas_start = frame.map(|f| f.gas_start).unwrap_or(0);
        // self.gas.consumed() - gas_start
        todo!()
    }

    // fn cur_gas_max(&self) -> Option<u64> {
    //     if let Some(frame) = self.stack.borrow().last() {
    //         frame.gas_max
    //     } else {
    //         self.gas.limit()
    //     }
    // }

    // pub(crate) fn consume_gas(&self, gas: u64) -> Result<(), ErrorCode> {
    //     // self.gas.consume(gas)?;
    //     // let consumed = self.gas.consumed();
    //     // if let Some(Some(gas_max)) = self.stack.borrow().last().map(|f| f.gas_max) {
    //     //     if consumed > gas_max {
    //     //         return Err(ErrorCode::SystemCode(
    //     //             ixc_message_api::code::SystemCode::OutOfGas,
    //     //         ));
    //     //     }
    //     // }
    //     // Ok(())
    //     todo!()
    // }
    pub(crate) fn meter(&self) -> &GasMeter {
        &self.gas
    }
}
