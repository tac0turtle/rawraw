use std::fmt;
use chumsky::input::{Input, Stream, ValueInput};
use chumsky::prelude::SimpleSpan;
use logos::{Logos, SpannedIter};

#[derive(Logos, Debug, PartialEq, Eq, Clone)]
pub enum Token<'a> {
    Error,
    #[regex(r#"[ \t\n\r\f\v]+"#, logos::skip)]
    Whitespace,
    #[regex(r#"//[^\n\r\f\v]*"#, logos::skip)]
    LineComment,
    // TODO block comments
    // #[token(r#"\/\*(\*(?!\/)|[^*])*\*\/"#, |lex| lex.slice())]
    // BlockComment(&'a str),
    #[token("interface")]
    Interface,
    #[token("handler")]
    Handler,
    #[token("struct")]
    Struct,
    #[token("event")]
    Event,
    #[token("tx")]
    Tx,
    #[token("query")]
    Query,
    #[token("key")]
    Key,
    #[token("map")]
    Map,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice())]
    Ident(&'a str),
    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice())]
    String(&'a str),
    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice())]
    Numeric(&'a str),
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,
    #[token("=")]
    Equal,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("&")]
    And,
    #[token("|")]
    Or,
    #[token("^")]
    Xor,
    #[token("~")]
    Tilde,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    NotEqual,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("&&")]
    AndAnd,
    #[token("||")]
    OrOr,
    #[token("!")]
    Not,
    #[token("?")]
    Question,
}

// #[derive(Default, Debug, Clone, PartialEq)]
// pub enum LexicalError {
//     #[default]
//     InvalidToken
// }

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Whitespace => write!(f, "<whitespace>"),
            Token::LineComment => write!(f, "<line_comment>"),
            // Token::BlockComment(s) => write!(f, "{}", s),
            Token::Interface => write!(f, "interface"),
            Token::Struct => write!(f, "struct"),
            Token::Handler => write!(f, "handler"),
            Token::Event => write!(f, "event"),
            Token::Tx => write!(f, "tx"),
            Token::Query => write!(f, "query"),
            Token::Key => write!(f, "key"),
            Token::Map => write!(f, "map"),
            Token::Ident(s) => write!(f, "{}", s),
            Token::String(s) => write!(f, "{}", s),
            Token::Numeric(s) => write!(f, "{}", s),
            Token::Semicolon => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Dot => write!(f, "."),
            Token::Equal => write!(f, "="),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::And => write!(f, "&"),
            Token::Or => write!(f, "|"),
            Token::Xor => write!(f, "^"),
            Token::Tilde => write!(f, "~"),
            Token::Less => write!(f, "<"),
            Token::Greater => write!(f, ">"),
            Token::EqualEqual => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::LessEqual => write!(f, "<="),
            Token::GreaterEqual => write!(f, ">="),
            Token::AndAnd => write!(f, "&&"),
            Token::OrOr => write!(f, "||"),
            Token::Not => write!(f, "!"),
            Token::Question => write!(f, "?"),
            Token::Error => write!(f, "<error>"),
        }
    }
}

pub fn lex(input: &str) -> impl ValueInput<Token=Token, Span=SimpleSpan> {
    // this code was copied from the example in https://github.com/zesterer/chumsky/blob/f0a86946172023b1d48f8f85bd91e433c9dbd746/examples/logos.rs#L130
    let token_iter = Token::lexer(&input)
        .spanned()
        // Convert logos errors into tokens. We want parsing to be recoverable and not fail at the lexing stage, so
        // we have a dedicated `Token::Error` variant that represents a token error that was previously encountered
        .map(|(tok, span)| match tok {
            // Turn the `Range<usize>` spans logos gives us into chumsky's `SimpleSpan` via `Into`, because it's easier
            // to work with
            Ok(tok) => (tok, span.into()),
            Err(_) => (Token::Error, span.into()),
        });
    Stream::from_iter(token_iter)
        .spanned(SimpleSpan::from(input.len()..input.len()))
}
