use rowan::ast::AstNode;
use crate::frontend::syntax::IXCLanguage;

pub trait HasMembers: AstNode<Language = IXCLanguage> {
    fn provide_members(&self, member_set: &mut MemberSet);
}

pub struct MemberSet {}

impl MemberSet {
}