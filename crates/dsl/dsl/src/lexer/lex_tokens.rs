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
    LCurly,
    #[token("}")]
    RCurly,
    #[token(";")]
    Semicolon,
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
    #[token("emits")]
    EmitsKw,
    #[token("[")]
    LSquare,
    #[token("]")]
    RSquare,
    #[token("struct")]
    StructKw,
    #[token("event")]
    EventKw,
    #[token("map")]
    MapKw,
    #[token("=>")]
    RArrow,
    #[token("client")]
    ClientKw,
    #[token("impl")]
    ImplKw,
    #[token("for")]
    ForKw,
    #[token(".")]
    Dot,
    #[token("=")]
    Eq,
    #[token("in")]
    InKw,
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
            Token::LCurly => SyntaxKind::L_CURLY,
            Token::RCurly => SyntaxKind::R_CURLY,
            Token::Semicolon => SyntaxKind::SEMICOLON,
            Token::HandlerKw => SyntaxKind::HANDLER_KW,
            Token::LParen => SyntaxKind::L_PAREN,
            Token::RParen => SyntaxKind::R_PAREN,
            Token::TxKw => SyntaxKind::TX_KW,
            Token::QueryKw => SyntaxKind::QUERY_KW,
            Token::PureKw => SyntaxKind::PURE_KW,
            Token::KeyKw => SyntaxKind::KEY_KW,
            Token::Colon => SyntaxKind::COLON,
            Token::Comma => SyntaxKind::COMMA,
            Token::EmitsKw => SyntaxKind::EMITS_KW,
            Token::LSquare => SyntaxKind::L_SQUARE,
            Token::RSquare => SyntaxKind::R_SQUARE,
            Token::StructKw => SyntaxKind::STRUCT_KW,
            Token::EventKw => SyntaxKind::EVENT_KW,
            Token::MapKw => SyntaxKind::MAP_KW,
            Token::RArrow => SyntaxKind::R_ARROW,
            Token::ClientKw => SyntaxKind::CLIENT_KW,
            Token::ImplKw => SyntaxKind::IMPL_KW,
            Token::ForKw => SyntaxKind::FOR_KW,
            Token::Dot => SyntaxKind::DOT,
            Token::Eq => SyntaxKind::EQ,
            Token::InKw => SyntaxKind::IN_KW,
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
            Token::LCurly => "{",
            Token::RCurly => "}",
            Token::Semicolon => ";",
            Token::HandlerKw => "handler",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::TxKw => "tx",
            Token::QueryKw => "query",
            Token::PureKw => "pure",
            Token::KeyKw => "key",
            Token::Colon => ":",
            Token::Comma => ",",
            Token::EmitsKw => "emits",
            Token::LSquare => "[",
            Token::RSquare => "]",
            Token::StructKw => "struct",
            Token::EventKw => "event",
            Token::MapKw => "map",
            Token::RArrow => "=>",
            Token::ClientKw => "client",
            Token::ImplKw => "impl",
            Token::ForKw => "for",
            Token::Dot => ".",
            Token::Eq => "=",
            Token::InKw => "in",
        }
    }
}
