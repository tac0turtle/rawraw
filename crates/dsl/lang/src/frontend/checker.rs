use crate::frontend::ast::{Name, NameRef};
use crate::frontend::diagnostic::Diagnostic;
use crate::frontend::syntax::{SyntaxNode, SyntaxNodePtr, SyntaxToken};
use rowan::ast::{AstNode, AstPtr};
use salsa::{Accumulator, Database};
use std::collections::HashMap;

/// For a given Ident token which is in a definer role what is the node that it names?
fn name_defines(token: &Name) -> Option<SyntaxNodePtr> {
    None
}

fn scope_for_name_ref(token: &NameRef) -> Option<&Scope> {
    None
}

struct Scope {}

impl Scope {
    fn resolve(&self, name_ref: NameRef) -> Option<Name> {
        None
    }
}

struct ScopeProviderRegistry {
    // providers: HashMap<String, >,
}

impl ScopeProviderRegistry {
    fn register_provider<N: AstNode>(&mut self, provider: fn(N, &mut ScopeBuilder)) {}
}

struct ScopeBuilder<'db> {
    db: &'db dyn Database,
}

impl<'db> ScopeBuilder<'db> {
    pub fn provide_symbol_for_children<N: Resolver>(&mut self, name: Option<Name>, node: N) {
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

struct ResolveContext<'db> {
    db: &'db dyn Database,
}

enum DefinitionPointer {}

impl ResolveContext<'_> {
    fn resolve(&self, name: NameRef) {}
}

trait Resolver: AstNode {
    fn resolve(&self, ctx: &ResolveContext) -> Option<Definition>;
}

enum Definition {}
