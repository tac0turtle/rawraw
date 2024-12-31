use rowan::ast::AstNode;
use crate::frontend::ast::*;
use crate::frontend::resolver::ids::AstPtr;
use crate::frontend::resolver::item_ref::ItemPtr;
use crate::frontend::syntax::IXCLanguage;

pub trait SymbolDefiner: AstNode<Language = IXCLanguage> {
    fn get_name(&self) -> Option<Name>;
    fn wrap_ptr(ptr: AstPtr<Self>) -> ItemPtr;
}

impl SymbolDefiner for Interface {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }

    fn wrap_ptr(ptr: AstPtr<Self>) -> ItemPtr {
        ItemPtr::Interface(ptr)
    }
}

impl SymbolDefiner for Object {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }

    fn wrap_ptr(ptr: AstPtr<Self>) -> ItemPtr {
        ItemPtr::Object(ptr)
    }
}

impl SymbolDefiner for Event {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }

    fn wrap_ptr(ptr: AstPtr<Self>) -> ItemPtr {
        ItemPtr::Event(ptr)
    }
}

impl SymbolDefiner for InterfaceFn {
    fn get_name(&self) -> Option<Name> {
        self.sig().map(|it| it.name()).flatten()
    }

    fn wrap_ptr(ptr: AstPtr<Self>) -> ItemPtr {
        ItemPtr::InterfaceFn(ptr)
    }
}

impl SymbolDefiner for MapCollection {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }

    fn wrap_ptr(ptr: AstPtr<Self>) -> ItemPtr {
        ItemPtr::MapCollection(ptr)
    }
}

impl SymbolDefiner for VarCollection {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }

    fn wrap_ptr(ptr: AstPtr<Self>) -> ItemPtr {
        ItemPtr::VarCollection(ptr)
    }
}