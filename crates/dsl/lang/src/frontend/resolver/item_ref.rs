use crate::frontend::ast::*;
use crate::frontend::resolver::ids::{AstPtr, NodePath};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ItemPtr {
    File(AstPtr<File>),
    Interface(AstPtr<Interface>),
    Object(AstPtr<Object>),
    Struct(AstPtr<Struct>),
    ImplFn(AstPtr<ImplFn>),
    FnParam(AstPtr<FnParam>),
    Event(AstPtr<Event>),
    InterfaceFn(AstPtr<InterfaceFn>),
    MapCollection(AstPtr<MapCollection>),
    VarCollection(AstPtr<VarCollection>),
}

impl ItemPtr {
    pub fn node_path(&self) -> NodePath {
        match self {
            ItemPtr::File(ptr) => ptr.path.clone(),
            ItemPtr::Interface(ptr) => ptr.path.clone(),
            ItemPtr::Object(ptr) => ptr.path.clone(),
            ItemPtr::Struct(ptr) => ptr.path.clone(),
            ItemPtr::ImplFn(ptr) => ptr.path.clone(),
            ItemPtr::FnParam(ptr) => ptr.path.clone(),
            ItemPtr::Event(ptr) => ptr.path.clone(),
            ItemPtr::InterfaceFn(ptr) => ptr.path.clone(),
            ItemPtr::MapCollection(ptr) => ptr.path.clone(),
            ItemPtr::VarCollection(ptr) => ptr.path.clone(),
        }
    }
}
