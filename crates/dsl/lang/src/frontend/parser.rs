use crate::frontend::ast::ParsedAST;
use crate::db::FileSource;
use crate::frontend::lexer;
use salsa::{Accumulator, Database};

mod file;
mod state;
mod type_;
mod collections;
mod expr;
mod block;
mod interface;
mod object;
mod impl_;
mod fn_;
mod struct_;

#[salsa::tracked]
pub fn parse(db: &dyn Database, src: FileSource) -> ParsedAST<'_> {
    let tokens = lexer::lex(src.text(db));
    let mut parser = state::Parser::new(src.text(db), tokens.collect());
    file::file(&mut parser);
    let root = parser.finish(Default::default(), db);
    ParsedAST::new(db, root)
}
