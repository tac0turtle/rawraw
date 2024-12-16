#![allow(missing_docs)]

mod ast;
mod lex;

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::input::{Stream, ValueInput};
use chumsky::prelude::*;
use logos::Logos;
use std::io::Read;
// use lalrpop_util::lalrpop_mod;
use crate::ast::File;
use crate::lex::Token;
use crate::lex::Token::Interface;
// lalrpop_mod!(grammar);

// fn lex(input: &str) -> Vec<lex::Token> {
//     lex::Token::lexer(input).collect()
// }
//
fn parse(input: &str) -> Result<File, anyhow::Error> {
    Ok(File { items: vec![] })
}

//
// fn rust_codegen(ast: File) -> anyhow::Result<syn::File> {
//     Ok(syn::File{
//         shebang: None,
//         attrs: vec![],
//         items: vec![],
//     })
// }
//

fn read_example() -> anyhow::Result<String> {
    let mut input = String::new();
    // read examples/ex1.ixc
    std::fs::File::open("crates/dsl/dsl/examples/ex1.ixc")?.read_to_string(&mut input)?;
    Ok(input)
}

fn parser<'a, I>() -> impl Parser<'a, I, File, extra::Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    just(Token::Interface).map(|_| File { items: vec![] })
}

// fn lex(input: &str) -> Stream<impl Iterator<Item = Token<'_>>> {
//     // this code was copied from https://github.com/zesterer/chumsky/blob/f0a86946172023b1d48f8f85bd91e433c9dbd746/examples/logos.rs#L130
//     let token_iter = Token::lexer(input)
//         .spanned()
//         // Convert logos errors into tokens. We want parsing to be recoverable and not fail at the lexing stage, so
//         // we have a dedicated `Token::Error` variant that represents a token error that was previously encountered
//         .map(|(tok, span)| match tok {
//             // Turn the `Range<usize>` spans logos gives us into chumsky's `SimpleSpan` via `Into`, because it's easier
//             // to work with
//             Ok(tok) => (tok, span.into()),
//             Err(_) => (Token::Error, span.into()),
//         });
//     // Turn the token iterator into a stream that chumsky can use for things like backtracking
//     Stream::from_iter(token_iter)
//         // // Tell chumsky to split the (Token, SimpleSpan) stream into its parts so that it can handle the spans for us
//         // // This involves giving chumsky an 'end of input' span: we just use a zero-width span at the end of the string
//         // .map((0..input.len()).into(), |(t, s): (_, _)| (t, s))
// }

fn compile() -> anyhow::Result<()> {
    let input = read_example()?;
    let tokens = Token::lexer(&input)
        .spanned()
        // Convert logos errors into tokens. We want parsing to be recoverable and not fail at the lexing stage, so
        // we have a dedicated `Token::Error` variant that represents a token error that was previously encountered
        .map(|(tok, span)| match tok {
            // Turn the `Range<usize>` spans logos gives us into chumsky's `SimpleSpan` via `Into`, because it's easier
            // to work with
            Ok(tok) => (tok, span.into()),
            Err(_) => (Token::Error, span.into()),
        }).collect::<Vec<_>>();
    let tokens = tokens.spanned(SimpleSpan::from(input.len()..input.len()));
    // Turn the token iterator into a stream that chumsky can use for things like backtracking
    // let token_stream = Stream::from_iter(token_iter).spanned(spans);
    //     // // // Tell chumsky to split the (Token, SimpleSpan) stream into its parts so that it can handle the spans for us
    //     // // // This involves giving chumsky an 'end of input' span: we just use a zero-width span at the end of the string
    //     // .map((0..input.len()).into(), |(t, s): (_, _)| (t, s));

    match parser().parse(tokens).into_result() {
        // If parsing was successful, attempt to evaluate the s-expression
        Ok(file) => println!("{:?}", file),
        // If parsing was unsuccessful, generate a nice user-friendly diagnostic with ariadne. You could also use
        // codespan, or whatever other diagnostic library you care about. You could even just display-print the errors
        // with Rust's built-in `Display` trait, but it's a little crude
        Err(errs) => {
            // for err in errs {
            //     Report::build(ReportKind::Error, err.span())
            //         .with_code(3)
            //         .with_message(err.to_string())
            //         .finish()
            //         .eprint(Source::from(&input))
            //         .unwrap();
            // }
            println!("{:?}", errs)
        }
    }
    // let syn_ast = rust_codegen(ast)?;
    // println!("{}", prettyplease::unparse(&syn_ast));
    Ok(())
}

fn main() {
    compile().unwrap();
}
