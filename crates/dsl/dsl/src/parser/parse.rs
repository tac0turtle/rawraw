use logos::Span;
use rowan::{Checkpoint, GreenNodeBuilder};
use crate::ast;
use crate::ast::ErrorNode;
use crate::lexer::Token;
use crate::parser::state::State;
use crate::syntax::{SyntaxKind, SyntaxNode};

pub struct Parser<'source, 'cache, I: Iterator<Item = (Token<'source>, Span)>> {
    state: State<'source, 'cache, I>,
}

impl<'source, 'cache, I: Iterator<Item = (Token<'source>, Span)>> Parser<'source, 'cache, I> {
    pub fn new(source: I, mut builder: GreenNodeBuilder<'cache>) -> Self {
        builder.start_node(SyntaxKind::ROOT.into());
        Self { state: State::new(source, builder) }
    }

    pub fn parse(&mut self) -> Result<(), ()> {
        match self.cur()? {
            Token::InterfaceKw => self.interface()?,
            _ => {}
        }
        Ok(())
    }

    fn interface(&mut self) -> Result<(), ()> {
        let checkpoint = self.state.open();
        self.eat(Token::InterfaceKw);
        self.expect_ident(&checkpoint)?;
        self.expect(Token::LBracket, &checkpoint)?;
        self.state.close::<ast::Interface>(checkpoint)
    }

    fn fn_arg(&mut self) -> Result<(), ()> {
        let checkpoint = self.state.open();
        self.eat(Token::KeyKw);
        self.expect_ident(&checkpoint)?;
        self.expect(Token::Colon, &checkpoint)?;
        self.typ()?;
        self.state.close::<ast::FnArg>(checkpoint)
    }

    fn typ(&mut self) -> Result<(), ()> {
        if self.at(Token::LBracket) {
            self.typ_array()?;
        }
        let checkpoint = self.state.open();
        self.expect_kind(SyntaxKind::IDENT, &checkpoint)?;
        self.state.close::<ast::TypeIdent>(checkpoint)
    }

    fn expect_ident(&mut self, checkpoint: &Checkpoint) -> Result<(), ()> {
        self.expect_kind(SyntaxKind::IDENT, checkpoint)
    }

    fn typ_array(&mut self) -> Result<(), ()> {
        let checkpoint = self.state.open();
        self.expect(Token::LBracket, &checkpoint)?;
        self.expect(Token::RBracket,  &checkpoint)?;
        self.typ()?;
        self.state.close::<ast::TypeArray>(checkpoint)
    }

    pub fn at(&mut self, token: Token<'source>) -> bool {
        self.state.peek() == Some(&token)
    }

    pub fn eat(&mut self, token: Token<'source>) -> bool {
        self.state.next_if_eq(&token).is_some()
    }

    pub fn eat_kind(&mut self, kind: SyntaxKind) -> bool {
        self.state.next_if(|it| it.kind() == kind).is_some()
    }

    pub fn expect(&mut self, token: Token<'source>, checkpoint: &Checkpoint) -> Result<(), ()> {
        if !self.eat(token) {
            self.state.close::<ErrorNode>(checkpoint.clone())?;
            Err(())
        } else {
            Ok(())
        }
    }

    pub fn cur(&mut self) -> Result<&Token<'source>, ()> {
        self.state.peek().ok_or(())
    }

    pub fn expect_kind(&mut self, kind: SyntaxKind, checkpoint: &Checkpoint) -> Result<(), ()> {
        if !self.eat_kind(kind) {
            self.state.close::<ErrorNode>(checkpoint.clone())?;
            Err(())
        } else {
            Ok(())
        }
    }
    pub fn finish(self) -> SyntaxNode {
        let mut builder = self.state.builder;
        builder.finish_node();
        let node = builder.finish();
        SyntaxNode::new_root(node)
    }
}
