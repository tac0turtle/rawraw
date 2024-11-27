use std::fmt;
use logos::{Logos, SpannedIter};

#[derive(Logos, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    #[regex(r#"[ \t\n\r\f\v]+"#)]
    Whitespace,
    #[regex(r#"//[^\n\r\f\v]*"#, |lex| lex.slice())]
    LineComment(&'a str),
    // TODO block comments
    // #[token(r#"\/\*(\*(?!\/)|[^*])*\*\/"#, |lex| lex.slice())]
    // BlockComment(&'a str),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice())]
    Ident(&'a str),
    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice())]
    String(&'a str),
    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice())]
    Numeric(&'a str),
    #[token(";")]
    Semi,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,
    #[token("=")]
    Equal,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("&")]
    And,
    #[token("|")]
    Or,
    #[token("^")]
    Xor,
    #[token("~")]
    Tilde,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    NotEqual,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("&&")]
    AndAnd,
    #[token("||")]
    OrOr,
    #[token("!")]
    Not,
    #[token("?")]
    Question,
}

impl <'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;
//
// pub struct Lexer<'a> {
//     token_stream: SpannedIter<'a, Token<'a>>,
// }
//
// impl<'input> Lexer<'input> {
//     pub fn new(input: &'input str) -> Self {
//         Self { token_stream: Token::lexer(input).spanned() }
//     }
// }
//
// impl<'input> Iterator for Lexer<'input> {
//     type Item = Result<Token<'input>, LexicalError>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.token_stream
//             .next()
//             .map(|(token, span)| Ok((span.start, token?, span.end)))
//     }
// }