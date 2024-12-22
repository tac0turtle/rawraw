use crate::frontend::ast::*;
use crate::frontend::checker::ids::AstPtr;

pub enum ItemRef<'db> {
    // TODO replace these with AstPtr based on node path which will be more stable when the file is mutated
    _Phantom(std::marker::PhantomData<&'db ()>),
    File(AstPtr<'db, File>),
    Interface(AstPtr<'db, Interface>),
    Object(AstPtr<'db, Object>),
    Struct(AstPtr<'db, Struct>),
    ImplFn(AstPtr<'db, ImplFn>),
    FnParam(AstPtr<'db, FnParam>),
}

pub enum ItemDef<'db> {
    ImplFn(FnSigDef<'db>),
    FnParam(FnParamDef<'db>),
}

pub struct FnSigDef<'db> {
    pub name: String,
    pub params: Vec<FnParamDef<'db>>,
    pub ret: Option<Type>,
}

pub struct FnParamDef<'db> {
    pub name: String,
    pub ty: Option<TypeId<'db>>,
}

pub enum TypeId<'db> {
    Array(Box<TypeId<'db>>),
    Named(ItemRef<'db>),
}