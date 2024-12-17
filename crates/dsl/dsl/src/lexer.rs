use logos::Logos;
use rowan::{GreenToken, NodeOrToken};

mod lex_tokens;

pub use lex_tokens::Token;

impl <'a> From<Token<'a>> for GreenToken {
    fn from(value: Token<'a>) -> Self {
        GreenToken::new(value.kind().into(), value.text())
    }
}

pub fn lex(input: &str) -> impl Iterator<Item=Token> {
    Token::lexer(&input).spanned().map(|(res, span)| {
        match res {
            Ok(token) => token,
            Err(err) => Token::Error(&input[span.start..span.end]),
        }
    })
}
