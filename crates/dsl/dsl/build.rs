use std::collections::HashSet;
use heck::{ToShoutySnakeCase, ToUpperCamelCase};
use prettyplease::unparse;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::ops::Index;
use std::path::Path;
use std::str::FromStr;
use anyhow::bail;
use syn::parse::Parser;
use syn::{parse_quote, LitStr};
use ungrammar::{Grammar, Node, Rule, Token};

fn main() {
    let grammar = read_ungrammar();
    generate_syntax_kinds(&grammar).unwrap();
    generate_lex_tokens(&grammar).unwrap();
    generate_ast(&grammar).unwrap();
}

fn generate_syntax_kinds(grammar: &Grammar) -> anyhow::Result<()> {
    let tokens = grammar
        .tokens()
        .map(|token| {
            let data = grammar.index(token);
            let name = data.name.clone();
            token_syntax_name(&name)
        })
        .collect::<Vec<_>>();
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
            EOF,
            ERROR, // this is for errors represented as a single token
            ERROR_NODE, // this is for errors represented as a node of possible multiple tokens
            #[num_enum(catch_all)]
            UNKNOWN(u16),
            WHITESPACE,
            LINE_COMMENT,
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
                    #[regex(#regex)]
                    #case_name
                }
            } else {
                quote! {
                    #[token(#token_lit)]
                    #case_name
                }
            };
            let to_syntax_kind = quote! { Token::#case_name => SyntaxKind::#syntax_kind   };
            (enum_case, to_syntax_kind)
        })
        .collect::<Vec<_>>();
    let enum_cases = tokens
        .clone()
        .into_iter()
        .map(|(enum_case, _)| enum_case);
    let to_syntax_kind = tokens
        .clone()
        .into_iter()
        .map(|(_, to_syntax_kind)| to_syntax_kind);
    let file = parse_quote! {
        use crate::syntax::SyntaxKind;
        use logos::Logos;

        #[derive(Logos, Debug, PartialEq, Eq, Clone, Copy)]
        pub enum Token {
            Error,
            Eof,
            #[regex(r#"[ \t\n\r\f\v]+"#)]
            Whitespace,
            #[regex(r#"//[^\n\r\f\v]*"#)]
            LineComment,
            #(#enum_cases),*
        }

        impl Token {
            pub fn kind(&self) -> SyntaxKind {
                match self {
                    Token::Error => SyntaxKind::ERROR,
                    Token::Eof => SyntaxKind::EOF,
                    Token::Whitespace => SyntaxKind::WHITESPACE,
                    Token::LineComment => SyntaxKind::LINE_COMMENT,
                    #(#to_syntax_kind),*
                }
            }
        }
    };
    write_file(&file, "src/lexer/lex_tokens.rs")
}

