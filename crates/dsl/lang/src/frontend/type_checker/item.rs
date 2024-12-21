use rowan::ast::AstPtr;
use crate::frontend::ast::*;
// use crate::frontend::type_checker::ids::AstPtr;

pub enum ItemRef<'db> {
    // TODO replace these with AstPtr based on node path which will be more stable when the file is mutated
    _Phantom(std::marker::PhantomData<&'db ()>),
    File(AstPtr<File>),
    Interface(AstPtr<Interface>),
    Object(AstPtr<Object>),
    Struct(AstPtr<Struct>),
    ImplFn(AstPtr<ImplFn>),
    FnParam(AstPtr<FnParam>),
}

pub enum ResolvedItem<'db> {
    ImplFn(ResolvedFnSig),
    FnParam(ResolvedFnParam),
    _Phantom(std::marker::PhantomData<&'db ()>),
}

pub struct ResolvedFnSig {
    pub name: String,
    pub params: Vec<ResolvedFnParam>,
    pub ret: Option<Type>,
}

pub struct ResolvedFnParam {
    pub name: String,
    pub ty: Option<Type>
}