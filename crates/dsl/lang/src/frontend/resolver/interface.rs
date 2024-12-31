use crate::frontend::ast::{Interface, InterfaceItem};
use crate::frontend::resolver::members::{HasMembers, MemberSet};
use crate::frontend::resolver::scope::{ScopeBuilder, ScopeProvider};

impl ScopeProvider for Interface {
    fn provide_scope(&self, scope: &mut ScopeBuilder) {
        scope.inherit_parent_scope();
        scope.put_members_into_scope(self);
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