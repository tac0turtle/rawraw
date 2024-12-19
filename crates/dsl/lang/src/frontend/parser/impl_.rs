use crate::frontend::ast;
use crate::frontend::lexer::Token::*;
use crate::frontend::parser::block::block;
use crate::frontend::parser::fn_::{fn_sig, FN_TYPES};
use crate::frontend::parser::state::Parser;

pub fn impl_block(p: &mut Parser) {
    let m = p.open();
    p.expect(ImplKw);
    p.expect(Ident);
    {
        let m = p.open();
        p.expect(ForKw);
        p.expect(Ident);
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
    fn_sig(p, LCurly);
    block(p);
    p.close::<ast::ImplFn>(m);
}
