use crate::gas::GasMeter;
use crate::scope_guard::ScopeGuardStack;
use arrayvec::ArrayVec;
use core::cell::RefCell;
use ixc_message_api::code::{ErrorCode, SystemCode};
use ixc_message_api::gas::GasTracker;

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
    scoped_gas_limit: u64,
}

pub(crate) struct GasScopeGuard<'a, const CALL_STACK_LIMIT: usize> {
    stack: &'a GasStack<CALL_STACK_LIMIT>,
    tracker: Option<&'a GasTracker>,
    popped: bool,
}

impl<const CALL_STACK_LIMIT: usize> GasStack<CALL_STACK_LIMIT> {
    pub(crate) fn push<'a>(
        &'a self,
        scoped_gas_tracker: Option<&'a GasTracker>,
    ) -> Result<GasScopeGuard<'a, CALL_STACK_LIMIT>, ErrorCode> {
        // if we're already out of gas then just error out
        if self.meter().out_of_gas() {
            return Err(ErrorCode::SystemCode(SystemCode::OutOfGas));
        }

        let gas_start = self.gas.consumed.get();
        let frame = if let Some(scoped_gas_limit) = scoped_gas_tracker.map(|g| g.limit).flatten() {
            // first get the amount of gas that has been consumed
            // this is the new proposed gas limit, which we get by adding the gas start to the scoped gas limit
            let proposed_limit = gas_start + scoped_gas_limit;
            // this is the current gas limit based on the current scope or the root limit
            let cur_limit = self.cur_gas_limit();
            // the new limit is the minimum of these
            let new_limit = if cur_limit > 0 {
                // if the cur gas max is 0, then we have no gas limit
                proposed_limit.min(cur_limit)
            } else {
                proposed_limit
            };
            self.gas.limit.set(new_limit);
            Frame {
                gas_start,
                scoped_gas_limit: new_limit,
            }
        } else {
            Frame {
                gas_start,
                scoped_gas_limit: self.cur_gas_limit(),
            }
        };
        self.stack.borrow_mut().try_push(frame).map_err(|_| {
            ErrorCode::SystemCode(ixc_message_api::code::SystemCode::CallStackOverflow)
        })?;
        Ok(GasScopeGuard {
            stack: self,
            tracker: scoped_gas_tracker,
            popped: false,
        })
    }

    // returns 0 if there is no gas limit
    fn cur_gas_limit(&self) -> u64 {
        if let Some(frame) = self.stack.borrow().last() {
            frame.scoped_gas_limit
        } else {
            self.root_limit.unwrap_or(0)
        }
    }

    pub(crate) fn meter(&self) -> &GasMeter {
        &self.gas
    }
}

impl<'a, const CALL_STACK_LIMIT: usize> GasScopeGuard<'a, CALL_STACK_LIMIT> {
    pub(crate) fn pop(mut self) {
        self.do_pop();
    }

    pub(crate) fn do_pop(&mut self) {
        if !self.popped {
            let frame = self.stack.stack.borrow_mut().pop();
            if let Some(frame) = frame {
                if let Some(tracker) = self.tracker {
                    tracker
                        .consumed
                        .set(self.stack.gas.consumed.get() - frame.gas_start);
                }
            }
            self.stack.gas.limit.set(self.stack.cur_gas_limit());
            self.popped = true;
        }
    }
}

