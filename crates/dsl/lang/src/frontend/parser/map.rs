use crate::frontend::ast;
use crate::frontend::lexer::Token;
use crate::frontend::lexer::Token::*;
use crate::frontend::parser::state::Parser;
use crate::frontend::parser::type_::type_;

pub fn at_start_map(p: &mut Parser) -> bool {
    p.at_any(&MAP_START_KWS)
}

const MAP_START_KWS: &[Token] = &[MapKw, ScopedKw];

pub fn map_collection(p: &mut Parser) {
    let m = p.open();
    p.eat(ScopedKw);
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
    type_(p);
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
    type_(p);
    if !p.at(Semicolon) {
        p.expect(Comma);
    }
    p.close::<ast::MapField>(m);
}

