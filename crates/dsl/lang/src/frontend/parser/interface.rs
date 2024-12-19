use crate::frontend::ast;
use crate::frontend::lexer::Token::*;
use crate::frontend::parser::fn_::{fn_sig, FN_TYPES};
use crate::frontend::parser::state::Parser;
use crate::frontend::parser::struct_::{event_struct, struct_};

pub fn interface(p: &mut Parser) {
    let m = p.open();
    p.expect(InterfaceKw);
    p.expect(Ident);
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
        fn_sig(p, Semicolon);
        p.expect(Semicolon);
    } else if cur == StructKw {
       struct_(p);
    } else if cur == EventKw {
        event_struct(p);
    } else {
        p.advance_with_error("expected interface item");
    }
}
