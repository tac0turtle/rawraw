//! GENERATED CODE -- DO NOT EDIT!

use crate::frontend::syntax::SyntaxKind;
use logos::Logos;
#[derive(Logos, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    Error,
    Eof,
    #[regex(r#"[ \t\n\r\f\v]+"#)]
    Whitespace,
    #[regex(r#"//[^\n\r\f\v]*"#)]
    LineComment,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,
    #[token("interface")]
    InterfaceKw,
    #[token("{")]
    LCurly,
    #[token("}")]
    RCurly,
    #[token(";")]
    Semicolon,
    #[token("object")]
    ObjectKw,
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
    #[token("mut")]
    MutKw,
    #[token("ref")]
    RefKw,
    #[token("transfer")]
    TransferKw,
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
    #[token("account_scoped")]
    AccountScopedKw,
    #[token("map")]
    MapKw,
    #[token("var")]
    VarKw,
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
    #[token("test")]
    TestKw,
}
impl Token {
    pub fn kind(&self) -> SyntaxKind {
        match self {
            Token::Error => SyntaxKind::ERROR,
            Token::Eof => SyntaxKind::EOF,
            Token::Whitespace => SyntaxKind::WHITESPACE,
            Token::LineComment => SyntaxKind::LINE_COMMENT,
            Token::Ident => SyntaxKind::IDENT,
            Token::InterfaceKw => SyntaxKind::INTERFACE_KW,
            Token::LCurly => SyntaxKind::L_CURLY,
            Token::RCurly => SyntaxKind::R_CURLY,
            Token::Semicolon => SyntaxKind::SEMICOLON,
            Token::ObjectKw => SyntaxKind::OBJECT_KW,
            Token::LParen => SyntaxKind::L_PAREN,
            Token::RParen => SyntaxKind::R_PAREN,
            Token::TxKw => SyntaxKind::TX_KW,
            Token::QueryKw => SyntaxKind::QUERY_KW,
            Token::PureKw => SyntaxKind::PURE_KW,
            Token::KeyKw => SyntaxKind::KEY_KW,
            Token::Colon => SyntaxKind::COLON,
            Token::Comma => SyntaxKind::COMMA,
            Token::MutKw => SyntaxKind::MUT_KW,
            Token::RefKw => SyntaxKind::REF_KW,
            Token::TransferKw => SyntaxKind::TRANSFER_KW,
            Token::EmitsKw => SyntaxKind::EMITS_KW,
            Token::LSquare => SyntaxKind::L_SQUARE,
            Token::RSquare => SyntaxKind::R_SQUARE,
            Token::StructKw => SyntaxKind::STRUCT_KW,
            Token::EventKw => SyntaxKind::EVENT_KW,
            Token::AccountScopedKw => SyntaxKind::ACCOUNT_SCOPED_KW,
            Token::MapKw => SyntaxKind::MAP_KW,
            Token::VarKw => SyntaxKind::VAR_KW,
            Token::RArrow => SyntaxKind::R_ARROW,
            Token::ClientKw => SyntaxKind::CLIENT_KW,
            Token::ImplKw => SyntaxKind::IMPL_KW,
            Token::ForKw => SyntaxKind::FOR_KW,
            Token::Dot => SyntaxKind::DOT,
            Token::Eq => SyntaxKind::EQ,
            Token::InKw => SyntaxKind::IN_KW,
            Token::TestKw => SyntaxKind::TEST_KW,
        }
    }
}
