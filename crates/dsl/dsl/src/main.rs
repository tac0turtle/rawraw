#![allow(missing_docs)]

mod lex;
mod ast;

use logos::Logos;
use std::io::Read;
use std::ops::Range;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar);

struct File {
    items: Vec<Item>,
}

enum Item {
    Interface(ItemInterface),
    Handler(Handler),
    Impl(Impl),
}

struct ItemInterface {
    name: String,
    items: Vec<InterfaceItem>,
}

struct Ident {
    value: String,
    span: Span,
}

struct Span(Range<usize>);

enum InterfaceItem {
    Fn(InterfaceItemFn),
    // Event(Struct),
    // Struct(Struct),
    // Enum(Enum),
}

struct InterfaceItemFn {
    name: String,
    fn_type: FnType,
    visibility: Visibility,
    fn_args: Vec<FnArg>,
}

struct FnArg {
    name: String,
    ty: String,
    key: bool,
}

enum FnType {
    Tx,
    Query,
    Pure,
}

enum Visibility {
    Public,
    Private,
}

struct Handler {}

struct Impl {}

enum HandlerItem {
    Collection(),
    Client(),
}

struct Block {
    stmts: Vec<Stmt>,
}

enum Stmt {
    Local(Local),
    Expr(Expr),
}

struct Local {
    name: String,
    ty: String,
}

enum Expr {
    Call(ExprCall),
}

struct ExprCall {
    name: String,
}

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

fn compile() -> anyhow::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    parse(&input)?;
    // for (token, span) in lex::Token::lexer(&input).spanned() {
    //     println!("{:?} {:?}", token, span);
    // }
    // let ast = parse(&mut input.as_str())
    //     .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;
    // let syn_ast = rust_codegen(ast)?;
    // println!("{}", prettyplease::unparse(&syn_ast));
    Ok(())
}

fn main() {
    compile().unwrap();
}
