use crate::frontend::ast;
use crate::frontend::lexer::Token::*;
use crate::frontend::parser::state::Parser;

// parse a type
pub fn typ(p: &mut Parser) {
    if p.at(LSquare) {
        let m = p.open();
        p.expect(LSquare);
        p.expect(RSquare);
        typ(p);
        p.close::<ast::TypeArray>(m);
    } else {
        let m = p.open();
        p.expect(Ident);
        p.close::<ast::TypeIdent>(m);
    }
}

