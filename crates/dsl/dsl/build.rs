use heck::{ToShoutySnakeCase, ToUpperCamelCase};
use prettyplease::unparse;
use quote::{format_ident, quote};
use std::env;
use std::ops::Index;
use std::path::Path;
use std::str::FromStr;
use syn::parse::Parser;
use syn::{parse_quote, parse_str};
use ungrammar::Grammar;

fn main() {
    let grammar = read_ungrammar();
    generate_syntax_kinds(&grammar);
}

fn generate_syntax_kinds(grammar: &Grammar) {
    let tokens = grammar.tokens();
    let tokens = tokens
        .map(|token| {
            let data = grammar.index(token);
            let name = data.name.clone();
            let name = token_name(&name);
            if name.len() == 0 {
                panic!("failed to generate token name for {data:?}");
            }
            format_ident!("{}", name.to_shouty_snake_case())
        })
        .collect::<Vec<_>>();
    let syntax_kinds_src = quote! {
        pub enum SyntaxKind {
            ROOT,
            ERROR,
            #(#tokens),*
        }
    };
    let file: syn::File = parse_str(&syntax_kinds_src.to_string()).unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();
    std::fs::write(Path::new(&out_dir).join("syntax_kind.rs"), unparse(&file)).unwrap();
}

// fn generate_tokens(grammar: &Grammar) {
//     let tokens = grammar.tokens();
//     let tokens_src = quote! {
//         pub enum Token {
//             #(#tokens),*
//         }
//     };
//     let out_dir = env::var_os("OUT_DIR").unwrap();
//     std::fs::write(Path::new(&out_dir).join("tokens.rs"), unparse(&tokens_src)).unwrap();
// }

fn token_name(name: &str) -> String {
    if name.starts_with('#') {
        return name.trim_start_matches('#').to_upper_camel_case().to_string();
    }
    match name {
        "[" => "LBrace",
        "]" => "RBrace",
        "(" => "LParen",
        ")" => "RParen",
        "{" => "LBracket",
        "}" => "RBracket",
        "," => "Comma",
        ";" => "Semicolon",
        "." => "Dot",
        ":" => "Colon",
        _ => return name.to_upper_camel_case(),
    }.into()
}

fn read_ungrammar() -> Grammar {
    let src = std::fs::read_to_string("src/ixc.ungram").unwrap();
    println!("cargo:rerun-if-changed=src/ixc.ungram");
    Grammar::from_str(&src).unwrap()
}
