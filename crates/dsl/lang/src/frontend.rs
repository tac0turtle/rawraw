use std::io::Read;
use comemo::Tracked;
use rowan::GreenNode;
use crate::files::FileSources;
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

pub fn compile(files: Tracked<FileSources>, filename: &str) -> ParsedAST {
    let src = files.get(filename).unwrap();
    let mut ast = parser::parse(&src);
    if let Err(diagnostics) = resolver::resolve(files, filename) {
        ast.diagnostics.extend(diagnostics);
    }
    ast
}

pub fn compile_cli(filename: &str) {
    if let Some(input) = read_file(filename) {
        let ast = parser::parse(input.as_str());
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