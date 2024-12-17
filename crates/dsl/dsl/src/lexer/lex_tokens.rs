//! GENERATED CODE -- DO NOT EDIT!

use crate::syntax::SyntaxKind;
use logos::Logos;
#[derive(Logos, Debug, PartialEq, Eq, Clone)]
pub enum LexicalToken<'a> {
    Error(&'a str),
    #[regex(r#"[ \t\n\r\f\v]+"#)]
    Whitespace(&'a str),
    #[regex(r#"//[^\n\r\f\v]*"#)]
    Comment(&'a str),
    #[token("interface")]
    InterfaceKw,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex|lex.slice())]
    Ident(&'a str),
    #[token("{")]
    LBracket,
    #[token("}")]
    RBracket,
    #[token("handler")]
    HandlerKw,
    #[token("fn")]
    FnKw,
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
    #[token(",")]
    Comma,
}
impl<'a> LexicalToken<'a> {
    pub fn kind(&'a self) -> SyntaxKind {
        match self {
            LexicalToken::Error(_) => SyntaxKind::ERROR,
            LexicalToken::Whitespace(_) => SyntaxKind::WHITESPACE,
            LexicalToken::Comment(_) => SyntaxKind::COMMENT,
            LexicalToken::InterfaceKw => SyntaxKind::INTERFACE_KW,
            LexicalToken::Ident(_) => SyntaxKind::IDENT,
            LexicalToken::LBracket => SyntaxKind::L_BRACKET,
            LexicalToken::RBracket => SyntaxKind::R_BRACKET,
            LexicalToken::HandlerKw => SyntaxKind::HANDLER_KW,
            LexicalToken::FnKw => SyntaxKind::FN_KW,
            LexicalToken::LParen => SyntaxKind::L_PAREN,
            LexicalToken::RParen => SyntaxKind::R_PAREN,
            LexicalToken::Colon => SyntaxKind::COLON,
            LexicalToken::LBrace => SyntaxKind::L_BRACE,
            LexicalToken::RBrace => SyntaxKind::R_BRACE,
            LexicalToken::Comma => SyntaxKind::COMMA,
        }
    }
    pub fn text(&'a self) -> &'a str {
        match self {
            LexicalToken::Error(x) => x,
            LexicalToken::Whitespace(x) => x,
            LexicalToken::Comment(x) => x,
            LexicalToken::InterfaceKw => "interface",
            LexicalToken::Ident(x) => x,
            LexicalToken::LBracket => "{",
            LexicalToken::RBracket => "}",
            LexicalToken::HandlerKw => "handler",
            LexicalToken::FnKw => "fn",
            LexicalToken::LParen => "(",
            LexicalToken::RParen => ")",
            LexicalToken::Colon => ":",
            LexicalToken::LBrace => "[",
            LexicalToken::RBrace => "]",
            LexicalToken::Comma => ",",
        }
    }
}
