use heck::{ToShoutySnakeCase, ToUpperCamelCase};
use prettyplease::unparse;
use quote::{format_ident, quote};
use std::ops::Index;
use std::path::Path;
use std::str::FromStr;
use syn::parse::Parser;
use syn::{parse_quote, LitStr};
use ungrammar::Grammar;

fn main() {
    let grammar = read_ungrammar();
    generate_syntax_kinds(&grammar).unwrap();
    generate_lex_tokens(&grammar).unwrap();
}

fn generate_syntax_kinds(grammar: &Grammar) -> anyhow::Result<()> {
    let tokens = grammar
        .tokens()
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
    println!("cargo:warning={tokens:?}");
    let nodes = grammar
        .iter()
        .map(|node| {
            let data = grammar.index(node);
            format_ident!("{}", data.name.to_shouty_snake_case())
        })
        .collect::<Vec<_>>();
    let mut idents = tokens.clone();
    idents.extend(nodes.clone());
    let file: syn::File = parse_quote! {
        #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, num_enum::FromPrimitive, num_enum::IntoPrimitive)]
        #[repr(u16)]
        pub enum SyntaxKind {
            ROOT,
            ERROR,
            #[num_enum(catch_all)]
            UNKNOWN(u16),
            WHITESPACE,
            COMMENT,
            #(#idents),*
        }
    };
    write_file(&file, "src/syntax/syntax_kind.rs")
}

fn generate_lex_tokens(grammar: &Grammar) -> anyhow::Result<()> {
    let tokens = grammar
        .tokens()
        .map(|token| {
            let data = grammar.index(token);
            let raw_name = data.name.clone();
            let name = token_name(&raw_name);
            let case_name = format_ident!("{}", name.to_upper_camel_case());
            let syntax_kind = format_ident!("{}", name.to_shouty_snake_case());
            let regex = token_regex(&raw_name);
            let token_lit = LitStr::new(&raw_name, proc_macro2::Span::call_site());
            let enum_case = if let Some(regex) = regex.clone() {
                LitStr::new(&regex, proc_macro2::Span::call_site());
                quote! {
                    #[regex(#regex, |lex| lex.slice())]
                    #case_name(&'a str)
                }
            } else {
                quote! {
                    #[token(#token_lit)]
                    #case_name
                }
            };
            let to_syntax_kind = if regex.is_some() {
                quote! { LexicalToken::#case_name(_) => SyntaxKind::#syntax_kind }
            } else {
                quote! { LexicalToken::#case_name => SyntaxKind::#syntax_kind   }
            };
            let to_str = if regex.is_some() {
                quote! { LexicalToken::#case_name(x) => x }
            } else {
                quote! { LexicalToken::#case_name => #token_lit }
            };
            (enum_case, to_syntax_kind, to_str)
        })
        .collect::<Vec<_>>();
    let enum_cases = tokens
        .clone()
        .into_iter()
        .map(|(enum_case, _, _)| enum_case);
    let to_syntax_kind = tokens
        .clone()
        .into_iter()
        .map(|(_, to_syntax_kind, _)| to_syntax_kind);
    let to_str = tokens.into_iter().map(|(_, _, display)| display);
    let file = parse_quote! {
        use crate::syntax::SyntaxKind;
        use logos::Logos;

        #[derive(Logos, Debug, PartialEq, Eq, Clone)]
        pub enum LexicalToken<'a> {
            Error(&'a str),
            #[regex(r#"[ \t\n\r\f\v]+"#)]
            Whitespace(&'a str),
            #[regex(r#"//[^\n\r\f\v]*"#)]
            Comment(&'a str),
            #(#enum_cases),*
        }

        impl <'a> LexicalToken<'a> {
            pub fn kind(&'a self) -> SyntaxKind {
                match self {
                    LexicalToken::Error(_) => SyntaxKind::ERROR,
                    LexicalToken::Whitespace(_) => SyntaxKind::WHITESPACE,
                    LexicalToken::Comment(_) => SyntaxKind::COMMENT,
                    #(#to_syntax_kind),*
                }
            }

            pub fn text(&'a self) -> &'a str {
                match self {
                    LexicalToken::Error(x) => x,
                    LexicalToken::Whitespace(x) => x,
                    LexicalToken::Comment(x) => x,
                    #(#to_str),*
                }
            }
        }
    };
    write_file(&file, "src/lexer/lex_tokens.rs")
}

fn token_name(name: &str) -> String {
    if name.starts_with('#') {
        return name
            .trim_start_matches('#')
            .to_upper_camel_case()
            .to_string();
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
        _ => return format!("{}_KW", name.to_upper_camel_case()),
    }
    .into()
}

fn token_regex(name: &str) -> Option<String> {
    if !name.starts_with('#') {
        return None;
    }
    let name = name.trim_start_matches('#');
    Some(
        match name {
            "ident" => r"[a-zA-Z_][a-zA-Z0-9_]*",
            _ => panic!("unknown regex for {}", name),
        }
        .into(),
    )
}

fn read_ungrammar() -> Grammar {
    let src = std::fs::read_to_string("src/ixc.ungram").unwrap();
    println!("cargo:rerun-if-changed=src/ixc.ungram");
    Grammar::from_str(&src).unwrap()
}

fn write_file(file: &syn::File, file_name: &str) -> anyhow::Result<()> {
    let src = unparse(file);
    const WARNING: &'static str = "//! GENERATED CODE -- DO NOT EDIT!\n\n";
    let src = format!("{}{}", WARNING, src);
    std::fs::write(Path::new(file_name), src)?;
    Ok(())
}
