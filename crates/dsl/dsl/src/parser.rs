use logos::Span;
use rowan::GreenNodeBuilder;
use crate::lexer::Token;
use crate::syntax::{SyntaxKind, SyntaxNode};

mod parse;
mod state;

pub fn parse<'source, I: Iterator<Item=(Token<'source>, Span)>>(tokens: I) -> SyntaxNode {
    let mut parser = state::Parser::new(tokens.collect());
    parse::parse(&mut parser);
    parser.finish(Default::default())
}