use crate::ast;
use crate::lexer::Token;
use crate::parser::state::Parser;

pub fn file(p: &mut Parser) {
    while !p.eof() {
        match p.cur() {
            Token::InterfaceKw => interface(p),
            _ => p.advance_with_error("expected interface, handler or impl"),
        }
    }
}

fn interface(parser: &mut Parser) {
    let m = parser.open();
    parser.expect(Token::InterfaceKw);
    expect_ident(parser);
    parser.expect(Token::LBracket);
    while !parser.at(Token::RBracket) && !parser.eof() {
        interface_item(parser);
    }
    parser.expect(Token::RBracket);
}

fn interface_item(p: &mut Parser) {
    if p.at_any(FN_TYPES) {
        fn_sig(p);
    } else {
        p.advance_with_error("expected interface item");
    }
}

fn fn_sig(p: &mut Parser) {
    let m = p.open();
    p.expect_any(FN_TYPES);
    expect_ident(p);
    if !p.at(Token::LParen) {
        fn_param_list(p);
    }
    p.close::<ast::FnSignature>(m)
}

 const FN_TYPES: &[Token] = &[Token::TxKw, Token::QueryKw, Token::PureKw];

fn fn_param_list(p: &mut Parser) {
    let m = p.open();
    p.expect(Token::LParen);
    while !p.at(Token::RParen) && !p.eof(){
        if at_ident(p) {
            fn_param(p);
        } else {
            break;
        }
    }
    p.expect(Token::RParen);
    p.close::<ast::FnArgList>(m)
}

fn fn_param(p: &mut Parser) {
    let m = p.open();
    p.eat(Token::KeyKw);
    expect_ident(p);
    p.expect(Token::Colon);
    typ(p);
    if !p.at(Token::RParen) {
        p.expect(Token::Comma);
    }
    p.close::<ast::FnArg>(m)
}

fn typ(p: &mut Parser) {
    let m= p.open();
    expect_ident(p);
    p.close::<ast::TypeIdent>(m)
}

// fn typ_array(p: &mut Parser) -> bool {
// }
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

fn at_ident(parser: &mut Parser) -> bool {
    parser.at_f(|it| matches!(it, Token::Ident(_)))
}

fn expect_ident(parser: &mut Parser) {
    parser.expect_f(|it| matches!(it, Token::Ident(_)), "expected identifier")
}
//
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
