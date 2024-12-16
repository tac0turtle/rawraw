use rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IXCLanguage {}

include!(concat!(env!("OUT_DIR"), "/syntax_kind.rs"));

impl Language for IXCLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        // TODO: add default case
        SyntaxKind::try_from(raw.0).unwrap()
    }

    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}
