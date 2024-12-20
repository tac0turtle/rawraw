use crate::frontend::ast;
use crate::frontend::lexer::Token::*;
use crate::frontend::parser::expr::expr;
use crate::frontend::parser::parser::Parser;

pub fn block(p: &mut Parser) {
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

fn for_stmt(p: &mut Parser) {
    let m = p.open();
    p.expect(ForKw);
    p.expect(Ident);
    p.expect(InKw);
    expr(p);
    block(p);
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
    p.expect(Ident);
    if p.at(Colon) {
        p.expect(Colon);
        expr(p);
    }
    p.close::<ast::ExprConstructField>(m);
}
