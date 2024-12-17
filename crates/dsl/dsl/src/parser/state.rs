use itertools::{peek_nth, PeekNth};
use logos::{Span, SpannedIter};
use rowan::GreenNodeBuilder;
use crate::lexer::Token;

pub struct State<'source, 'cache, I: Iterator<Item = (Span, Token<'source>)>> {
    source: PeekNth<I>,
    builder: GreenNodeBuilder<'cache>,
}

impl<'source, 'cache, I: Iterator<Item = (Span, Token<'source>)>> State<'source, 'cache, I> {
    // pub fn new(source: SpannedIter<'source, Token<'source>>, builder: GreenNodeBuilder<'cache>) -> Self {
    //     Self { source: peek_nth(source), builder }
    // }
    //
    pub fn peek(&mut self) -> Option<&Token<'source>> {
        self.source.peek().map(|(_, it)| it)
    }

    pub fn next(&mut self) -> Option<Token<'source>> {
        self.source.next().map(|(_, it)| {
            self.builder.token(it.kind().into(), it.text());
            it
        })
    }

    pub fn next_if(&mut self, func: impl FnOnce(&Token<'source>) -> bool) -> Option<Token<'source>> {
        if let Some(it) = self.peek() {
            if func(it) {
                return self.next()
            }
        }
        None
    }

    pub fn next_if_eq(&mut self, expected: &Token<'source>) -> Option<Token<'source>> {
        self.next_if(|next| next == expected)
    }
}
