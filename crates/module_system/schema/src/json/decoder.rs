use crate::decoder::DecodeError;
use crate::list::ListDecodeVisitor;
use crate::mem::MemoryManager;
use crate::structs::{StructDecodeVisitor, StructType};
use crate::value::ValueCodec;
use alloc::string::String;
use alloc::vec::Vec;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use core::iter::Peekable;
use core::str::FromStr;
use ixc_message_api::alloc_util::{copy_bytes, copy_str};
use ixc_message_api::AccountID;
use logos::{Lexer, Logos};
use simple_time::{Duration, Time};
use crate::enums::EnumType;

pub fn decode_value<'a, V: ValueCodec<'a> + Default>(
    input: &'a str,
    memory_manager: &'a MemoryManager,
) -> Result<V, DecodeError> {
    let mut decoder = Decoder {
        tokens: Lexer::new(input).peekable(),
        mem: memory_manager,
    };
    let mut res = V::default();
    res.decode(&mut decoder)?;
    Ok(res)
}

struct Decoder<'a> {
    tokens: Peekable<Lexer<'a, Token<'a>>>,
    mem: &'a MemoryManager,
}

#[derive(Debug, Logos, PartialEq, Eq, Clone)]
#[logos(skip r"[ \t\r\n\f]+")]
enum Token<'source> {
    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Bool(bool),

    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[token("[")]
    BracketOpen,

    #[token("]")]
    BracketClose,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token("null")]
    Null,

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice())]
    Number(&'source str),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice())]
    String(&'source str),
}

impl<'a> Decoder<'a> {
    fn next_token(&mut self) -> Result<Token<'a>, DecodeError> {
        if let Some(token) = self.tokens.next() {
            token.map_err(|_| DecodeError::InvalidData)
        } else {
            Err(DecodeError::OutOfData)
        }
    }

    fn peek_token(&mut self) -> Result<Token<'a>, DecodeError> {
        let token = self.tokens.peek().ok_or(DecodeError::OutOfData)?.clone();
        let token = token.map_err(|_| DecodeError::InvalidData)?;
        Ok(token)
    }

    fn expect_number<I: FromStr>(&mut self) -> Result<I, DecodeError> {
        match self.next_token()? {
            Token::Number(n) => Ok(I::from_str(n).map_err(|_| DecodeError::InvalidData)?),
            _ => Err(DecodeError::InvalidData),
        }
    }

    fn expect_str(&mut self) -> Result<String, DecodeError> {
        match self.next_token()? {
            Token::String(s) => {
                let s = &s[1..s.len() - 1];
              escape8259::unescape(s).map_err(|_| DecodeError::InvalidData)
            },
            _ => Err(DecodeError::InvalidData),
        }
    }
}

