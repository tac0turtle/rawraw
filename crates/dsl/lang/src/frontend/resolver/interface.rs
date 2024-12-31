use crate::frontend::ast::{Interface, InterfaceItem};
use crate::frontend::resolver::scope::{ScopeBuilder, ScopeProvider};

impl ScopeProvider for Interface {
    fn provide_scope(&self, scope: &mut ScopeBuilder) {
        scope.inherit_parent_scope();
        for item in self.items() {
            match item {
                InterfaceItem::InterfaceFn(it) => scope.put_into_scope(it),
                InterfaceItem::Struct(it) => scope.put_into_scope(it),
                InterfaceItem::Event(it) => scope.put_into_scope(it),
                InterfaceItem::MapCollection(it) => scope.put_into_scope(it),
                InterfaceItem::VarCollection(it) => scope.put_into_scope(it),
            }
        }
    }
}
    
