use crate::ast::{AstStruct, ErrorNode};
use crate::lexer::Token;
use crate::syntax::SyntaxKind;
use itertools::{peek_nth, PeekNth};
use logos::Span;
use rowan::{Checkpoint, GreenNodeBuilder};

pub struct State<'source, 'cache, I: Iterator<Item = (Token<'source>, Span)>> {
    source: PeekNth<I>,
    pub(crate) builder: GreenNodeBuilder<'cache>,
}

impl<'source, 'cache, I: Iterator<Item = (Token<'source>, Span)>> State<'source, 'cache, I> {
    pub fn new(source: I, builder: GreenNodeBuilder<'cache>) -> Self {
        Self {
            source: peek_nth(source),
            builder,
        }
    }

    pub fn open(&mut self) -> Checkpoint {
        self.builder.checkpoint()
    }

    pub fn close<T: AstStruct>(&mut self, checkpoint: Checkpoint) -> Result<(), ()> {
        self.builder.start_node_at(checkpoint, T::KIND.into());
        Ok(())
    }

    pub fn peek(&mut self) -> Option<&Token<'source>> {
        self.skip_ws();
        self.source.peek().map(|(it, _)| it)
    }

    pub fn next(&mut self) -> Option<Token<'source>> {
        self.skip_ws();
        self.source.next().map(|(it, _)| {
            self.builder.token(it.kind().into(), it.text());
            it
        })
    }

    pub fn next_if(&mut self, func: impl FnOnce(&Token<'source>) -> bool) -> Option<Token<'source>> {
        if let Some(it) = self.peek() {
            if func(it) {
                return self.next();
            }
        }
        None
    }

    pub fn next_if_eq(&mut self, expected: &Token<'source>) -> Option<Token<'source>> {
        self.next_if(|next| next == expected)
    }

    fn skip_ws(&mut self)  {
        while let Some((token,_)) = self.source.next_if(|(token,_)| match token {
            Token::Whitespace(_) => true,
            Token::LineComment(_) => true,
            _ => false,
        }) {
            self.builder.token(token.kind().into(), token.text());
        }
    }
}
