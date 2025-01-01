use crate::frontend::ast::{Interface, InterfaceItem};
use crate::frontend::resolver::members::{HasMembers, MemberSet};
use crate::frontend::resolver::scope::{ScopeBuilder, ScopeProvider};

impl ScopeProvider for Interface {
    fn provide_scope(&self, scope: &mut ScopeBuilder) {
        scope.inherit_parent_scope();
        for item in self.items() {
            match item {
                InterfaceItem::Struct(it) => scope.put_into_scope(it),
                InterfaceItem::Event(it) => scope.put_into_scope(it),
                _ => {} // other items don't go into scope locally
            }
        }
    }
}
    
impl HasMembers for Interface {
    fn provide_members(&self, member_set: &mut MemberSet) {
        for item in self.items() {
            match item {
                InterfaceItem::InterfaceFn(it) => member_set.add(it),
                InterfaceItem::Struct(it) => member_set.add(it),
                InterfaceItem::Event(it) => member_set.add(it),
                InterfaceItem::MapCollection(it) => member_set.add(it),
                InterfaceItem::VarCollection(it) => member_set.add(it),
            }
        }
    }
}