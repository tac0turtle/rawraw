use crate::frontend::ast::*;
use crate::frontend::resolver::ids::AstPtr;

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
