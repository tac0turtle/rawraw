use crate::ast::ParsedAST;
use crate::db::FileSource;
use crate::lexer;
use crate::lexer::Token;
use crate::syntax::{SyntaxKind, SyntaxNode};
use logos::Span;
use rowan::{GreenNode, GreenNodeBuilder};
use salsa::{Accumulator, Database};

mod file;
mod state;

#[salsa::tracked]
pub fn parse(db: &dyn Database, src: FileSource) -> ParsedAST<'_> {
    let tokens = lexer::lex(src.text(db));
    let mut parser = state::Parser::new(src.text(db), tokens.collect());
    file::file(&mut parser);
    let root = parser.finish(Default::default(), db);
    ParsedAST::new(db, root)
}
