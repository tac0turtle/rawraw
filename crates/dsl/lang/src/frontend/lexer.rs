use std::ops::Range;
use logos::Logos;
use rowan::{GreenToken, NodeOrToken};

mod lex_tokens;

pub use lex_tokens::Token;

pub fn lex(input: &str) -> impl Iterator<Item=(Token, Range<usize>)> + use<'_> {
    Token::lexer(&input).spanned().map(|(res, span)| {
        match res {
            Ok(token) => (token, span),
            Err(_) => (Token::Error, span),
        }
    })
}
