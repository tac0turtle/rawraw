use crate::frontend::syntax::{IXCLanguage, SyntaxNode};
use rowan::ast::AstNode;
use salsa::Database;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AstPtr<'db, N: AstNode + ?Sized> {
    _marker: std::marker::PhantomData<fn(N)>,
    path: NodeId<'db>,
}

// impl <N: AstNode + ?Sized> Debug for AstPtr<'_, N> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "AstPtr({:?})", self.path.path(self.db()).resolve(&self.syntax()))
//     }
// }

impl<'db, N: AstNode<Language = IXCLanguage>> AstPtr<'db, N> {
    pub fn new(db: &'db dyn Database, node: &N) -> Self {
        let path = NodePath::new(node.syntax());
        let id = NodeId::new(db, path);
        AstPtr {
            _marker: Default::default(),
            path: id,
        }
    }

    pub fn resolve(&self, db: &'db dyn Database, node: &SyntaxNode) -> Option<N> {
        let node = self.path.path(db).resolve(node)?;
        N::cast(node)
    }
}

#[salsa::interned]
pub struct NodeId<'db> {
    #[return_ref]
    pub path: NodePath,
    // #[return_ref]
    // pub file_id: FileId<'db>,
}

impl<'db> NodeId<'db> {
    pub fn parent_path(&self, db: &'db dyn Database) -> Option<NodeId<'db>> {
        if let Some(parent) = self.path(db).parent_path() {
            return Some(Self::new(db, parent))
        }
        None
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
