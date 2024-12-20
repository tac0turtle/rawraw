use crate::frontend::ast;
use crate::frontend::lexer::Token::*;
use crate::frontend::parser::block::block;
use crate::frontend::parser::parser::Parser;

pub fn test(p: &mut Parser) {
    let m = p.open();
    p.expect(TestKw);
    p.expect(Ident);
    block(p);
    p.close::<ast::Test>(m);
}