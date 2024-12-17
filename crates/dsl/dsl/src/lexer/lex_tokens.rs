//! GENERATED CODE -- DO NOT EDIT!

use crate::syntax::SyntaxKind;
use logos::Logos;
#[derive(Logos, Debug, PartialEq, Eq, Clone)]
pub enum Token<'a> {
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
impl<'a> Token<'a> {
    pub fn kind(&'a self) -> SyntaxKind {
        match self {
            Token::Error(_) => SyntaxKind::ERROR,
            Token::Whitespace(_) => SyntaxKind::WHITESPACE,
            Token::Comment(_) => SyntaxKind::COMMENT,
            Token::InterfaceKw => SyntaxKind::INTERFACE_KW,
            Token::Ident(_) => SyntaxKind::IDENT,
            Token::LBracket => SyntaxKind::L_BRACKET,
            Token::RBracket => SyntaxKind::R_BRACKET,
            Token::HandlerKw => SyntaxKind::HANDLER_KW,
            Token::FnKw => SyntaxKind::FN_KW,
            Token::LParen => SyntaxKind::L_PAREN,
            Token::RParen => SyntaxKind::R_PAREN,
            Token::Colon => SyntaxKind::COLON,
            Token::LBrace => SyntaxKind::L_BRACE,
            Token::RBrace => SyntaxKind::R_BRACE,
            Token::Comma => SyntaxKind::COMMA,
        }
    }
    pub fn text(&'a self) -> &'a str {
        match self {
            Token::Error(x) => x,
            Token::Whitespace(x) => x,
            Token::Comment(x) => x,
            Token::InterfaceKw => "interface",
            Token::Ident(x) => x,
            Token::LBracket => "{",
            Token::RBracket => "}",
            Token::HandlerKw => "handler",
            Token::FnKw => "fn",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::Colon => ":",
            Token::LBrace => "[",
            Token::RBrace => "]",
            Token::Comma => ",",
        }
    }
}
