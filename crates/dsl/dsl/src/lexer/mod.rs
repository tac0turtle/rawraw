use rowan::GreenToken;

include!(concat!(env!("OUT_DIR"), "/lex_tokens.rs"));

impl <'a> From<LexicalToken<'a>> for GreenToken {
    fn from(value: LexicalToken<'a>) -> Self {
        let kind: crate::syntax::SyntaxKind = value.clone().into();
        let rowan_kind: rowan::SyntaxKind = rowan::SyntaxKind(kind.into());
        let value = format!("{}", value);
        GreenToken::new(rowan_kind, &value)
    }
}

