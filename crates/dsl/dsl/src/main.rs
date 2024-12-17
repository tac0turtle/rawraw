#![allow(missing_docs)]

mod ast;
mod codegen;
mod parser;
mod syntax;
mod lexer;

use crate::lexer::lex;
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

fn compile() -> anyhow::Result<()> {
    let input = read_example()?;
    let green_tokens = lex(&input).map(|token| {
        NodeOrToken::Token(token.into())
    }).collect::<Vec<_>>();
    let green_node = GreenNode::new(SyntaxKind::ROOT.into(), green_tokens);
    let root = SyntaxNode::new_root(green_node);
    println!("{:#?}", root);

    Ok(())
}

fn main() {
    compile().unwrap();
}
