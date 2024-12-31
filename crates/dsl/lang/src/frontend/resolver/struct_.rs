use crate::frontend::ast::{Name, Struct, StructField};
use crate::frontend::resolver::ids::AstPtr;
use crate::frontend::resolver::item_ref::ItemPtr;
use crate::frontend::resolver::members::{HasMembers, MemberSet};
use crate::frontend::resolver::symbol::SymbolDefiner;

impl SymbolDefiner for Struct {
    fn get_name(&self) -> Option<Name> {
        None
    }

    fn wrap_ptr(ptr: AstPtr<Self>) -> ItemPtr {
        ItemPtr::Struct(ptr)
    }
}

impl SymbolDefiner for StructField {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }

    fn wrap_ptr(ptr: AstPtr<Self>) -> ItemPtr {
        todo!()
    }
}

impl HasMembers for Struct {
    fn provide_members(&self, member_set: &mut MemberSet) {
        for field in self.fields() {
            member_set.add(field);
        }
    }
}
