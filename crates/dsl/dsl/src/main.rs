#![allow(missing_docs)]

mod ast;
mod codegen;
mod parser;
mod syntax;
mod lexer;
mod lsp;
mod db;
mod diagnostic;

use crate::lexer::{lex};
use crate::syntax::{SyntaxKind, SyntaxNode};
use chumsky::input::ValueInput;
use chumsky::prelude::*;
use logos::Logos;
use rowan::{GreenNode, NodeOrToken};
use std::io::Read;

fn read_example() -> anyhow::Result<String> {
    let mut input = String::new();
    // read examples/ex1.ixc
    std::fs::File::open("crates/dsl/dsl/examples/ex1.ixc")?.read_to_string(&mut input)?;
    Ok(input)
}

// fn compile() -> anyhow::Result<()> {
//     let input = read_example()?;
//     let tokens = lex(&input);
//     let root = parser::parse_tokens(&input, tokens);
//     println!("{:#?}", root);
//
//     Ok(())
// }

fn main() {
    //
    // compile().unwrap();
    lsp::main()
}
