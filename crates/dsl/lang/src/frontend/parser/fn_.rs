use crate::frontend::ast;
use crate::frontend::lexer::Token;
use crate::frontend::lexer::Token::*;
use crate::frontend::parser::state::Parser;
use crate::frontend::parser::type_::type_;

pub fn fn_sig(p: &mut Parser, rhs: Token) {
    let m = p.open();
    p.expect_any(FN_TYPES);
    p.expect(Ident);
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
    p.expect(Ident);
    p.expect(Colon);
    type_(p);
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
        type_(p);
        if !p.at(RParen) {
            p.expect(Comma);
        }
    }
    p.expect(RParen);
    p.close::<ast::FnEvents>(m);
}

