use crate::ast;
use crate::lexer::Token::*;
use crate::parser::state::Parser;
use crate::parser::typ::typ;

pub fn map_collection(p: &mut Parser) {
    let m = p.open();
    p.expect(MapKw);
    p.expect(Ident);
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
    p.expect(Ident);
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
    p.expect(Ident);
    p.expect(Colon);
    typ(p);
    if !p.at(Semicolon) {
        p.expect(Comma);
    }
    p.close::<ast::MapField>(m);
}

