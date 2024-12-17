//! GENERATED CODE -- DO NOT EDIT!

use crate::syntax::SyntaxKind;
use logos::Logos;
#[derive(Logos, Debug, PartialEq, Eq, Clone)]
pub enum Token<'a> {
    Error(&'a str),
    Eof,
    #[regex(r#"[ \t\n\r\f\v]+"#)]
    Whitespace(&'a str),
    #[regex(r#"//[^\n\r\f\v]*"#)]
    LineComment(&'a str),
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
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("tx")]
    TxKw,
    #[token("query")]
    QueryKw,
    #[token("pure")]
    PureKw,
    #[token("key")]
    KeyKw,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("[")]
    LBrace,
    #[token("]")]
    RBrace,
}
impl<'a> Token<'a> {
    pub fn kind(&'a self) -> SyntaxKind {
        match self {
            Token::Error(_) => SyntaxKind::ERROR,
            Token::Eof => SyntaxKind::EOF,
            Token::Whitespace(_) => SyntaxKind::WHITESPACE,
            Token::LineComment(_) => SyntaxKind::LINE_COMMENT,
            Token::InterfaceKw => SyntaxKind::INTERFACE_KW,
            Token::Ident(_) => SyntaxKind::IDENT,
            Token::LBracket => SyntaxKind::L_BRACKET,
            Token::RBracket => SyntaxKind::R_BRACKET,
            Token::HandlerKw => SyntaxKind::HANDLER_KW,
            Token::LParen => SyntaxKind::L_PAREN,
            Token::RParen => SyntaxKind::R_PAREN,
            Token::TxKw => SyntaxKind::TX_KW,
            Token::QueryKw => SyntaxKind::QUERY_KW,
            Token::PureKw => SyntaxKind::PURE_KW,
            Token::KeyKw => SyntaxKind::KEY_KW,
            Token::Colon => SyntaxKind::COLON,
            Token::Comma => SyntaxKind::COMMA,
            Token::LBrace => SyntaxKind::L_BRACE,
            Token::RBrace => SyntaxKind::R_BRACE,
        }
    }
    pub fn text(&'a self) -> &'a str {
        match self {
            Token::Error(x) => x,
            Token::Eof => "",
            Token::Whitespace(x) => x,
            Token::LineComment(x) => x,
            Token::InterfaceKw => "interface",
            Token::Ident(x) => x,
            Token::LBracket => "{",
            Token::RBracket => "}",
            Token::HandlerKw => "handler",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::TxKw => "tx",
            Token::QueryKw => "query",
            Token::PureKw => "pure",
            Token::KeyKw => "key",
            Token::Colon => ":",
            Token::Comma => ",",
            Token::LBrace => "[",
            Token::RBrace => "]",
        }
    }
}
