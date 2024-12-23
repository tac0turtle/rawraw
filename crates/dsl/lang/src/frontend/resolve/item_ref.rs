use crate::frontend::ast::*;
use crate::frontend::resolve::ids::AstPtr;

pub enum ItemPtr<'db> {
    File(AstPtr<'db, File>),
    Interface(AstPtr<'db, Interface>),
    Object(AstPtr<'db, Object>),
    Struct(AstPtr<'db, Struct>),
    ImplFn(AstPtr<'db, ImplFn>),
    FnParam(AstPtr<'db, FnParam>),
    Event(AstPtr<'db, Event>),
    InterfaceFn(AstPtr<'db, InterfaceFn>),
    MapCollection(AstPtr<'db, MapCollection>),
    VarCollection(AstPtr<'db, VarCollection>),
}
