use rowan::ast::{AstNode, AstPtr};
use crate::frontend::ast::{File, FnParam, ImplFn, Interface, Item, Object, ObjectItem};
use crate::frontend::type_checker::item::ItemRef;
use crate::frontend::type_checker::scope::ScopeBuilder;

pub trait BindSymbols: AstNode {
    fn bind_symbols<'db>(&self, builder: &mut ScopeBuilder<'db>) {}
    fn item_ref<'db>(ptr: AstPtr<Self>) -> ItemRef<'db>;
}

impl BindSymbols for File {
    fn bind_symbols<'db>(&self, builder: &mut ScopeBuilder<'db>) {
        for item in self.items() {
            match item {
                Item::Interface(it) => {
                    builder.bind_symbol(it.name(), it);
                }
                Item::Object(it) => {
                    builder.bind_symbol(it.name(), it);
                }
                _ => {}
            }
        }
    }

    fn item_ref<'db>(ptr: AstPtr<Self>) -> ItemRef<'db> {
        ItemRef::File(ptr)
    }
}

impl<'db> BindSymbols for Object {
    fn bind_symbols(&self, builder: &mut ScopeBuilder<'db>) {
        for item in self.items() {
            match item {
                ObjectItem::ImplFn(it) => {
                    builder.bind_symbol_with_children(it.sig().map(|it| it.name()).flatten(), it);
                }
                _ => {}
            }
        }
    }

    fn item_ref<'db>(ptr: AstPtr<Self>) -> ItemRef<'db> {
        ItemRef::Object(ptr)
    }
}

impl<'db> BindSymbols for ImplFn {
    fn bind_symbols<'db>(&self, builder: &mut ScopeBuilder<'db>) {
        if let Some(sig) = self.sig() {
            if let Some(args) = sig.args() {
                for arg in args.args() {
                    builder.bind_symbol(arg.name(), arg);
                }
            }
        }
    }

    fn item_ref<'db>(ptr: AstPtr<Self>) -> ItemRef<'db> {
        ItemRef::ImplFn(ptr)
    }
}

impl<'db> BindSymbols for Interface {
    fn item_ref<'db>(ptr: AstPtr<Self>) -> ItemRef<'db> {
        ItemRef::Interface(ptr)
    }
}

impl BindSymbols for FnParam {
    fn item_ref<'db>(ptr: AstPtr<Self>) -> ItemRef<'db> {
        ItemRef::FnParam(ptr)
    }
}