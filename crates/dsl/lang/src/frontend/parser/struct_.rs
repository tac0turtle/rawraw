use crate::frontend::ast;
use crate::frontend::lexer::Token::*;
use crate::frontend::parser::state::Parser;
use crate::frontend::parser::type_::type_;

pub fn struct_(p: &mut Parser) {
    let m = p.open();
    p.expect(StructKw);
    struct_inner(p);
    p.close::<ast::Struct>(m);
}

fn struct_inner(p: &mut Parser) {
    p.expect(Ident);
    p.expect(LCurly);
    while !p.at(RCurly) && !p.eof() {
        struct_field(p);
    }
    p.expect(RCurly);
}

fn struct_field(p: &mut Parser) {
    let m = p.open();
    p.expect(Ident);
    p.expect(Colon);
    type_(p);
    if !p.at(RCurly) {
        p.expect(Comma);
    }
    p.close::<ast::StructField>(m);
}

pub fn event_struct(p: &mut Parser) {
    let m = p.open();
    p.expect(EventKw);
    struct_inner(p);
    p.close::<ast::Event>(m);
}