impl<'a, const CALL_STACK_LIMIT: usize> Drop for GasScopeGuard<'a, CALL_STACK_LIMIT> {
    fn drop(&mut self) {
        self.do_pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ixc_message_api::code::SystemCode;

    #[test]
    fn test_gas_limit_stacking() {
        let gas_stack: GasStack<256> = GasStack::new(Some(100));
        assert_eq!(gas_stack.meter().left(), Some(100));
        {
            let tracker = GasTracker::limited(20);
            let scope = gas_stack.push(Some(&tracker)).unwrap();
            assert_eq!(gas_stack.meter().left(), Some(20));
            gas_stack.meter().consume(1).unwrap();
            assert_eq!(gas_stack.meter().left(), Some(19));
            {
                let tracker = GasTracker::limited(10);
                let scope = gas_stack.push(Some(&tracker)).unwrap();
                assert_eq!(gas_stack.meter().left(), Some(10));
                {
                    let tracker = GasTracker::unlimited();
                    let scope = gas_stack.push(Some(&tracker)).unwrap();
                    // inherit child limit
                    assert_eq!(gas_stack.meter().left(), Some(10));
                    {
                        let tracker = GasTracker::limited(100);
                        let scope = gas_stack.push(Some(&tracker)).unwrap();
                        // cannot exceed parent limit
                        assert_eq!(gas_stack.meter().left(), Some(10));
                        // consume all the gas
                        assert_eq!(gas_stack.meter().consume(10), Ok(()));
                        scope.pop();
                        assert_eq!(tracker.consumed.get(), 10);
                    }
                    assert_eq!(gas_stack.meter().left(), Some(0));
                    assert_eq!(gas_stack.meter().consumed(), 11);
                    scope.pop();
                    assert_eq!(tracker.consumed.get(), 10);
                }

                assert_eq!(gas_stack.meter().left(), Some(0));
                // consuming any more causes an out of gas error
                assert_eq!(
                    gas_stack.meter().consume(1),
                    Err(ErrorCode::SystemCode(SystemCode::OutOfGas))
                );
                assert_eq!(gas_stack.meter().consumed(), 12);
                assert_eq!(gas_stack.meter().left(), Some(0));
                scope.pop();
                assert_eq!(tracker.consumed.get(), 11);
            }
            assert_eq!(gas_stack.meter().left(), Some(8));
            assert_eq!(gas_stack.meter().consume(5), Ok(()));
            assert_eq!(gas_stack.meter().left(), Some(3));
            assert_eq!(gas_stack.meter().consumed(), 17);
            scope.pop();
            assert_eq!(tracker.consumed.get(), 17);
        }
        assert_eq!(gas_stack.meter().left(), Some(83));
        assert_eq!(gas_stack.meter().consumed(), 17);
        {
            // push a frame with no gas limit
            let scope = gas_stack.push(None).unwrap();
            gas_stack.meter().consume(10).unwrap();
            assert_eq!(gas_stack.meter().left(), Some(73));
            assert_eq!(gas_stack.meter().consumed(), 27);
            let tracker = GasTracker::limited(10);
            {
                let scope = gas_stack.push(Some(&tracker)).unwrap();
                gas_stack.meter().consume(10).unwrap();
                assert_eq!(gas_stack.meter().left(), Some(0));
                assert_eq!(gas_stack.meter().consumed(), 37);
                scope.pop();
            }
            scope.pop();
        }
        assert_eq!(gas_stack.meter().left(), Some(63));
        assert_eq!(gas_stack.meter().consumed(), 37);
    }

    #[test]
    fn test_gas_limit_stacking_no_root_limit() {
        let gas_stack: GasStack<256> = GasStack::new(None);
        assert_eq!(gas_stack.cur_gas_limit(), 0);
        gas_stack.meter().consume(100).unwrap();
        assert_eq!(gas_stack.meter().consumed(), 100);
        assert_eq!(gas_stack.meter().left(), None);
        {
            let tracker = GasTracker::limited(20);
            let scope = gas_stack.push(Some(&tracker)).unwrap();
            assert_eq!(gas_stack.meter().left(), Some(20));
            gas_stack.meter().consume(10).unwrap();
            assert_eq!(gas_stack.meter().left(), Some(10));
            {
                let tracker = GasTracker::limited(100);
                let scope = gas_stack.push(Some(&tracker)).unwrap();
                // cannot exceed parent limit
                assert_eq!(gas_stack.meter().left(), Some(10));
                {
                    let scope = gas_stack.push(None).unwrap();
                    // inherit child limit
                    assert_eq!(gas_stack.meter().left(), Some(10));
                    {
                        let tracker = GasTracker::limited(5);
                        let scope = gas_stack.push(Some(&tracker)).unwrap();
                        //cannot exceed parent limit
                        assert_eq!(gas_stack.meter().left(), Some(5));
                        assert_eq!(
                            gas_stack.meter().consume(6),
                            Err(ErrorCode::SystemCode(SystemCode::OutOfGas))
                        );
                        scope.pop();
                        assert_eq!(tracker.consumed.get(), 6);
                    }
                    assert_eq!(gas_stack.meter().left(), Some(4));
                    scope.pop();
                }
                scope.pop();
                assert_eq!(tracker.consumed.get(), 6);
            }
            scope.pop();
            assert_eq!(tracker.consumed.get(), 16);
        }
        assert_eq!(gas_stack.meter().left(), None);
        assert_eq!(gas_stack.gas.consumed.get(), 116);
    }
}
