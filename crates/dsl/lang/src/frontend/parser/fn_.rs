use crate::frontend::ast;
use crate::frontend::lexer::Token;
use crate::frontend::lexer::Token::*;
use crate::frontend::parser::name::name;
use crate::frontend::parser::parser::Parser;
use crate::frontend::parser::type_::type_;

pub fn fn_sig(p: &mut Parser, rhs: Token) {
    let m = p.open();
    p.expect_any(FN_TYPES);
    name(p);
    fn_param_list(p);
    // events
    if p.at(EmitsKw) {
        fn_events(p);
    }
    // return type
    if !p.at(rhs) {
        let n = p.open();
        type_(p);
        p.close::<ast::FnRet>(n);
    }
    p.close::<ast::FnSignature>(m);
}

pub const FN_TYPES: &[Token] = &[TxKw, QueryKw, PureKw];

fn fn_param_list(p: &mut Parser) {
    let m = p.open();
    p.expect(LParen);
    while !p.at(RParen) && !p.eof() {
        if p.at_any(&[KeyKw, Ident]) {
            fn_param(p);
        } else {
            break;
        }
    }
    p.expect(RParen);
    p.close::<ast::FnParamList>(m);
}

fn fn_param(p: &mut Parser) {
    let m = p.open();
    p.eat(KeyKw);
    name(p);
    p.expect(Colon);
    fn_param_modifier(p);
    type_(p);
    if !p.at(RParen) {
        p.expect(Comma);
    }
    p.close::<ast::FnParam>(m);
}

pub fn fn_param_modifier(p: &mut Parser) {
    if p.at_any(FN_PARAM_MODIFIER_KW) {
        let n = p.open();
        p.advance();
        p.close::<ast::FnParamModifier>(n);
    }
}

const FN_PARAM_MODIFIER_KW: &[Token] = &[MutKw, RefKw, TransferKw];

fn fn_events(p: &mut Parser) {
    let m = p.open();
    p.expect(EmitsKw);
    p.expect(LParen);
    while !p.at(RParen) && !p.eof() {
        type_(p);
        if !p.at(RParen) {
            p.expect(Comma);
        }
    }
    p.expect(RParen);
    p.close::<ast::FnEvents>(m);
}

