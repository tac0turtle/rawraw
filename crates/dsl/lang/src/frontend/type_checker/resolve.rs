use rowan::ast::AstNode;
use salsa::Database;
use crate::frontend::ast::*;
use crate::frontend::type_checker::item::*;
use crate::frontend::type_checker::scope::Scope;
use crate::frontend::type_checker::type_::resolve_type;

trait ResolveSymbols<'db>: AstNode {
    type Resolved;

    fn resolve_symbols(&self, scope: &Scope<'db>) -> Option<Self::Resolved>;

    fn resolve_item(r: Self::Resolved) -> ResolvedItem<'db>;
}

impl<'db> ResolveSymbols<'db> for FnSignature {
    type Resolved = ();

    fn resolve_symbols(&self, scope: &Scope<'db>) -> Option<Self::Resolved> {
        todo!()
    }

    fn resolve_item(r: Self::Resolved) -> ResolvedItem<'db> {
        todo!()
    }
}

impl<'db> ResolveSymbols<'db> for FnParam {
    type Resolved = ResolvedFnParam;

    fn resolve_symbols(&self, scope: &Scope<'db>) -> Option<Self::Resolved> {
        let ty = self.ty()?;
        let ty = resolve_type(ty, scope);
        Some(ResolvedFnParam {
            name: self.name()?.text().to_string(),
            ty,
        })
    }

    fn resolve_item(r: Self::Resolved) -> ResolvedItem<'db> {
        ResolvedItem::FnParam(r)
    }
}