use crate::frontend::ast;
use crate::frontend::lexer::Token::*;
use crate::frontend::parser::map::map_collection;
use crate::frontend::parser::state::Parser;
use crate::frontend::parser::type_::type_;

pub fn handler(p: &mut Parser) {
    let m = p.open();
    p.expect(HandlerKw);
    p.expect(Ident);
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

