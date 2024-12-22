use rowan::ast::AstNode;
use salsa::Database;
use crate::frontend::ast::{Name, NameRef};
use crate::frontend::checker::ids::AstPtr;
use crate::frontend::checker::ItemPtr;

struct ResolveContext<'db> {
    db: &'db dyn Database,
}

impl ResolveContext<'_> {
    fn resolve(&self, name: NameRef) -> ItemPtr {}
}

pub trait ItemDefiner: AstNode {
    fn wrap_ptr(ptr: AstPtr<Self>) -> ItemPtr;
    fn resolve_name(&self) -> Option<Name>;
    type Definition;
    fn resolve_definition(&self, ctx: &ResolveContext) -> Option<Self::Definition>;
    // fn resolve(&self, ctx: &ResolveContext) -> Option<ItemPtr>;

}

