use crate::frontend::ast;
use crate::frontend::lexer::Token::*;
use crate::frontend::parser::collections::{
    at_start_map, at_start_var, map_collection, var_collection,
};
use crate::frontend::parser::fn_::FN_TYPES;
use crate::frontend::parser::impl_::impl_fn;
use crate::frontend::parser::parser::Parser;
use crate::frontend::parser::type_::type_;

pub fn object(p: &mut Parser) {
    let m = p.open();
    p.expect(ObjectKw);
    p.expect(Ident);
    p.expect(LCurly);
    while !p.at(RCurly) && !p.eof() {
        object_item(p);
    }
    p.expect(RCurly);
    p.close::<ast::Object>(m);
}

fn object_item(p: &mut Parser) {
    if at_start_map(p) {
        map_collection(p);
    } else if at_start_var(p) {
        var_collection(p);
    } else if p.at(ClientKw) {
        client(p);
    } else if p.at_any(&FN_TYPES) {
        impl_fn(p);
    } else {
        p.advance_with_error("expected handler item");
    }
}

fn client(p: &mut Parser) {
    let m = p.open();
    p.expect(ClientKw);
    p.expect(Ident);
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
    type_(p);
    if !p.at(Semicolon) {
        p.expect(Comma);
    }
    p.close::<ast::ClientType>(m);
}
