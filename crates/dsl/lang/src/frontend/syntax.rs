use rowan::Language;

mod syntax_kind;
pub use syntax_kind::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IXCLanguage {}

pub type SyntaxNode = rowan::SyntaxNode<IXCLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<IXCLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<IXCLanguage>;
pub type SyntaxNodePtr = rowan::ast::SyntaxNodePtr<IXCLanguage>;

impl Language for IXCLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        SyntaxKind::from(raw.0)
    }

    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

impl Into<rowan::SyntaxKind> for SyntaxKind {
    fn into(self) -> rowan::SyntaxKind {
        rowan::SyntaxKind(self.into())
    }
}
