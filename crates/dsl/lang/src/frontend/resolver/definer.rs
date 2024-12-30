use rowan::ast::AstNode;
use crate::frontend::ast::*;
use crate::frontend::resolver::ids::AstPtr;
use crate::frontend::resolver::item_ref::ItemPtr;
use crate::frontend::syntax::IXCLanguage;

pub trait ItemDefiner: AstNode<Language = IXCLanguage> {
    fn get_name(&self) -> Option<Name>;
    fn wrap_ptr(ptr: AstPtr<Self>) -> ItemPtr;
}

impl ItemDefiner for Interface {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }

    fn wrap_ptr(ptr: AstPtr<Self>) -> ItemPtr {
        ItemPtr::Interface(ptr)
    }
}

impl ItemDefiner for Object {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }

    fn wrap_ptr(ptr: AstPtr<Self>) -> ItemPtr {
        ItemPtr::Object(ptr)
    }
}

impl ItemDefiner for Struct {
    fn get_name(&self) -> Option<Name> {
        None
    }

    fn wrap_ptr(ptr: AstPtr<Self>) -> ItemPtr {
        ItemPtr::Struct(ptr)
    }
}

impl ItemDefiner for Event {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }

    fn wrap_ptr(ptr: AstPtr<Self>) -> ItemPtr {
        ItemPtr::Event(ptr)
    }
}

impl ItemDefiner for InterfaceFn {
    fn get_name(&self) -> Option<Name> {
        self.sig().map(|it| it.name()).flatten()
    }

    fn wrap_ptr(ptr: AstPtr<Self>) -> ItemPtr {
        ItemPtr::InterfaceFn(ptr)
    }
}

impl ItemDefiner for MapCollection {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }

    fn wrap_ptr(ptr: AstPtr<Self>) -> ItemPtr {
        ItemPtr::MapCollection(ptr)
    }
}

impl ItemDefiner for VarCollection {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }

    fn wrap_ptr(ptr: AstPtr<Self>) -> ItemPtr {
        ItemPtr::VarCollection(ptr)
    }
}