impl<'a> crate::decoder::Decoder<'a> for Decoder<'a> {
    fn decode_bool(&mut self) -> Result<bool, DecodeError> {
        match self.next_token()? {
            Token::Bool(tf) => Ok(tf),
            _ => Err(DecodeError::InvalidData),
        }
    }

    fn decode_u8(&mut self) -> Result<u8, DecodeError> {
        self.expect_number()
    }

    fn decode_u16(&mut self) -> Result<u16, DecodeError> {
        self.expect_number()
    }

    fn decode_u32(&mut self) -> Result<u32, DecodeError> {
        self.expect_number()
    }

    fn decode_u64(&mut self) -> Result<u64, DecodeError> {
        let s = self.expect_str()?;
        Ok(u64::from_str(&s).map_err(|_| DecodeError::InvalidData)?)
    }

    fn decode_u128(&mut self) -> Result<u128, DecodeError> {
        let s = self.expect_str()?;
        Ok(u128::from_str(&s).map_err(|_| DecodeError::InvalidData)?)
    }

    fn decode_i8(&mut self) -> Result<i8, DecodeError> {
        self.expect_number()
    }

    fn decode_i16(&mut self) -> Result<i16, DecodeError> {
        self.expect_number()
    }

    fn decode_i32(&mut self) -> Result<i32, DecodeError> {
        self.expect_number()
    }

    fn decode_i64(&mut self) -> Result<i64, DecodeError> {
        let s = self.expect_str()?;
        Ok(i64::from_str(&s).map_err(|_| DecodeError::InvalidData)?)
    }

    fn decode_i128(&mut self) -> Result<i128, DecodeError> {
        let s = self.expect_str()?;
        Ok(i128::from_str(&s).map_err(|_| DecodeError::InvalidData)?)
    }

    fn decode_borrowed_str(&mut self) -> Result<&'a str, DecodeError> {
        let s = self.expect_str()?;
        unsafe { copy_str(self.mem, &s).map_err(|_| DecodeError::InvalidData) }
    }

    fn decode_owned_str(&mut self) -> Result<String, DecodeError> {
        Ok(self.expect_str().map(|s| s.into())?)
    }

    fn decode_borrowed_bytes(&mut self) -> Result<&'a [u8], DecodeError> {
        let bz = self.decode_owned_bytes()?;
        unsafe { copy_bytes(self.mem, bz.as_slice()).map_err(|_| DecodeError::InvalidData) }
    }

    fn decode_owned_bytes(&mut self) -> Result<Vec<u8>, DecodeError> {
        let s = self.expect_str()?;
        BASE64_STANDARD
            .decode(s)
            .map_err(|_| DecodeError::InvalidData)
    }

    fn decode_struct(
        &mut self,
        visitor: &mut dyn StructDecodeVisitor<'a>,
        struct_type: &StructType,
    ) -> Result<(), DecodeError> {
        let start = self.next_token()?;
        if start != Token::BraceOpen {
            return Err(DecodeError::InvalidData);
        }
        if let Token::BraceClose = self.peek_token()? {
            self.tokens.next();
            return Ok(());
        }
        loop {
            let field_name = self.expect_str()?;
            let idx = struct_type.fields.iter().position(|f| f.name == field_name)
                .ok_or(DecodeError::InvalidData)?;

            if Token::Colon != self.next_token()? {
                return Err(DecodeError::InvalidData);
            }

            visitor.decode_field(idx, self)?;

            let peek = self.peek_token()?;
            if Token::Comma == peek {
                self.tokens.next();
            } else if Token::BraceClose == peek {
                self.tokens.next();
                return Ok(());
            } else {
                return Err(DecodeError::InvalidData);
            }
        }
    }

    fn decode_list(&mut self, visitor: &mut dyn ListDecodeVisitor<'a>) -> Result<(), DecodeError> {
        let start = self.next_token()?;
        if start != Token::BracketOpen {
            return Err(DecodeError::InvalidData);
        }

        if Token::BracketClose == self.peek_token()? {
            self.tokens.next();
            return Ok(());
        }

        loop {
            visitor.next(self)?;
            let peek = self.peek_token()?;
            if Token::Comma == peek {
                self.tokens.next();
            } else if Token::BracketClose == peek {
                self.tokens.next();
                return Ok(());
            } else {
                return Err(DecodeError::InvalidData);
            }
        }
    }

    fn decode_option(&mut self, visitor: &mut dyn ValueCodec<'a>) -> Result<bool, DecodeError> {
        if Token::Null == self.peek_token()? {
            self.tokens.next();
            Ok(false)
        } else {
            visitor.decode(self)?;
            Ok(true)
        }
    }

    fn decode_account_id(&mut self) -> Result<AccountID, DecodeError> {
        Ok(AccountID::new(self.decode_u128()?))
    }

    fn decode_time(&mut self) -> Result<Time, DecodeError> {
        todo!()
    }

    fn decode_duration(&mut self) -> Result<Duration, DecodeError> {
        todo!()
    }

    fn decode_enum_discriminant(&mut self, enum_type: &EnumType) -> Result<i32, DecodeError> {
        let s = self.expect_str()?;
        let variant = enum_type.variants.iter().find(|v| v.name == s)
            .ok_or(DecodeError::InvalidData)?;
        Ok(variant.discriminant)
    }

    fn mem_manager(&self) -> &'a MemoryManager {
        self.mem
    }
}
