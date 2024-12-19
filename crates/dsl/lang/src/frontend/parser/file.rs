use crate::frontend::ast;
use crate::frontend::lexer::Token::*;
use crate::frontend::parser::handler::handler;
use crate::frontend::parser::impl_::impl_block;
use crate::frontend::parser::interface::interface;
use crate::frontend::parser::state::Parser;

pub fn file(p: &mut Parser) {
    let m = p.open();
    while !p.eof() {
        match p.cur() {
            InterfaceKw => interface(p),
            HandlerKw => handler(p),
            ImplKw => impl_block(p),
            _ => p.advance_with_error("expected interface, handler or impl"),
        }
    }
    p.close::<ast::File>(m);
}
