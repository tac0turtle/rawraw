use crate::ast;
use crate::lexer::Token;
use crate::parser::state::{MarkClosed, Parser};

pub fn file(p: &mut Parser) {
    while !p.eof() {
        match p.cur() {
            Token::InterfaceKw => interface(p),
            Token::HandlerKw => handler(p),
            Token::ImplKw => impl_(p),
            _ => p.advance_with_error("expected interface, handler or impl"),
        }
    }
}

fn interface(parser: &mut Parser) {
    let m = parser.open();
    parser.expect(Token::InterfaceKw);
    expect_ident(parser);
    parser.expect(Token::LCurly);
    while !parser.at(Token::RCurly) && !parser.eof() {
        interface_item(parser);
    }
    parser.expect(Token::RCurly);
}

fn interface_item(p: &mut Parser) {
    let cur = p.cur();
    if FN_TYPES.contains(&cur) {
        fn_sig(p);
        p.expect(Token::Semicolon);
    } else if cur == Token::StructKw {
        struct_(p);
    } else if cur == Token::EventKw {
        event(p);
    } else {
        p.advance_with_error("expected interface item");
    }
}

fn fn_sig(p: &mut Parser) {
    let m = p.open();
    p.expect_any(FN_TYPES);
    expect_ident(p);
    fn_param_list(p);
    // events
    if p.at(Token::EmitsKw) {
        fn_events(p);
    }
    // return type
    if !p.at(Token::Semicolon) {
        let m = p.open();
        expect_ident(p);
        p.close::<ast::FnRet>(m);
    }
    p.expect(Token::Semicolon);
    p.close::<ast::FnSignature>(m);
}

const FN_TYPES: &[Token] = &[Token::TxKw, Token::QueryKw, Token::PureKw];

fn fn_param_list(p: &mut Parser) {
    let m = p.open();
    p.expect(Token::LParen);
    while !p.at(Token::RParen) && !p.eof() {
        fn_param(p);
    }
    p.expect(Token::RParen);
    p.close::<ast::FnParamList>(m);
}

fn fn_param(p: &mut Parser) {
    let m = p.open();
    p.eat(Token::KeyKw);
    expect_ident(p);
    p.expect(Token::Colon);
    typ(p);
    if !p.at(Token::RParen) {
        p.expect(Token::Comma);
    }
    p.close::<ast::FnParam>(m);
}

fn fn_events(p: &mut Parser) {
    let m = p.open();
    p.expect(Token::EmitsKw);
    p.expect(Token::LParen);
    while !p.at(Token::RParen) && !p.eof() {
        typ(p);
        if !p.at(Token::RParen) {
            p.expect(Token::Comma);
        }
    }
    p.expect(Token::RParen);
    p.close::<ast::FnEvents>(m);
}

fn typ(p: &mut Parser) {
    if p.at(Token::LCurly) {
        let m = p.open();
        p.expect(Token::LCurly);
        p.expect(Token::RCurly);
        typ(p);
        p.close::<ast::TypeArray>(m);
    }
    let m = p.open();
    expect_ident(p);
    p.close::<ast::TypeIdent>(m);
}

fn at_ident(parser: &mut Parser) -> bool {
    parser.at_f(|it| matches!(it, Token::Ident(_)))
}

fn expect_ident(parser: &mut Parser) {
    if !parser.eat_f(|it| matches!(it, Token::Ident(_))) {
        // TODO maybe we don't advance but instead just call expect
        parser.advance_with_error("expected identifier")
    }
}

fn struct_(p: &mut Parser) {
    let m = p.open();
    p.expect(Token::StructKw);
    struct_inner(p);
    p.close::<ast::Struct>(m);
}

fn struct_inner(p: &mut Parser) {
    expect_ident(p);
    p.expect(Token::LCurly);
    while !p.at(Token::RCurly) && !p.eof() {
        struct_field(p);
    }
    p.expect(Token::RCurly);
}

fn struct_field(p: &mut Parser) {
    let m = p.open();
    expect_ident(p);
    p.expect(Token::Colon);
    typ(p);
    if !p.at(Token::RCurly) {
        p.expect(Token::Comma);
    }
    p.close::<ast::StructField>(m);
}

fn event(p: &mut Parser) {
    let m = p.open();
    p.expect(Token::EventKw);
    struct_inner(p);
    p.close::<ast::Event>(m);
}

fn handler(p: &mut Parser) {
    let m = p.open();
    p.expect(Token::HandlerKw);
    expect_ident(p);
    p.expect(Token::LCurly);
    while !p.at(Token::RCurly) && !p.eof() {
        handler_item(p);
    }
    p.expect(Token::RCurly);
    p.close::<ast::Handler>(m);
}

fn handler_item(p: &mut Parser) {
    let cur = p.cur();
    if cur == Token::MapKw {
        map_collection(p);
    } else if cur == Token::ClientKw {
        client(p);
    } else {
        p.advance_with_error("expected handler item");
    }
}

fn map_collection(p: &mut Parser) {
    let m = p.open();
    p.expect(Token::MapKw);
    map_key_fields(p);
    if p.at(Token::RArrow) {
        map_value_fields(p);
    }
    p.expect(Token::Semicolon);
    p.close::<ast::MapCollection>(m);
}

fn map_key_fields(p: &mut Parser) {
    let m = p.open();
    p.expect(Token::LSquare);
    while !p.at(Token::RSquare) && !p.eof() {
        map_key_field(p);
    }
    p.expect(Token::RSquare);
    p.close::<ast::MapKeyFields>(m);
}

