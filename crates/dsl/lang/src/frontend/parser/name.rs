use crate::frontend::ast;
use crate::frontend::lexer::Token::Ident;
use crate::frontend::parser::parser::Parser;

pub fn name(p: &mut Parser) {
    let m = p.open();
    p.expect(Ident);
    p.close::<ast::Name>(m);
}

pub fn name_ref(p: &mut Parser) {
    let m = p.open();
    p.expect(Ident);
    p.close::<ast::NameRef>(m);
}