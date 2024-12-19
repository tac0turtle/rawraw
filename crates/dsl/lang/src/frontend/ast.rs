use rowan::GreenNode;
use salsa::Database;
use crate::frontend::syntax::{SyntaxKind, SyntaxNode};

mod nodes;

pub use nodes::*;

/// A trait for concrete AST nodes,
/// not enums where the parent syntax kind is actually unmaterialized in the tree.
pub trait ConcreteNode {
    const KIND: SyntaxKind;
}

pub struct ErrorNode;

impl ConcreteNode for ErrorNode {
    const KIND: SyntaxKind = SyntaxKind::ERROR_NODE;
}

#[salsa::tracked]
pub struct ParsedAST<'db> {
    #[return_ref] pub root: GreenNode,
}

impl <'db> ParsedAST<'db> {
    pub fn syntax(&self, db: &'db dyn Database) -> SyntaxNode {
        SyntaxNode::new_root(self.root(db).clone())
    }
}