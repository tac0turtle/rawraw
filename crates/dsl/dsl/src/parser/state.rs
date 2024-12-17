use crate::ast::{AstStruct, ErrorNode};
use crate::lexer::Token;
use crate::syntax::{SyntaxKind, SyntaxNode};
use logos::Span;
use std::cell::Cell;
use rowan::GreenNodeBuilder;

pub struct Parser<'source> {
    tokens: Vec<(Token<'source>, Span)>,
    pos: usize,
    fuel: Cell<u32>,
    events: Vec<Event>,
}

enum Event {
    Open { kind: SyntaxKind },
    Close,
    Advance,
}

pub struct MarkOpened {
    index: usize,
}

impl<'source> Parser<'source> {
    pub fn new(source: Vec<(Token<'source>, Span)>) -> Self {
        let mut res = Self {
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

    pub fn close<T: AstStruct>(&mut self, m: MarkOpened) {
        self.events[m.index] = Event::Open { kind: T::KIND };
        self.events.push(Event::Close);
    }

    fn advance(&mut self) {
        assert!(!self.eof());
        self.events.push(Event::Advance);
        self.pos += 1;
        self.skip_ws()
    }

    pub fn eof(&self) -> bool {
        self.pos == self.tokens.len()
    }

    pub fn cur(&self) -> Token<'source> {
        self.nth(0)
    }

    pub fn nth(&self, lookahead: usize) -> Token<'source> {
        if self.fuel.get() == 0 {
            panic!("parser is stuck")
        }
        self.fuel.set(self.fuel.get() - 1);
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

    pub fn finish(self, builder: GreenNodeBuilder) -> SyntaxNode {
        let mut builder = builder;
        builder.start_node(SyntaxKind::ROOT.into());
        let mut i = 0;
        for event in self.events {
            match event {
                Event::Open { kind } => builder.start_node(kind.into()),
                Event::Close => builder.finish_node(),
                Event::Advance => {
                    let token = &self.tokens[i].0;
                    builder.token(token.kind().into(), token.text());
                    i += 1;
                }
            }
        }
        builder.finish_node();
        let node = builder.finish();
        SyntaxNode::new_root(node)
    }
}

fn is_whitespace(token: &Token) -> bool {
    match token {
        Token::Whitespace(_) => true,
        Token::LineComment(_) => true,
        _ => false,
    }
}
