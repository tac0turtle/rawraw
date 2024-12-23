use std::collections::HashMap;
use rowan::ast::{AstNode, AstPtr};
use salsa::{Accumulator, Database};
use crate::frontend::ast::*;
use crate::frontend::diagnostic::Diagnostic;
use crate::frontend::syntax::{SyntaxNode, SyntaxToken};
use crate::frontend::type_checker::bind::BindSymbols;
use crate::frontend::type_checker::item::ItemRef;

pub struct Scope<'db> {
    parent: Option<ItemRef<'db>>,
    symbols: HashMap<String, ItemRef<'db>>,
}

pub(crate) struct ScopeBuilder<'db> {
    db: &'db dyn Database,
    scope_node: SyntaxNode,
    symbols: HashMap<String, ItemRef<'db>>,
}

impl<'db> ScopeBuilder<'db> {
    pub fn bind_symbol<N: BindSymbols>(&mut self, name: Option<SyntaxToken>, node: N) {
        if let Some(name) = name {
            // self.symbols.insert(name.text().to_string(), N::item_ref(AstPtr::new(&node)));
        } else {
            // TODO: report diagnostic
        }
    }

    pub fn report_diagnostic(&self, diagnostic: Diagnostic) {
        diagnostic.accumulate(self.db)
    }
}

impl<'db> Scope<'db> {
    pub fn resolve_symbol(&self, name: &str) -> Option<ItemRef<'db>> {
        None
    }
}