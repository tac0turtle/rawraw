use rowan::GreenNode;
use crate::frontend::syntax::{SyntaxKind, SyntaxNode};

mod nodes;

pub use nodes::*;
use crate::frontend::diagnostic::Diagnostic;

/// A trait for concrete AST nodes,
/// not enums where the parent syntax kind is actually unmaterialized in the tree.
pub trait ConcreteNode {
    const KIND: SyntaxKind;
}

pub struct ErrorNode;

impl ConcreteNode for ErrorNode {
    const KIND: SyntaxKind = SyntaxKind::ERROR_NODE;
}

#[derive(Clone)]
pub struct ParsedAST {
    pub root: GreenNode,
    pub diagnostics: Vec<Diagnostic>,
}

impl ParsedAST {
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.root.clone())
    }
}