enum AstNodeAst<'a> {
    Struct(Vec<(String, AstStructField<'a>)>),
    NodeEnum(Vec<&'a Node>),
    TokenEnum(Vec<&'a Token>),
}

enum AstStructField<'a> {
    Node(&'a Node),
    Token(&'a Token),
    NodeChildren(&'a Node),
}

fn ast_node_ast<'a>(name: &str, rule: &'a Rule) -> anyhow::Result<AstNodeAst<'a>> {
    Ok(match rule {
        Rule::Labeled { label, rule } => {
            AstNodeAst::Struct(vec![(label.clone(), ast_node_field(rule)?)])
        }
        Rule::Seq(seq) => {
            let mut fields = vec![];
            for rule in seq {
                match rule {
                    Rule::Labeled { label, rule } => {
                        fields.push((label.clone(), ast_node_field(rule)?));
                    }
                    Rule::Node(n) => bail!("node rule must be labeled, got {:?}, in rule: {name}", n),
                    Rule::Rep(r) => bail!("rep rule must be labeled, got {:?}, in rule: {name}", r),
                    Rule::Token(_) => {} // skip unlabeled tokens
                    Rule::Opt(r) => bail!("opt rule must be labeled, got {:?}, in rule: {name}", r),
                    Rule::Seq(s) => bail!("nested seq rule not supported {:?}, in rule: {name}", s),
                    Rule::Alt(a) => bail!("nested alt rule not supported {:?}, in rule: {name}", a),
                }
            }
            AstNodeAst::Struct(fields)
        }
        Rule::Alt(alt) => {
            let mut res = AstNodeAst::Struct(vec![]);
            for rule in alt {
                match rule {
                    Rule::Node(n) => {
                        if let AstNodeAst::Struct(_) = res {
                            res = AstNodeAst::NodeEnum(vec![n]);
                        } else if let AstNodeAst::NodeEnum(mut cases) = res {
                            cases.push(n);
                            res = AstNodeAst::NodeEnum(cases);
                        } else {
                            bail!("rule: {name} alt rule must have all nodes or all tokens, got {:?}", rule);
                        }
                    }
                    Rule::Token(t) => {
                        if let AstNodeAst::Struct(_) = res {
                            res = AstNodeAst::TokenEnum(vec![t]);
                        } else if let AstNodeAst::TokenEnum(mut cases) = res {
                            cases.push(t);
                            res = AstNodeAst::TokenEnum(cases);
                        } else {
                            bail!("alt rule must have all nodes or all tokens, got {:?}", rule);
                        }
                    }
                    _ => bail!("alt rule child must be a node or token, got {:?}", rule),
                }
            }
            res
        },
        Rule::Token(t) => AstNodeAst::TokenEnum(vec![t]),
        Rule::Node(n) => AstNodeAst::NodeEnum(vec![n]),
        _ => bail!("rule {name}: top-level rule must be a labeled, seq, or alt, try labelling it, got {:?}", rule),
    })
}

fn ast_node_field(rule: &Rule) -> anyhow::Result<AstStructField> {
    Ok(match rule {
        Rule::Node(n) => AstStructField::Node(n),
        Rule::Token(t) => AstStructField::Token(t),
        Rule::Opt(r) => match r.as_ref() {
            Rule::Node(n) => AstStructField::Node(n),
            Rule::Token(t) => AstStructField::Token(t),
            _ => anyhow::bail!("opt rule must be a node or token"),
        },
        Rule::Rep(r) => match r.as_ref() {
            Rule::Node(n) => AstStructField::NodeChildren(n),
            _ => anyhow::bail!("rep rule must be a node"),
        },
        r => anyhow::bail!("labeled rule must be a node or token, got {:?}", r),
    })
}

fn ast_node_code(
    grammar: &Grammar,
    type_name: &syn::Ident,
    syntax_name: &syn::Ident,
    ast: &AstNodeAst,
) -> anyhow::Result<TokenStream> {
    match ast {
        AstNodeAst::Struct(ast) => ast_node_struct(grammar, type_name, syntax_name, ast),
        AstNodeAst::NodeEnum(ast) => ast_node_node_enum(grammar, type_name, ast),
        AstNodeAst::TokenEnum(ast) => ast_node_token_enum(grammar, type_name, ast),
    }
}

fn ast_node_struct(
    grammar: &Grammar,
    struct_name: &syn::Ident,
    syntax_name: &syn::Ident,
    ast: &[(String, AstStructField)],
) -> anyhow::Result<TokenStream> {
    let mut field_getters: Vec<TokenStream> = vec![];
    let mut syntax_kinds: HashSet<Ident> = HashSet::new();
    let mut check_syntax_kind = |syntax_kind: &Ident| {
        if syntax_kinds.contains(syntax_kind) {
            bail!("duplicate syntax kind {syntax_kind} within the same struct node {struct_name}, this will make the generated code ambiguous, please wrap one of the children in a new node");
        }
        syntax_kinds.insert(syntax_kind.clone());
        Ok(())
    };
    for (name, field) in ast {
        let field_getter = to_valid_ident(&name);
        match field {
            AstStructField::Node(n) => {
                let node_data = grammar.index(*n.clone());
                let node_struct = format_ident!("{}", node_data.name.to_upper_camel_case());
                check_syntax_kind(&format_ident!("{}", node_data.name.to_shouty_snake_case()))?;
                field_getters.push(quote! {
                    #[inline]
                    pub fn #field_getter(&self) -> Option<#node_struct> { rowan::ast::support::child(&self.syntax) }
                });
            }
            AstStructField::Token(t) => {
                let token_data = grammar.index(*t.clone());
                let syntax_kind = token_syntax_name(&token_data.name);
                check_syntax_kind(&syntax_kind)?;
                field_getters.push(quote! {
                    #[inline]
                    pub fn #field_getter(&self) -> Option<SyntaxToken> { rowan::ast::support::token(&self.syntax, SyntaxKind::#syntax_kind) }
                });
            }
            AstStructField::NodeChildren(n) => {
                let node_data = grammar.index(*n.clone());
                let node_struct = format_ident!("{}", node_data.name.to_upper_camel_case());
                check_syntax_kind(&format_ident!("{}", node_data.name.to_shouty_snake_case()))?;
                field_getters.push(quote! {
                    #[inline]
                    pub fn #field_getter(&self) -> rowan::ast::AstChildren<#node_struct> { rowan::ast::support::children(&self.syntax) }
                });
            }
        }
    };
    Ok(quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct #struct_name {
            syntax: SyntaxNode,
        }
        impl rowan::ast::AstNode for #struct_name {
            type Language = IXCLanguage;
            fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::#syntax_name }
            fn cast(syntax: SyntaxNode) -> Option<Self> {
                if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
            }
            fn syntax(&self) -> &SyntaxNode { &self.syntax }
        }
        impl #struct_name {
            #(#field_getters)*
        }
        impl crate::ast::AstStruct for #struct_name {
            const KIND: SyntaxKind = SyntaxKind::#syntax_name;
        }
    })
}

