use logos::Span;
use rowan::GreenNodeBuilder;
use crate::lexer::Token;
use crate::syntax::{SyntaxKind, SyntaxNode};

mod parse;
mod state;

pub fn parse<'source, I: Iterator<Item=(Token, Span)>>(src: &str, tokens: I) -> SyntaxNode {
    let mut parser = state::Parser::new(src, tokens.collect());
    parse::file(&mut parser);
    parser.finish(Default::default())
}