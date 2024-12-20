use std::panic;
use rowan::{GreenNode, TextRange};
use crate::frontend::ast::ParsedAST;
use crate::db::FileSource;
use crate::frontend::lexer;
use salsa::{Accumulator, Database};
use crate::frontend::diagnostic::{Diagnostic, Severity};
use crate::frontend::syntax::SyntaxKind;

mod file;


mod parser;
mod type_;
mod collections;
mod expr;
mod block;
mod interface;
mod object;
mod impl_;
mod fn_;
mod struct_;
mod test;

#[salsa::tracked]
pub fn parse(db: &dyn Database, src: FileSource) -> ParsedAST<'_> {
    let tokens = lexer::lex(src.text(db)).collect();
    let text =src.text(db);
    let res =panic::catch_unwind(|| {
        let mut parser = parser::Parser::new(text, tokens);
        file::file(&mut parser);
        let mut builder = Default::default();
        let diags = parser.finish(&mut builder);
        (builder.finish(), diags)
    });
    let (root, diags) = res.unwrap_or_else(|err| {
        let diag = Diagnostic::new(format!("{err:?}"), TextRange::new(0.into(), text.len().try_into().unwrap()), Severity::Error);
        (GreenNode::new(SyntaxKind::ERROR_NODE.into(), vec![]), vec![diag])
    });
    for diag in diags {
        diag.accumulate(db);
    }
    ParsedAST::new(db, root)
}
