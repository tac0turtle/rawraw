use rowan::ast::AstNode;
use salsa::Database;
use crate::frontend::syntax::{SyntaxElement, SyntaxNode, SyntaxToken};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AstPtr<'db, N: AstNode> {
    _marker: std::marker::PhantomData<N>,
    path: NodeId<'db>,
}

impl<'db, N: AstNode> AstPtr<'db, N> {
    pub fn new(db: &'db dyn Database, file_id: &FileId, node: &N) -> Option<Self> {
        todo!()
    }

    pub fn resolve(&self, db: &'db dyn Database, file_id: &FileId) -> Option<N> {
        // let resolve_src = db.file_line_index(file_id)?;
        // let node = self.path.path(db).resolve(node)?;
        // N::cast(node)
        todo!()
    }
}

#[salsa::interned]
pub struct NodeId<'db> {
    #[return_ref]
    pub path: NodePath,
    #[return_ref]
    pub file_id: FileId<'db>,
}

#[salsa::interned]
pub struct FileId<'db> {
    #[return_ref]
    pub url: String,
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
}

// #[salsa::interned]
// pub struct TokenId<'db> {
//     #[return_ref]
//     pub path: TokenPath,
// }
//
// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
// pub struct TokenPath(NodePath, usize);
//
// impl TokenPath {
//     pub fn resolve(&self, node: &SyntaxNode) -> Option<SyntaxToken> {
//         let node = self.0.resolve(node)?;
//         let elem = node.children_with_tokens().nth(self.1)?;
//         elem.into_token()
//     }
// }
//
