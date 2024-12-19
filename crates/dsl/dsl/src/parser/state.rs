use crate::ast::{AstStruct, ErrorNode};
use crate::lexer::Token;
use crate::syntax::{SyntaxKind, SyntaxNode};
use std::cell::Cell;
use std::ops::Range;
use rowan::{GreenNode, GreenNodeBuilder};

pub struct Parser<'a> {
    input: &'a str,
    tokens: Vec<(Token, Span)>,
    pos: usize,
    fuel: Cell<u32>,
    events: Vec<Event>,
}

pub type Span = Range<usize>;

enum Event {
    Open { kind: SyntaxKind },
    Close,
    Advance,
}

#[derive(Clone)]
pub struct MarkOpened {
    index: usize,
}

#[derive(Clone)]
pub struct MarkClosed {
    index: usize,
}

impl <'a> Parser<'a> {
    pub fn new(input: &'a str, source: Vec<(Token, Span)>) -> Self {
        let mut res = Self {
            input,
            tokens: source,
            pos: 0,
            fuel: Cell::new(256),
            events: vec![],
        };
        res.skip_ws();
        res
    }

    pub fn open(&mut self) -> MarkOpened {
        let mark = MarkOpened {
            index: self.events.len(),
        };
        self.events.push(Event::Open {
            kind: SyntaxKind::ERROR_NODE,
        });
        mark
    }

    pub fn close<T: AstStruct>(&mut self, m: MarkOpened) -> MarkClosed {
        self.events[m.index] = Event::Open { kind: T::KIND };
        self.events.push(Event::Close);
        MarkClosed { index: m.index }
    }

    pub fn open_before(&mut self, m: MarkClosed) -> MarkOpened {
        let mark = MarkOpened { index: m.index };
        self.events.insert(
            m.index,
            Event::Open { kind: SyntaxKind::ERROR_NODE },
        );
        mark
    }

    pub fn advance(&mut self) {
        assert!(!self.eof());
        self.events.push(Event::Advance);
        self.fuel.set(256);
        self.pos += 1;
        self.skip_ws()
    }

    pub fn eof(&self) -> bool {
        self.pos == self.tokens.len()
    }

    pub fn cur(&self) -> Token {
        self.nth(0)
    }

    pub fn nth(&self, lookahead: usize) -> Token {
        if self.fuel.get() == 0 {
            panic!("parser is stuck")
        } else {
            self.fuel.set(self.fuel.get() - 1);
        }
        self.tokens
            .get(self.pos + lookahead)
            .map_or(Token::Eof, |(token, _)| token.clone())
    }

    pub fn at(&self, token: Token) -> bool {
        self.nth(0) == token
    }

    pub fn at_any(&self, tokens: &[Token]) -> bool {
        self.at_f(|it| tokens.contains(&it))
    }

    pub fn at_f(&self, f: impl FnOnce(Token) -> bool) -> bool {
        f(self.nth(0))
    }

    pub fn at_exact(&self, tokens: &[Token]) -> bool {
        let mut i = 0;
        for token in tokens {
            if self.nth(i) != *token {
                return false;
            }
            i += 1;
        }
        true
    }

    pub fn eat_exact(&mut self, tokens: &[Token]) -> bool {
        if self.at_exact(tokens) {
            for _ in tokens {
                self.advance();
            }
            true
        } else {
            false
        }
    }

    pub fn eat_f(&mut self, f: impl FnOnce(Token) -> bool) -> bool {
        if self.at_f(f) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn eat_any(&mut self, tokens: &[Token]) -> bool {
        self.eat_f(|it| tokens.contains(&it))
    }

    pub fn eat(&mut self, f: Token) -> bool {
        self.eat_f(|it| it == f)
    }

    pub fn expect(&mut self, token: Token) {
        if self.eat(token.clone()) {
            return;
        }
        // TODO: Error reporting.
        eprintln!("expected {token:?}");
    }

    pub fn expect_f(&mut self, eq: impl FnOnce(Token) -> bool, error: &str) {
        if self.eat_f(eq) {
            return;
        }
        // TODO: Error reporting.
        eprintln!("error: {error}");
    }

    pub fn expect_any(&mut self, tokens: &[Token]) {
        if self.eat_any(tokens) {
            return;
        }
        // TODO: Error reporting.
        eprintln!("expected one of: {tokens:?}");
    }

    pub fn advance_with_error(&mut self, error: &str) {
        let m = self.open();
        // TODO: Error reporting.
        eprintln!("{error}");
        self.advance();
        self.close::<ErrorNode>(m);
    }

    fn skip_ws(&mut self) {
        while let Some((token, _)) = self.tokens.get(self.pos) {
            if is_whitespace(token) {
                self.advance();
                continue;
            }
            break;
        }
    }

    pub fn finish(self, mut builder: GreenNodeBuilder) -> GreenNode {
        builder.start_node(SyntaxKind::ROOT.into());
        let mut i = 0;
        for event in self.events {
            match event {
                Event::Open { kind } => builder.start_node(kind.into()),
                Event::Close => builder.finish_node(),
                Event::Advance => {
                    let (token, span) = &self.tokens[i];
                    let text = &self.input[span.start..span.end];
                    builder.token(token.kind().into(), text);
                    i += 1;
                }
            }
        }
        builder.finish_node();
        builder.finish()
    }
}

fn is_whitespace(token: &Token) -> bool {
    match token {
        Token::Whitespace => true,
        Token::LineComment => true,
        _ => false,
    }
}
