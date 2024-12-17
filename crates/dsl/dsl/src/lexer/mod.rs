use logos::Logos;
use rowan::{GreenToken, NodeOrToken};

mod lex_tokens;

pub use lex_tokens::LexicalToken;

impl <'a> From<LexicalToken<'a>> for GreenToken {
    fn from(value: LexicalToken<'a>) -> Self {
        GreenToken::new(value.kind().into(), value.text())
    }
}

pub fn lex(input: &str) -> impl Iterator<Item=LexicalToken> {
    LexicalToken::lexer(&input).spanned().map(|(res, span)| {
        match res {
            Ok(token) => token,
            Err(err) => LexicalToken::Error(&input[span.start..span.end]),
        }
    })
}