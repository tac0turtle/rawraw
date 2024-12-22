mod ids;
mod scope;
mod resolver;
mod item;
mod provide;

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

struct ResolveContext<'db> {
    db: &'db dyn Database,
}

enum ItemPtr {}

enum Definition {}
