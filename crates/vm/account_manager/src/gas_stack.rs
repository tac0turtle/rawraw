use core::cell::RefCell;
use arrayvec::ArrayVec;
use ixc_message_api::gas::Gas;

#[derive(Debug)]
pub(crate) struct GasStack<const CALL_STACK_LIMIT: usize> { {
    stack: RefCell<ArrayVec<Frame, CALL_STACK_LIMIT>>,
    pub(crate) gas: Gas,
}

pub(crate) struct GasScopeGuard<'a, const CALL_STACK_LIMIT: usize> {
    stack: &'a mut GasStack<CALL_STACK_LIMIT>,
    limit: Option<&'a Gas>,
}

impl <const CALL_STACK_LIMIT: usize> GasScopeGuard<CALL_STACK_LIMIT> {
    pub(crate) fn pop(self) {}

    fn do_pop(&mut self) {
        if let Some(gas) = self.limit {
            gas.consume().unwrap()
        }
    }
}

impl Drop for GasScopeGuard<'_> {
    fn drop(&mut self) {
    }
}

#[derive(Debug)]
struct Frame {
    gas_start: u64,
    gas_max: Option<u64>,
}