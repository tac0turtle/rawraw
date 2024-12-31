use crate::frontend::ast::ParsedAST;
use crate::frontend::lexer;

mod file;

mod block;
mod collections;
mod expr;
mod fn_;
mod impl_;
mod interface;
mod name;
mod object;
mod parser;
mod struct_;
mod test;
mod type_;

#[comemo::memoize]
pub fn parse(src: &str) -> ParsedAST {
    let tokens = lexer::lex(src).collect();
    let mut parser = parser::Parser::new(src, tokens);
    file::file(&mut parser);
    let mut builder = Default::default();
    let diags = parser.finish(&mut builder);
    let root = builder.finish();
    ParsedAST {
        root,
        diagnostics: diags,
    }
}
