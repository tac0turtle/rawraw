use crate::frontend::syntax::{IXCLanguage, SyntaxNode};
use rowan::ast::AstNode;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AstPtr<N: AstNode + ?Sized> {
    _marker: std::marker::PhantomData<fn(N)>,
    path: NodePath,
}

impl<N: AstNode<Language = IXCLanguage>> AstPtr<N> {
    pub fn new(node: &N) -> Self {
        let path = NodePath::new(node.syntax());
        AstPtr {
            _marker: Default::default(),
            path,
        }
    }

    pub fn resolve(&self, node: &SyntaxNode) -> Option<N> {
        let node = self.path.resolve(node)?;
        N::cast(node)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodePath(Vec<usize>);

impl NodePath {
    pub fn new(node: &SyntaxNode) -> Self {
        let mut path = vec![node.index()];
        while let Some(node) = node.parent() {
            path.push(node.index());
        }
        NodePath(path)
    }

    pub fn resolve(&self, node: &SyntaxNode) -> Option<SyntaxNode> {
        let mut node = node.clone();
        let mut idx = self.0.len();
        while idx > 0 {
            let i = self.0[idx - 1];
            node = node.children().nth(i)?;
            idx -= 1;
        }
        Some(node)
    }

    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }

    pub fn parent_path(&self) -> Option<NodePath> {
        if self.0.len() < 1 {
            return None;
        }
        Some(NodePath(self.0[1..].to_vec()))
    }
}
