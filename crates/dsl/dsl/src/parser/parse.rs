use crate::ast;
use crate::lexer::Token;
use crate::lexer::Token::*;
use crate::parser::state::{MarkClosed, Parser};

pub fn file(p: &mut Parser) {
    let m = p.open();
    while !p.eof() {
        match p.cur() {
            InterfaceKw => interface(p),
            HandlerKw => handler(p),
            ImplKw => impl_(p),
            _ => p.advance_with_error("expected interface, handler or impl"),
        }
    }
    p.close::<ast::File>(m);
}

fn interface(p: &mut Parser) {
    let m = p.open();
    p.expect(InterfaceKw);
    expect_ident(p);
    p.expect(LCurly);
    while !p.at(RCurly) && !p.eof() {
        interface_item(p);
    }
    p.expect(RCurly);
    p.close::<ast::Interface>(m);
}

fn interface_item(p: &mut Parser) {
    let cur = p.cur();
    if FN_TYPES.contains(&cur) {
        fn_sig(p);
        p.expect(Semicolon);
    } else if cur == StructKw {
        struct_(p);
    } else if cur == EventKw {
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
    if p.at(EmitsKw) {
        fn_events(p);
    }
    // return type
    if !p.at(Semicolon) {
        let m = p.open();
        expect_ident(p);
        p.close::<ast::FnRet>(m);
    }
    p.expect(Semicolon);
    p.close::<ast::FnSignature>(m);
}

const FN_TYPES: &[Token] = &[TxKw, QueryKw, PureKw];

fn fn_param_list(p: &mut Parser) {
    let m = p.open();
    p.expect(LParen);
    while !p.at(RParen) && !p.eof() {
        fn_param(p);
    }
    p.expect(RParen);
    p.close::<ast::FnParamList>(m);
}

fn fn_param(p: &mut Parser) {
    let m = p.open();
    p.eat(KeyKw);
    expect_ident(p);
    p.expect(Colon);
    typ(p);
    if !p.at(RParen) {
        p.expect(Comma);
    }
    p.close::<ast::FnParam>(m);
}

fn fn_events(p: &mut Parser) {
    let m = p.open();
    p.expect(EmitsKw);
    p.expect(LParen);
    while !p.at(RParen) && !p.eof() {
        typ(p);
        if !p.at(RParen) {
            p.expect(Comma);
        }
    }
    p.expect(RParen);
    p.close::<ast::FnEvents>(m);
}

fn typ(p: &mut Parser) {
    if p.at(LCurly) {
        let m = p.open();
        p.expect(LCurly);
        p.expect(RCurly);
        typ(p);
        p.close::<ast::TypeArray>(m);
    }
    let m = p.open();
    expect_ident(p);
    p.close::<ast::TypeIdent>(m);
}

fn expect_ident(parser: &mut Parser) {
    if !parser.eat(Ident) {
        parser.advance_with_error("expected identifier");
    }
}

fn struct_(p: &mut Parser) {
    let m = p.open();
    p.expect(StructKw);
    struct_inner(p);
    p.close::<ast::Struct>(m);
}

fn struct_inner(p: &mut Parser) {
    expect_ident(p);
    p.expect(LCurly);
    while !p.at(RCurly) && !p.eof() {
        struct_field(p);
    }
    p.expect(RCurly);
}

fn struct_field(p: &mut Parser) {
    let m = p.open();
    expect_ident(p);
    p.expect(Colon);
    typ(p);
    if !p.at(RCurly) {
        p.expect(Comma);
    }
    p.close::<ast::StructField>(m);
}

fn event(p: &mut Parser) {
    let m = p.open();
    p.expect(EventKw);
    struct_inner(p);
    p.close::<ast::Event>(m);
}

fn handler(p: &mut Parser) {
    let m = p.open();
    p.expect(HandlerKw);
    expect_ident(p);
    p.expect(LCurly);
    while !p.at(RCurly) && !p.eof() {
        handler_item(p);
    }
    p.expect(RCurly);
    p.close::<ast::Handler>(m);
}

fn handler_item(p: &mut Parser) {
    let cur = p.cur();
    if cur == MapKw {
        map_collection(p);
    } else if cur == ClientKw {
        client(p);
    } else {
        p.advance_with_error("expected handler item");
    }
}

fn map_collection(p: &mut Parser) {
    let m = p.open();
    p.expect(MapKw);
    map_key_fields(p);
    if p.at(RArrow) {
        map_value_fields(p);
    }
    p.expect(Semicolon);
    p.close::<ast::MapCollection>(m);
}

fn map_key_fields(p: &mut Parser) {
    let m = p.open();
    p.expect(LSquare);
    while !p.at(RSquare) && !p.eof() {
        map_key_field(p);
    }
    p.expect(RSquare);
    p.close::<ast::MapKeyFields>(m);
}

fn map_key_field(p: &mut Parser) {
    let m = p.open();
    expect_ident(p);
    p.expect(Colon);
    typ(p);
    if !p.at(RSquare) {
        p.expect(Comma);
    }
    p.close::<ast::MapField>(m);
}

fn map_value_fields(p: &mut Parser) {
    let m = p.open();
    p.expect(RArrow);
    while !p.at(Semicolon) && !p.eof() {
        map_value_field(p);
    }
    p.close::<ast::MapValueFields>(m);
}

fn map_value_field(p: &mut Parser) {
    let m = p.open();
    expect_ident(p);
    p.expect(Colon);
    typ(p);
    if !p.at(Semicolon) {
        p.expect(Comma);
    }
    p.close::<ast::MapField>(m);
}

fn client(p: &mut Parser) {
    let m = p.open();
    p.expect(ClientKw);
    expect_ident(p);
    p.expect(Colon);
    client_types(p);
    p.expect(Semicolon);
    p.close::<ast::Client>(m);
}

fn client_types(p: &mut Parser) {
    let m = p.open();
    while !p.at(Semicolon) && !p.eof() {
        client_type(p);
    }
    p.close::<ast::ClientTypes>(m);
}

fn client_type(p: &mut Parser) {
    let m = p.open();
    typ(p);
    if !p.at(Semicolon) {
        p.expect(Comma);
    }
    p.close::<ast::ClientType>(m);
}

fn impl_(p: &mut Parser) {
    let m = p.open();
    p.expect(ImplKw);
    expect_ident(p);
    {
        let m = p.open();
        p.expect(ForKw);
        expect_ident(p);
        p.close::<ast::ImplFor>(m);
    }
    p.expect(LCurly);
    while !p.at(RCurly) && !p.eof() {
        impl_item(p);
    }
    p.expect(RCurly);
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
    p.expect(LCurly);
    while !p.at(RCurly) && !p.eof() {
        statement(p);
    }
    p.expect(RCurly);
    p.close::<ast::FnBlock>(m);
}

fn statement(p: &mut Parser) {
    let cur = p.cur();
    match cur {
        ForKw => {
            for_stmt(p);
            // optional semicolon
            p.eat(Semicolon);
        },
        _ => {
            expr(p);
            if !p.at(RCurly) {
                p.expect(Semicolon);
            }
        }
    }
}

fn expr(p: &mut Parser) {
    expr_rec(p, Eof);
}

fn expr_rec(p: &mut Parser, left: Token) {
    fn right_binds_tighter(left: &Token, right: &Token) -> bool {
        fn tightness(kind: &Token) -> Option<usize> {
            [
                // Precedence table:
                &[Eq],
                // [Plus, Minus].as_slice(),
                // &[Star, Slash],
                &[Dot],
            ]
            .iter()
            .position(|level| level.contains(kind))
        }
        let Some(right_tightness) = tightness(right) else {
            return false;
        };
        let Some(left_tightness) = tightness(left) else {
            assert_eq!(left, &Eof);
            return true;
        };
        right_tightness > left_tightness
    }

    // ExprCall
    let mut lhs = expr_delimited(p);
    while p.at(LParen) {
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
        Ident => {
            expect_ident(p);
            let mut lhs = p.close::<ast::NameExpr>(m);
            //  ExprConstruct
            if p.at(LCurly) {
                let m = p.open_before(lhs);
                expr_construct_field_list(p);
                lhs = p.close::<ast::ExprConstruct>(m);
            }
            lhs
        }
        LParen => {
            p.expect(LParen);
            expr(p);
            p.expect(RParen);
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
    assert!(p.at(LParen));
    let m = p.open();
    p.expect(LParen);
    while !p.at(RParen) && !p.eof() {
        arg(p);
    }
    p.expect(RParen);
    p.close::<ast::ArgList>(m);
}

// Arg = Expr ','?
fn arg(p: &mut Parser) {
    let m = p.open();
    expr(p);
    if !p.at(RParen) {
        p.expect(Comma);
    }
    p.close::<ast::Arg>(m);
}

fn for_stmt(p: &mut Parser) {
    let m = p.open();
    p.expect(ForKw);
    expect_ident(p);
    p.expect(InKw);
    expr(p);
    fn_block(p);
    p.close::<ast::ForStmt>(m);
}

fn expr_construct_field_list(p: &mut Parser) {
    let m = p.open();
    p.expect(LCurly);
    while !p.at(RCurly) && !p.eof() {
        expr_construct_field(p);
    }
    p.expect(RCurly);
    p.close::<ast::ExprConstructFieldList>(m);
}

fn expr_construct_field(p: &mut Parser) {
    let m = p.open();
    expect_ident(p);
    if p.at(Colon) {
        p.expect(Colon);
        expr(p);
    }
    p.close::<ast::ExprConstructField>(m);
}