use crate::frontend::ast;
use crate::frontend::lexer::Token::*;
use crate::frontend::parser::parser::Parser;

// parse a type
pub fn type_(p: &mut Parser) {
    if p.at(LSquare) {
        let m = p.open();
        p.expect(LSquare);
        p.expect(RSquare);
        type_(p);
        p.close::<ast::TypeArray>(m);
    } else {
        let m = p.open();
        p.expect(Ident);
        p.close::<ast::TypeIdent>(m);
    }
}

