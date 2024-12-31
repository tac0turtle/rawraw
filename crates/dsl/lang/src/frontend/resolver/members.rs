use std::collections::BTreeMap;
use rowan::ast::AstNode;
use crate::frontend::ast::*;
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
    members: BTreeMap<String, SymbolId>,
}

impl MemberSet {
    pub fn add<N: SymbolDefiner>(&mut self, node: N) {
        // TODO
    }
}