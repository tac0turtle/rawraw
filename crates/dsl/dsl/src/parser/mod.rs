mod state;

use crate::ast;
use crate::ast::File;
use crate::lexer::Token;
use crate::parser::NodeBuilder::{Children, Token};
use crate::syntax::SyntaxKind;
use chumsky::error::Rich;
use chumsky::input::ValueInput;
use chumsky::prelude::*;
use rowan::{GreenNode, GreenNodeBuilder, NodeCache};

enum NodeBuilder<'a> {
    Node(SyntaxKind, Vec<NodeBuilder<'a>>),
    Token(Token<'a>),
    Children(Vec<NodeBuilder<'a>>),
}

fn just_token<'a, I>(
    t: Token<'a>,
) -> impl Parser<'a, I, NodeBuilder<'a>, extra::Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    just(t).map(Token)
}

fn node<'a, I>(kind: SyntaxKind, children: I) -> NodeBuilder<'a>
where
    I: IntoIterator<Item = NodeBuilder<'a>>,
    I::IntoIter: ExactSizeIterator,
{
    NodeBuilder::Node(kind, children.into_iter().collect())
}

pub fn parser<'a, I>() -> impl Parser<'a, I, NodeBuilder<'a>, extra::Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    let trivia_item = select! {
        Token::Whitespace(it) => Token(Token::Whitespace(it)),
        Token::Comment(it) => Token(Token::Comment(it)),
    };
    let trivia = trivia_item.repeated().collect::<Vec<_>>().map(Children);
    let ident = select! { Token::Ident(ident) => Token::Ident(ident) }
        .labelled("identifier")
        .map(Token);

    let type_ = ident
        .then(trivia)
        .then(
            just_token(Token::LBracket)
                .then(just_token(Token::RBracket))
                .or_not(),
        )
        .map(|((t, ident), arr)| match arr {
            Some((l, r)) => node(SyntaxKind::TYPE_ARRAY, [t, ident, l, r]),
            None => node(SyntaxKind::TYPE_IDENT, [t, ident]),
        })
        .map(|it| node(SyntaxKind::TYPE, [it]));

    let struct_field = ident
        .then(trivia)
        .then(just_token(Token::Colon))
        .then(trivia)
        .then(type_)
        .map(|((((name, a), col), b), ty)| node(SyntaxKind::STRUCT_FIELD, [name, a, col, b, ty]));

    // let struct_fields = struct_field
    //     .separated_by(just(Token::Comma))
    //     .allow_trailing()
    //     .collect::<Vec<_>>()
    //     .delimited_by(just(Token::LBrace), just(Token::RBrace));
    //
    // let struct_ = just(Token::Struct)
    //     .ignore_then(ident)
    //     .then(struct_fields.clone())
    //     .map(|(name, fields)| ast::Struct { name, fields });
    //
    // let event = just(Token::Event)
    //     .ignore_then(ident)
    //     .then(struct_fields)
    //     .map(|(name, fields)| ast::Struct { name, fields });
    //
    // let fn_arg = just(Token::Key)
    //     .or_not()
    //     .then(ident)
    //     .then_ignore(just(Token::Colon))
    //     .then(type_.clone())
    //     .map(|((key, name), ty)| ast::FnArg {
    //         name,
    //         ty,
    //         key: key.is_some(),
    //     });
    //
    // let fn_args = fn_arg
    //     .separated_by(just(Token::Comma))
    //     .allow_trailing()
    //     .collect::<Vec<_>>()
    //     .delimited_by(just(Token::LParen), just(Token::RParen));
    //
    // let fn_type = just(Token::Tx)
    //     .map(|_| ast::FnType::Tx)
    //     .or(just(Token::Query).map(|_| ast::FnType::Query));
    //
    // let fn_events = just(Token::Event).ignore_then(
    //     ident
    //         .separated_by(just(Token::Comma))
    //         .allow_trailing()
    //         .collect::<Vec<_>>()
    //         .delimited_by(just(Token::LParen), just(Token::RParen)),
    // );
    //
    // let fn_sig = fn_type
    //     .then(ident)
    //     .then(fn_args)
    //     .then(fn_events.or_not())
    //     .then(type_.clone().or_not())
    //     .then_ignore(just(Token::Semicolon))
    //     .map(
    //         |((((fn_type, name), args), events), return_type)| ast::FnSignature {
    //             name,
    //             fn_type,
    //             args,
    //             events: events.unwrap_or_default(),
    //             return_type,
    //         },
    //     );
    //
    // let interface_item = struct_
    //     .map(|it| ast::InterfaceItem::Struct(it))
    //     .or(event.map(|it| ast::InterfaceItem::Event(it)))
    //     .or(fn_sig.map(|it| ast::InterfaceItem::Fn(it)));
    //
    // let interface_items = interface_item
    //     .repeated()
    //     .collect::<Vec<_>>()
    //     .delimited_by(just(Token::LBrace), just(Token::RBrace));
    //
    // let interface = just(Token::Interface)
    //     .ignore_then(ident)
    //     .then(interface_items)
    //     .map(|(name, items)| ast::Item::Interface(ast::ItemInterface { name, items }));
    //
    // let map_field = ident
    //     .then_ignore(just(Token::Colon))
    //     .then(type_.clone())
    //     .map(|(name, ty)| ast::MapField { name, ty });
    //
    // let map_key_fields = map_field
    //     .clone()
    //     .separated_by(just(Token::Comma))
    //     .allow_trailing()
    //     .collect::<Vec<_>>()
    //     .delimited_by(just(Token::LBrace), just(Token::RBrace));
    //
    // let map_value_fields = map_field
    //     .separated_by(just(Token::Comma))
    //     .allow_trailing()
    //     .collect::<Vec<_>>();
    //
    // let map_ = just(Token::Map)
    //     .ignore_then(ident)
    //     .then(map_key_fields)
    //     .then(map_value_fields)
    //     .then_ignore(just(Token::Semicolon))
    //     .map(|((name, key_fields), value_fields)| ast::Map {
    //         name,
    //         key_fields,
    //         value_fields,
    //     });
    //
    // let handler_item = map_.map(|it| ast::HandlerItem::Map(it));
    //
    // let handler_items = handler_item
    //     .repeated()
    //     .collect::<Vec<_>>()
    //     .delimited_by(just(Token::LBrace), just(Token::RBrace));
    //
    // let handler = just(Token::Handler)
    //     .ignore_then(ident)
    //     .then(handler_items)
    //     .map(|(name, items)| ast::Item::Handler(ast::Handler { name, items }));
    //
    // interface
    //     .or(handler)
    //     .repeated()
    //     .collect::<Vec<_>>()
    //     .map(|items| File { items })

    struct_field
}

pub fn node_builder_to_node(builder: NodeBuilder, gb: &mut GreenNodeBuilder) {
    match builder {
        NodeBuilder::Node(kind, children) => {
            gb.start_node(kind.into());
            for child in children {
                node_builder_to_node(child, gb)
            }
            gb.finish_node();
        }
        Token(token) => gb.token(token.kind().into(), token.text()),
        Children(children) => {
            for child in children {
                node_builder_to_node(child, gb)
            }
        }
    }
}
