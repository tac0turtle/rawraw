use crate::syntax::{SyntaxKind, SyntaxNode};

mod nodes;

pub trait AstStruct {
    const KIND: SyntaxKind;
}

pub struct ErrorNode;

impl AstStruct for ErrorNode {
    const KIND: SyntaxKind = SyntaxKind::ERROR_NODE;
}

