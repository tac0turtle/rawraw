use crate::db::FileSource;
use crate::frontend::ast::ParsedAST;
use crate::frontend::diagnostic::{Diagnostic, Severity};
use crate::frontend::lexer;
use crate::frontend::syntax::SyntaxKind;
use rowan::{GreenNode, TextRange};
use salsa::{Accumulator, Database};
use std::panic;

mod file;

mod block;
mod collections;
mod expr;
mod fn_;
mod impl_;
mod interface;
mod object;
mod parser;
mod struct_;
mod test;
mod type_;
mod name;

#[salsa::tracked]
pub fn parse(db: &dyn Database, src: FileSource) -> ParsedAST<'_> {
    let tokens = lexer::lex(src.text(db)).collect();
    let text = src.text(db);
    let mut parser = parser::Parser::new(text, tokens);
    file::file(&mut parser);
    let mut builder = Default::default();
    let diags = parser.finish(&mut builder);
    let root = builder.finish();
    for diag in diags {
        diag.accumulate(db);
    }
    ParsedAST::new(db, root)
}
