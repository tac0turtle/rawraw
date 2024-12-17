use logos::Span;
use crate::lexer::Token;
use crate::parser2::state::State;

mod state;

struct Parser<'source, 'cache, I: Iterator<Item = (Span, Token<'source>)>> {
    state: State<'source, 'cache, I>,
}

impl<'source, 'cache, I: Iterator<Item = (Span, Token<'source>)>> Parser<'source, 'cache, I> {
    fn at(&mut self, token: Token<'source>) -> bool {
        self.state.peek() == Some(&token)
    }

    fn expect(&mut self, token: Token<'source>) -> bool {
        self.state.next_if_eq(&token).is_some()
    }

    fn fn_arg(&mut self) {

    }
}