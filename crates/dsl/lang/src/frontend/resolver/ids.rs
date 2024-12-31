use crate::frontend::syntax::{IXCLanguage, SyntaxKind, SyntaxNode, SyntaxNodePtr};
use rowan::ast::AstNode;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AstPtr<N: AstNode + ?Sized> {
    _marker: std::marker::PhantomData<fn(N)>,
    pub path: NodePath,
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
pub struct NodePath(
    // TODO: replace with a Vec<usize> when we can fix that implementation because it is more stable with respect to formatting changes
    Vec<(SyntaxKind, usize)>
    // SyntaxNodePtr
);

impl NodePath {
    pub fn new(node: &SyntaxNode) -> Self {
        let mut path = vec![];
        let mut node = node.clone();
        while let Some(parent) = node.parent() {
            let kind = node.kind();
            let mut index = 0;
            while let Some(sibling) = node.prev_sibling() {
                if sibling.kind() == kind {
                    index += 1;
                }
                node = sibling;
            }
            path.push((kind, index));
            node = parent;
        }
        NodePath(path)
    }

    pub fn resolve(&self, root: &SyntaxNode) -> Option<SyntaxNode> {
        fn find_nth_child_by_kind(node: &SyntaxNode, kind: SyntaxKind, index: usize) -> Option<SyntaxNode> {
            let mut i = 0;
            for child in node.children() {
                if child.kind() == kind {
                    if index == i {
                        return Some(child);
                    }
                    i += 1;
                }
            }
            None
        }

        let mut node = root.clone();
        for (kind, index) in self.0.iter().rev() {
            if let Some(child) = find_nth_child_by_kind(&node, *kind, *index) {
                node = child;
            } else {
                return None;
            }
        }
        Some(node)
    }

    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }

    pub fn parent_path(&self) -> Option<NodePath> {
        if self.0.is_empty() {
            return None;
        }
        let path = self.0[1..].to_vec();
        Some(NodePath(path))
    }
}
