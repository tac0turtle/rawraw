use crate::frontend::ast::Impl;
use crate::frontend::resolver::scope::{resolve_scope, ScopeBuilder, ScopeProvider};
use crate::frontend::resolver::symbol::SymbolId;

impl ScopeProvider for Impl {
    fn provide_scope(&self, scope: &mut ScopeBuilder) {
        if let Some(name_ref) = self.name_ref() {
            if let Some(SymbolId::Node(name_ref)) = scope.resolve_name_ref(&name_ref) {
                if let Some(impL_scope) = scope.resolve_scope(&name_ref) {
                    scope.add_scope(&impL_scope);
                }
            }
        }
    }
}