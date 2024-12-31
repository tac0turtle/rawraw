use crate::frontend::ast::{File, Item};
use crate::frontend::resolver::scope::{ScopeBuilder, ScopeProvider};

impl ScopeProvider for File {
    fn provide_scope(&self, scope: &mut ScopeBuilder) {
        scope.inherit_parent_scope();
        for item in self.items() {
            match item {
                Item::Interface(it) => scope.put_into_scope(it),
                Item::Object(it) => scope.put_into_scope(it),
                _ => {}
            }
        }
    }
}