use rowan::ast::AstPtr;
use crate::frontend::ast;
use crate::frontend::ast::Struct;
use crate::frontend::type_checker::item::ItemRef;
use crate::frontend::type_checker::Scope;

enum Type<'db> {
    Ident(ItemRef<'db>),
    Array(Box<Type<'db>>)
}

pub fn resolve_type<'db>(type_: ast::Type, scope: &'db Scope) -> Option<Type<'db>> {
    match type_ {
        ast::Type::TypeIdent(it) => {
            let name = it.name()?;
            Type::Ident(scope.resolve_symbol(name.text())?)
        }
        ast::Type::TypeArray(it) => {
            let ty = resolve_type(it.typ(), scope);
            Type::Array(ty)
        }
    }
}