use rowan::ast::AstNode;
use salsa::{Accumulator, Database};
use crate::frontend::ast::Name;
use crate::frontend::checker::resolver::ItemDefiner;
use crate::frontend::diagnostic::Diagnostic;

pub struct ScopeProviderRegistry {
    // providers: HashMap<String, >,
}

impl ScopeProviderRegistry {
    fn register_provider<N: AstNode>(&mut self, provider: fn(N, &mut ScopeBuilder)) {}
}

pub struct ScopeBuilder<'db> {
    db: &'db dyn Database,
}

impl<'db> ScopeBuilder<'db> {
    pub fn provide_symbol_for_children<N: ItemDefiner>(&mut self, node: N) {
        if let Some(name) = name {
            // self.symbols
            //     .insert(name.text().to_string(), N::item_ref(AstPtr::new(&node)));
        } else {
            // TODO: report diagnostic
        }
    }

    pub fn inherit_parent_node_scope(&mut self) {}

    pub fn report_diagnostic(&self, diagnostic: Diagnostic) {
        diagnostic.accumulate(self.db)
    }
}

