use crate::ast;
use crate::ast::ErrorNode;
use crate::lexer::Token;
use crate::parser::state::Parser;
use crate::syntax::{SyntaxKind, SyntaxNode};
use rowan::Checkpoint;

pub fn parse(parser: &mut Parser) {
    match parser.cur() {
        Token::InterfaceKw => interface(parser),
        _ => {}
    }
}

fn interface(parser: &mut Parser) {
    let checkpoint = parser.open();
    parser.expect(Token::InterfaceKw);
    expect_ident(parser);
    parser.expect(Token::LBracket);
    parser.close::<ast::Interface>(checkpoint)
}
//
// fn fn_arg(&mut self) -> Result<(), ()> {
//     let checkpoint = self.state.open();
//     self.eat(Token::KeyKw);
//     self.expect_ident(&checkpoint)?;
//     self.expect(Token::Colon, &checkpoint)?;
//     self.typ()?;
//     self.state.close::<ast::FnArg>(checkpoint)
// }
//
// fn typ(&mut self) -> Result<(), ()> {
//     if self.at(Token::LBracket) {
//         self.typ_array()?;
//     }
//     let checkpoint = self.state.open();
//     self.expect_kind(SyntaxKind::IDENT, &checkpoint);
//     self.state.close::<ast::TypeIdent>(checkpoint)
// }

fn expect_ident(parser: &mut Parser) {
    parser.expect_f(|it| matches!(it, Token::Ident(_)), "expected identifier")
}
//
// fn typ_array(&mut self) -> Result<(), ()> {
//     let checkpoint = self.state.open();
//     self.expect(Token::LBracket, &checkpoint)?;
//     self.expect(Token::RBracket, &checkpoint)?;
//     self.typ()?;
//     self.state.close::<ast::TypeArray>(checkpoint)
// }
//
// pub fn at(&mut self, token: Token<'source>) -> bool {
//     self.state.peek() == Some(&token)
// }
//
// pub fn eat(&mut self, token: Token<'source>) -> bool {
//     self.state.next_if_eq(&token).is_some()
// }
//
// pub fn eat_kind(&mut self, kind: SyntaxKind) -> bool {
//     self.state.next_if(|it| it.kind() == kind).is_some()
// }
//
// pub fn expect(&mut self, token: Token<'source>, checkpoint: &Checkpoint) -> Result<(), ()> {
//     if !self.eat(token) {
//         self.state.close::<ErrorNode>(checkpoint.clone())?;
//         Err(())
//     } else {
//         Ok(())
//     }
// }
//
// pub fn cur(&mut self) -> Result<&Token<'source>, ()> {
//     self.state.peek().ok_or(())
// }
//
// pub fn expect_kind(&mut self, kind: SyntaxKind, checkpoint: &Checkpoint) -> Result<(), ()> {
//     if !self.eat_kind(kind) {
//         self.state.close::<ErrorNode>(checkpoint.clone())?;
//         Err(())
//     } else {
//         Ok(())
//     }
// }
// pub fn finish(self) -> SyntaxNode {
//     let mut builder = self.state.builder;
//     builder.finish_node();
//     let node = builder.finish();
//     SyntaxNode::new_root(node)
// }
