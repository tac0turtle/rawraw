use crate::gas::GasMeter;
use arrayvec::ArrayVec;
use core::cell::RefCell;
use ixc_message_api::code::ErrorCode;

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
    scoped_gas_limit: u64,
}

impl<const CALL_STACK_LIMIT: usize> GasStack<CALL_STACK_LIMIT> {
    pub(crate) fn push(&self, scoped_gas_limit: Option<u64>) -> Result<(), ErrorCode> {
        let frame = if let Some(scoped_gas_limit) = scoped_gas_limit {
            // first get the amount of gas that has been consumed
            let gas_start = self.gas.consumed.get();
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
                scoped_gas_limit: new_limit,
            }
        } else {
            Frame {
                scoped_gas_limit: self.cur_gas_limit(),
            }
        };
        self.stack.borrow_mut().try_push(frame).map_err(|_| {
            ErrorCode::SystemCode(ixc_message_api::code::SystemCode::CallStackOverflow)
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

    pub(crate) fn pop(&self) -> Result<(), ErrorCode> {
        self.stack.borrow_mut().pop().ok_or(ErrorCode::SystemCode(
            ixc_message_api::code::SystemCode::FatalExecutionError,
        ))?;
        // when we pop, we update the gas meter's limit
        self.gas.limit.set(self.cur_gas_limit());
        Ok(())
    }

    pub(crate) fn meter(&self) -> &GasMeter {
        &self.gas
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
            gas_stack.push(Some(20)).unwrap();
            assert_eq!(gas_stack.meter().left(), Some(20));
            gas_stack.meter().consume(1).unwrap();
            assert_eq!(gas_stack.meter().left(), Some(19));
            {
                gas_stack.push(Some(10)).unwrap();
                assert_eq!(gas_stack.meter().left(), Some(10));
                {
                    gas_stack.push(None).unwrap();
                    // inherit child limit
                    assert_eq!(gas_stack.meter().left(), Some(10));
                    {
                        gas_stack.push(Some(100)).unwrap();
                        // cannot exceed parent limit
                        assert_eq!(gas_stack.meter().left(), Some(10));
                        // consume all the gas
                        assert_eq!(gas_stack.meter().consume(10), Ok(()));
                        gas_stack.pop().unwrap();
                    }
                    assert_eq!(gas_stack.meter().left(), Some(0));
                    assert_eq!(gas_stack.meter().consumed(), 11);
                    gas_stack.pop().unwrap();
                }

                assert_eq!(gas_stack.meter().left(), Some(0));
                // consuming any more causes an out of gas error
                assert_eq!(
                    gas_stack.meter().consume(1),
                    Err(ErrorCode::SystemCode(SystemCode::OutOfGas))
                );
                assert_eq!(gas_stack.meter().consumed(), 12);
                assert_eq!(gas_stack.meter().left(), Some(0));
                gas_stack.pop().unwrap();
            }
            assert_eq!(gas_stack.meter().left(), Some(8));
            assert_eq!(gas_stack.meter().consume(5), Ok(()));
            assert_eq!(gas_stack.meter().left(), Some(3));
            assert_eq!(gas_stack.meter().consumed(), 17);
            gas_stack.pop().unwrap();
        }
        assert_eq!(gas_stack.meter().left(), Some(83));
        assert_eq!(gas_stack.meter().consumed(), 17);
    }

    #[test]
    fn test_gas_limit_stacking_no_root_limit() {
        let gas_stack: GasStack<256> = GasStack::new(None);
        assert_eq!(gas_stack.cur_gas_limit(), 0);
        gas_stack.meter().consume(100).unwrap();
        assert_eq!(gas_stack.meter().consumed(), 100);
        assert_eq!(gas_stack.meter().left(), None);
        {
            gas_stack.push(Some(20)).unwrap();
            assert_eq!(gas_stack.meter().left(), Some(20));
            gas_stack.meter().consume(10).unwrap();
            assert_eq!(gas_stack.meter().left(), Some(10));
            {
                gas_stack.push(Some(100)).unwrap();
                // cannot exceed parent limit
                assert_eq!(gas_stack.meter().left(), Some(10));
                {
                    gas_stack.push(None).unwrap();
                    // inherit child limit
                    assert_eq!(gas_stack.meter().left(), Some(10));
                    {
                        gas_stack.push(Some(5)).unwrap();
                        //cannot exceed parent limit
                        assert_eq!(gas_stack.meter().left(), Some(5));
                        assert_eq!(gas_stack.meter().consume(6), Err(ErrorCode::SystemCode(SystemCode::OutOfGas)));
                        gas_stack.pop().unwrap();
                    }
                    assert_eq!(gas_stack.meter().left(), Some(4));
                    gas_stack.pop().unwrap();
                }
                gas_stack.pop().unwrap();
            }
            gas_stack.pop().unwrap();
        }
        assert_eq!(gas_stack.meter().left(), None);
        assert_eq!(gas_stack.gas.consumed.get(), 116);
    }
}