fn map_key_field(p: &mut Parser) {
    let m = p.open();
    expect_ident(p);
    p.expect(Token::Colon);
    typ(p);
    if !p.at(Token::RSquare) {
        p.expect(Token::Comma);
    }
    p.close::<ast::MapField>(m);
}

fn map_value_fields(p: &mut Parser) {
    let m = p.open();
    p.expect(Token::RArrow);
    while !p.at(Token::Semicolon) && !p.eof() {
        map_value_field(p);
    }
    p.close::<ast::MapValueFields>(m);
}

fn map_value_field(p: &mut Parser) {
    let m = p.open();
    expect_ident(p);
    p.expect(Token::Colon);
    typ(p);
    if !p.at(Token::Semicolon) {
        p.expect(Token::Comma);
    }
    p.close::<ast::MapField>(m);
}

fn client(p: &mut Parser) {
    let m = p.open();
    p.expect(Token::ClientKw);
    expect_ident(p);
    p.expect(Token::Colon);
    client_types(p);
    p.expect(Token::Semicolon);
    p.close::<ast::Client>(m);
}

fn client_types(p: &mut Parser) {
    let m = p.open();
    while !p.at(Token::Semicolon) && !p.eof() {
        client_type(p);
    }
    p.close::<ast::ClientTypes>(m);
}

fn client_type(p: &mut Parser) {
    let m = p.open();
    typ(p);
    if !p.at(Token::Semicolon) {
        p.expect(Token::Comma);
    }
    p.close::<ast::ClientType>(m);
}

fn impl_(p: &mut Parser) {
    let m = p.open();
    p.expect(Token::ImplKw);
    expect_ident(p);
    {
        let m = p.open();
        p.expect(Token::ForKw);
        expect_ident(p);
        p.close::<ast::ImplFor>(m);
    }
    p.expect(Token::LCurly);
    while !p.at(Token::RCurly) && !p.eof() {
        impl_item(p);
    }
    p.expect(Token::RCurly);
    p.close::<ast::Impl>(m);
}

fn impl_item(p: &mut Parser) {
    let cur = p.cur();
    if FN_TYPES.contains(&cur) {
        impl_fn(p);
    } else {
        p.advance_with_error("expected impl item");
    }
}

fn impl_fn(p: &mut Parser) {
    let m = p.open();
    fn_sig(p);
    fn_block(p);
    p.close::<ast::ImplFn>(m);
}

fn fn_block(p: &mut Parser) {
    let m = p.open();
    p.expect(Token::LSquare);
    while !p.at(Token::RSquare) && !p.eof() {
        statement(p);
    }
    p.expect(Token::RSquare);
    p.close::<ast::FnBlock>(m);
}

fn statement(p: &mut Parser) {
    expr(p);
    p.advance_with_error("expected statement");
}

fn expr(p: &mut Parser) {
    let mut lhs = expr_delimited(p);

    // ExprCall = Expr ArgList
    while p.at(Token::LParen) {
        let m = p.open_before(lhs);
        arg_list(p);
        lhs = p.close::<ast::ExprCall>(m);
    }
}

fn expr_rec(p: &mut Parser, left: Token) {
    fn right_binds_tighter(left: &Token, right: &Token) -> bool {
        fn tightness(kind: &Token) -> Option<usize> {
            [
                // Precedence table:
                &[Token::Dot],
                // [Plus, Minus].as_slice(),
                // &[Star, Slash],
            ]
            .iter()
            .position(|level| level.contains(kind))
        }
        let Some(right_tightness) = tightness(right) else {
            return false;
        };
        let Some(left_tightness) = tightness(left) else {
            assert_eq!(left, &Token::Eof);
            return true;
        };
        right_tightness > left_tightness
    }

    // ExprCall
    let mut lhs = expr_delimited(p);
    while p.at(Token::LParen) {
        let m = p.open_before(lhs);
        arg_list(p);
        lhs = p.close::<ast::ExprCall>(m);
    }
    loop {
        let right = p.nth(0);
        if right_binds_tighter(&left, &right) {
            let m = p.open_before(lhs);
            p.advance();
            expr_rec(p, right);
            lhs = p.close::<ast::ExprBinary>(m);
        } else {
            break;
        }
    }
}

fn expr_delimited(p: &mut Parser) -> MarkClosed {
    let m = p.open();
    match p.cur() {
        Token::Ident(_) => {
            expect_ident(p);
            p.close::<ast::NameExpr>(m)
        }
        Token::LParen => {
            p.expect(Token::LParen);
            expr(p);
            p.expect(Token::RParen);
            p.close::<ast::ExprParen>(m)
        }
        _ => {
            if !p.eof() {
                p.advance();
            }
            p.close::<ast::ErrorNode>(m)
        }
    }
}

// ArgList = '(' Arg* ')'
fn arg_list(p: &mut Parser) {
    assert!(p.at(Token::LParen));
    let m = p.open();
    p.expect(Token::LParen);
    while !p.at(Token::RParen) && !p.eof() {
        arg(p);
    }
    p.expect(Token::RParen);
    p.close::<ast::ArgList>(m);
}

// Arg = Expr ','?
fn arg(p: &mut Parser) {
    let m = p.open();
    expr(p);
    if !p.at(Token::RParen) {
        p.expect(Token::Comma);
    }
    p.close::<ast::Arg>(m);
}
