use logos::Span;
use rowan::{GreenNode, GreenNodeBuilder};
use crate::lexer::Token;
use crate::syntax::{SyntaxKind, SyntaxNode};

mod parse;
mod state;

pub fn parse<'source, I: Iterator<Item=(Token, Span)>>(src: &str, tokens: I) -> GreenNode {
    let mut parser = state::Parser::new(src, tokens.collect());
    parse::file(&mut parser);
    parser.finish(Default::default())
}