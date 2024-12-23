pub(crate) struct ScopeGuard<'a, T: ScopeGuardStack> {
    value: &'a T,
    popped: bool,
}

pub(crate) trait ScopeGuardStack {
    fn pop(&self);
}

impl<'a, T: ScopeGuardStack> ScopeGuard<'a, T> {
    pub(crate) fn new(value: &'a T) -> Self {
        Self {
            value,
            popped: false,
        }
    }

    pub(crate) fn pop(mut self) { self.do_pop(); }

    pub(crate) fn do_pop(&mut self) {
        if !self.popped {
            self.value.pop();
            self.popped = true;
        }
    }
}

impl<T: ScopeGuardStack> Drop for ScopeGuard<'_, T> {
    fn drop(&mut self) {
        self.do_pop();
    }
}