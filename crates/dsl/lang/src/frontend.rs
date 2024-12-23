use std::io::Read;
use salsa::Database;
use crate::db::FileSource;
use crate::frontend;
use crate::frontend::ast::ParsedAST;
use crate::frontend::diagnostic::Diagnostic;

pub mod ast;
pub mod parser;
pub mod syntax;
pub mod lexer;
pub mod diagnostic;
pub mod resolve;
// mod type_checker;
// mod checker;

#[salsa::tracked]
pub fn compile(db: &dyn Database, src: FileSource) -> ParsedAST<'_> {
    parser::parse(&*db, src)
}

pub fn compile_cli(filename: &str) {
    if let Some(input) = read_file(filename) {
        let db = crate::db::Db::default();
        let src = FileSource::new(&db, input.clone());
        let ast = frontend::compile(&db, src);
        let diags = compile::accumulated::<Diagnostic>(&db, src);
        for diag in diags {
            diag.print_report(&input);
        }
        // debugging
        println!("{:#?}", ast.syntax(&db));
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