use std::fmt;
use logos::{Logos, SpannedIter};

#[derive(Logos, Debug, PartialEq, Eq, Clone)]
pub enum Token<'a> {
    Error,
    #[regex(r#"[ \t\n\r\f\v]+"#)]
    Whitespace(&'a str),
    #[regex(r#"//[^\n\r\f\v]*"#, |lex| lex.slice())]
    LineComment(&'a str),
    // TODO block comments
    // #[token(r#"\/\*(\*(?!\/)|[^*])*\*\/"#, |lex| lex.slice())]
    // BlockComment(&'a str),
    #[token("interface")]
    Interface,
    #[token("handler")]
    Handler,
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

// #[derive(Default, Debug, Clone, PartialEq)]
// pub enum LexicalError {
//     #[default]
//     InvalidToken
// }

impl <'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Whitespace(s) => write!(f, "{}", s),
            Token::LineComment(s) => write!(f, "{}", s),
            // Token::BlockComment(s) => write!(f, "{}", s),
            Token::Interface => write!(f, "interface"),
            Token::Handler => write!(f, "handler"),
            Token::Ident(s) => write!(f, "{}", s),
            Token::String(s) => write!(f, "{}", s),
            Token::Numeric(s) => write!(f, "{}", s),
            Token::Semi => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Dot => write!(f, "."),
            Token::Equal => write!(f, "="),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::And => write!(f, "&"),
            Token::Or => write!(f, "|"),
            Token::Xor => write!(f, "^"),
            Token::Tilde => write!(f, "~"),
            Token::Less => write!(f, "<"),
            Token::Greater => write!(f, ">"),
            Token::EqualEqual => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::LessEqual => write!(f, "<="),
            Token::GreaterEqual => write!(f, ">="),
            Token::AndAnd => write!(f, "&&"),
            Token::OrOr => write!(f, "||"),
            Token::Not => write!(f, "!"),
            Token::Question => write!(f, "?"),
            Token::Error => write!(f, "<error>"),
        }
    }
}
//
// pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;
//
// pub struct Lexer<'a> {
//     token_stream: SpannedIter<'a, Token>,
// }
//
// impl<'input> Lexer<'input> {
//     pub fn new(input: &'input str) -> Self {
//         Self { token_stream: Token::lexer(input).spanned() }
//     }
// }
//
// impl<'input> Iterator for Lexer<'input> {
//     type Item = Spanned<Token, usize, LexicalError>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.token_stream
//             .next()
//             .map(|(token, span)| Ok((span.start, token?, span.end)))
//     }
// }