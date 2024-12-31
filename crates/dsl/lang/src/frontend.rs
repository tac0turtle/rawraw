use std::io::Read;
use crate::frontend;
use crate::frontend::ast::ParsedAST;

pub mod ast;
pub mod parser;
pub mod syntax;
pub mod lexer;
pub mod diagnostic;
pub mod resolver;
// mod type_checker;
// mod checker;

pub fn compile(src: &str) -> ParsedAST {
    parser::parse(src)
}

pub fn compile_cli(filename: &str) {
    if let Some(input) = read_file(filename) {
        let ast = frontend::compile(input.as_str());
        for diag in &ast.diagnostics {
            diag.print_report(&input);
        }
        // debugging
        println!("{:#?}", ast.syntax());
    }
}

fn read_file(filename: &str) -> Option<String> {
    match std::fs::File::open(filename) {
        Ok(mut f) => {
            let mut result = String::new();
            match f.read_to_string(&mut result) {
                Ok(_) => {
                    return Some(result)
                }
                Err(_) => {
                    eprintln!("Could not read file {}", filename);
                }
            }
        }
        Err(_) => {
            eprintln!("Could not open file {}", filename);
        }
    }
    None
}