fn ast_node_node_enum(grammar: &Grammar, enum_name: &Ident, ast: &[&Node]) -> anyhow::Result<TokenStream> {
    Ok(quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum #enum_name {}
        impl rowan::ast::AstNode for #enum_name {
            type Language = IXCLanguage;
            fn can_cast(kind: SyntaxKind) -> bool { todo!() }
            fn cast(syntax: SyntaxNode) -> Option<Self> {
                todo!()
            }
            fn syntax(&self) -> &SyntaxNode { todo!() }
        }
    })
}

fn ast_node_token_enum(
    grammar: &Grammar,
    enum_name: &Ident,
    ast: &[&Token],
) -> anyhow::Result<TokenStream> {
    Ok(quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum #enum_name {}
        impl rowan::ast::AstNode for #enum_name {
            type Language = IXCLanguage;
            fn can_cast(kind: SyntaxKind) -> bool { todo!() }
            fn cast(syntax: SyntaxNode) -> Option<Self> {
                todo!()
            }
            fn syntax(&self) -> &SyntaxNode { todo!() }
        }
    })
}

fn generate_ast(grammar: &Grammar) -> anyhow::Result<()> {
    let mut nodes = vec![];
    for node in grammar.iter() {
        let data = grammar.index(node);
        let name = data.name.clone();
        let type_name = format_ident!("{}", name.to_upper_camel_case());
        let syntax_name = format_ident!("{}", name.to_shouty_snake_case());
        let ast_node_ast = ast_node_ast(&name, &data.rule)?;
        nodes.push(ast_node_code(
            &grammar,
            &type_name,
            &syntax_name,
            &ast_node_ast,
        )?);
    }
    let file = parse_quote! {
        use crate::syntax::{SyntaxKind, SyntaxNode, SyntaxToken, IXCLanguage};
        #(#nodes)*
    };
    write_file(&file, "src/ast/nodes.rs")
}

fn token_syntax_name(name: &str) -> syn::Ident {
    let name = token_name(name);
    if name.len() == 0 {
        panic!("failed to generate token name for {name:?}");
    }
    format_ident!("{}", name.to_shouty_snake_case())
}

fn token_name(name: &str) -> String {
    if name.starts_with('#') {
        return name
            .trim_start_matches('#')
            .to_upper_camel_case()
            .to_string();
    }
    match name {
        "[" => "LSquare",
        "]" => "RSquare",
        "(" => "LParen",
        ")" => "RParen",
        "{" => "LCurly",
        "}" => "RCurly",
        "," => "Comma",
        ";" => "Semicolon",
        "." => "Dot",
        ":" => "Colon",
        "=" => "Eq",
        "=>" => "RArrow",
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

fn to_valid_ident(name:&str) -> syn::Ident {
    syn::parse_str::<syn::Ident>(name).unwrap_or_else(|_| format_ident!("{name}_"))
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