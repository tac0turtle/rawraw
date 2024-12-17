#![allow(missing_docs)]

mod ast;
mod codegen;
mod lex;
mod parser;
mod syntax;
mod parser2;
mod lexer;

use crate::ast::File;
use crate::parser::parser;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::input::{Stream, ValueInput};
use chumsky::prelude::*;
use logos::Logos;
use std::io::Read;
use rowan::{GreenNode, GreenToken, NodeOrToken};
use crate::lexer::{lex, LexicalToken};
use crate::syntax::{SyntaxKind, SyntaxNode};

fn parse(input: &str) -> Result<File, anyhow::Error> {
    Ok(File { items: vec![] })
}

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
    // let tokens = lex(&input);
    // match parser().parse(tokens).into_result() {
    //     // If parsing was successful, attempt to evaluate the s-expression
    //     Ok(file) => println!("{:?}", file),
    //     // If parsing was unsuccessful, generate a nice user-friendly diagnostic with ariadne. You could also use
    //     // codespan, or whatever other diagnostic library you care about. You could even just display-print the errors
    //     // with Rust's built-in `Display` trait, but it's a little crude
    //     Err(errs) => {
    //         for err in errs {
    //             Report::build(ReportKind::Error, err.span().into_range())
    //                 .with_message(err.to_string())
    //                 .with_label(Label::new(err.span().into_range())
    //                     .with_message(err.reason().to_string())
    //                     .with_color(Color::Red))
    //                 .finish()
    //                 .eprint(Source::from(&input))
    //                 .unwrap();
    //         }
    //     }
    // }
    // let syn_ast = rust_codegen(ast)?;
    // println!("{}", prettyplease::unparse(&syn_ast));

    Ok(())
}

fn main() {
    compile().unwrap();
}
