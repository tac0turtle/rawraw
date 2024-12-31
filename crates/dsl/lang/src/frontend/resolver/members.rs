use std::collections::BTreeMap;
use rowan::ast::AstNode;
use crate::frontend::ast::*;
use crate::frontend::resolver::node_id::{NodeId, NodePath};
use crate::frontend::resolver::symbol::{SymbolDefiner, SymbolId};
use crate::frontend::syntax::{IXCLanguage, SyntaxKind, SyntaxNode};

pub trait HasMembers: AstNode<Language = IXCLanguage> {
    fn provide_members(&self, member_set: &mut MemberSet);
}

pub fn as_has_members(syntax_node: SyntaxNode) -> Option<Box<dyn HasMembers>> {
    match syntax_node.kind() {
        SyntaxKind::INTERFACE => Some(Box::new(Interface::cast(syntax_node)?)),
        SyntaxKind::STRUCT => Some(Box::new(Struct::cast(syntax_node)?)),
        _ => None,
    }
}

pub struct MemberSet {
    pub(crate) node_id: NodeId,
    pub(crate) members: BTreeMap<String, SymbolId>,
}

impl MemberSet {
    pub fn new(node_id: NodeId) -> Self {
        Self {
            node_id,
            members: Default::default(),
        }
    }
    
    pub fn add<N: SymbolDefiner>(&mut self, node: N) {
        let path = NodePath::new(&node.syntax());
        let id = NodeId::new(self.node_id.filename.as_str(), path);
        if let Some(name) = node.get_name() {
            if let Some(name) = name.name() {
                self.members.insert(name.text().to_string(), SymbolId::Node(id));
            }
        }
    }
}