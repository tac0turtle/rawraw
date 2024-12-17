//! GENERATED CODE -- DO NOT EDIT!

use crate::syntax::SyntaxKind;
use logos::Logos;
#[derive(Logos, Debug, PartialEq, Eq, Clone)]
pub enum LexicalToken<'a> {
    #[regex(r#"[ \t\n\r\f\v]+"#)]
    Whitespace(&'a str),
    #[regex(r#"//[^\n\r\f\v]*"#)]
    Comment(&'a str),
    #[token("interface")]
    Interface,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex|lex.slice())]
    Ident(&'a str),
    #[token("{")]
    LBracket,
    #[token("}")]
    RBracket,
    #[token("handler")]
    Handler,
    #[token("fn")]
    Fn,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token(":")]
    Colon,
    #[token("[")]
    LBrace,
    #[token("]")]
    RBrace,
}
impl<'a> From<LexicalToken<'a>> for crate::syntax::SyntaxKind {
    fn from(value: LexicalToken<'a>) -> Self {
        match value {
            LexicalToken::Whitespace(_) => SyntaxKind::WHITESPACE,
            LexicalToken::Comment(_) => SyntaxKind::COMMENT,
            LexicalToken::Interface => SyntaxKind::INTERFACE,
            LexicalToken::Ident(_) => SyntaxKind::IDENT,
            LexicalToken::LBracket => SyntaxKind::L_BRACKET,
            LexicalToken::RBracket => SyntaxKind::R_BRACKET,
            LexicalToken::Handler => SyntaxKind::HANDLER,
            LexicalToken::Fn => SyntaxKind::FN,
            LexicalToken::LParen => SyntaxKind::L_PAREN,
            LexicalToken::RParen => SyntaxKind::R_PAREN,
            LexicalToken::Colon => SyntaxKind::COLON,
            LexicalToken::LBrace => SyntaxKind::L_BRACE,
            LexicalToken::RBrace => SyntaxKind::R_BRACE,
        }
    }
}
impl<'a> std::fmt::Display for LexicalToken<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexicalToken::Whitespace(x) => write!(f, "{}", x),
            LexicalToken::Comment(x) => write!(f, "{}", x),
            LexicalToken::Interface => write!(f, "{}", "interface"),
            LexicalToken::Ident(x) => write!(f, "{}", x),
            LexicalToken::LBracket => write!(f, "{}", "{"),
            LexicalToken::RBracket => write!(f, "{}", "}"),
            LexicalToken::Handler => write!(f, "{}", "handler"),
            LexicalToken::Fn => write!(f, "{}", "fn"),
            LexicalToken::LParen => write!(f, "{}", "("),
            LexicalToken::RParen => write!(f, "{}", ")"),
            LexicalToken::Colon => write!(f, "{}", ":"),
            LexicalToken::LBrace => write!(f, "{}", "["),
            LexicalToken::RBrace => write!(f, "{}", "]"),
        }
    }
}
