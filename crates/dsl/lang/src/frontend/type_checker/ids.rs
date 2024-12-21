use rowan::ast::AstNode;
use salsa::Database;
use crate::frontend::syntax::{SyntaxElement, SyntaxNode, SyntaxToken};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AstPtr<'db, N: AstNode> {
    _marker: std::marker::PhantomData<N>,
    path: NodeId<'db>,
}

impl<'a, N: AstNode> AstPtr<'a, N> {
    pub fn resolve(&self, db: &dyn Database, node: &SyntaxNode) -> Option<N> {
        let node = self.path.path(db).resolve(node)?;
        N::cast(node)
    }
}

#[salsa::interned]
pub struct NodeId<'db> {
    #[return_ref]
    pub path: NodePath,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodePath(Vec<usize>);

impl NodePath {
    pub fn resolve(&self, node: &SyntaxNode) -> Option<SyntaxNode> {
        let mut node = node.clone();
        let mut idx = 0;
        for i in self.0.iter() {
            node = node.children().nth(*i)?;
        }
        Some(node)
    }
}

#[salsa::interned]
pub struct TokenId<'db> {
    #[return_ref]
    pub path: TokenPath,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TokenPath(NodePath, usize);

impl TokenPath {
    pub fn resolve(&self, node: &SyntaxNode) -> Option<SyntaxToken> {
        let node = self.0.resolve(node)?;
        let elem = node.children_with_tokens().nth(self.1)?;
        elem.into_token()
    }
}

