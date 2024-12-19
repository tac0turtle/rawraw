use rowan::GreenNode;
use crate::syntax::{SyntaxKind, SyntaxNode};

mod nodes;

pub use nodes::*;

pub trait AstStruct {
    const KIND: SyntaxKind;
}

pub struct ErrorNode;

impl AstStruct for ErrorNode {
    const KIND: SyntaxKind = SyntaxKind::ERROR_NODE;
}

#[salsa::tracked]
pub struct ParsedAST<'db> {
    #[return_ref] pub root: GreenNode,
}