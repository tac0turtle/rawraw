use alloc::string::String;
use alloc::vec::Vec;
use core::str::FromStr;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use ixc_message_api::AccountID;
use simple_time::{Duration, Time};
use crate::decoder::DecodeError;
use crate::list::ListDecodeVisitor;
use crate::mem::MemoryManager;
use crate::structs::{StructDecodeVisitor, StructType};
use crate::value::ValueCodec;
use logos::{Logos, Lexer};
use ixc_message_api::alloc_util::copy_bytes;

struct Decoder<'a> {
    tokens: &'a mut Lexer<'a, Token<'a>>,
    mem: &'a MemoryManager,
}

#[derive(Debug, Logos)]
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
    fn expect_token(&mut self) -> Result<Token<'a>, DecodeError> {
        if let Some(token) = self.tokens.next() {
            token.map_err(|_| DecodeError::InvalidData)
        } else {
            Err(DecodeError::OutOfData)
        }
    }

    fn expect_number<I: FromStr>(&mut self) -> Result<I, DecodeError> {
        match self.expect_token()? {
            Token::Number(n) => Ok(I::from_str(n).map_err(|_| DecodeError::InvalidData)?),
            _ => Err(DecodeError::InvalidData),
        }
    }

    fn expect_str(&mut self) -> Result<&'a str, DecodeError> {
        match self.expect_token()? {
            Token::String(s) => Ok(s),
            _ => Err(DecodeError::InvalidData),
        }
    }
}

impl<'a> crate::decoder::Decoder<'a> for Decoder<'a> {
    fn decode_bool(&mut self) -> Result<bool, DecodeError> {
        match self.expect_token()? {
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
        Ok(u64::from_str(s).map_err(|_| DecodeError::InvalidData)?)
    }

    fn decode_u128(&mut self) -> Result<u128, DecodeError> {
        let s = self.expect_str()?;
        Ok(u128::from_str(s).map_err(|_| DecodeError::InvalidData)?)
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
        Ok(i64::from_str(s).map_err(|_| DecodeError::InvalidData)?)

    }

    fn decode_i128(&mut self) -> Result<i128, DecodeError> {
        let s = self.expect_str()?;
        Ok(i128::from_str(s).map_err(|_| DecodeError::InvalidData)?)
    }

    fn decode_borrowed_str(&mut self) -> Result<&'a str, DecodeError> {
        // TODO escape characters
        self.expect_str()
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
        BASE64_STANDARD.decode(s).map_err(|_| DecodeError::InvalidData)
    }

    fn decode_struct(&mut self, visitor: &mut dyn StructDecodeVisitor<'a>, struct_type: &StructType) -> Result<(), DecodeError> {
        todo!()
    }

    fn decode_list(&mut self, visitor: &mut dyn ListDecodeVisitor<'a>) -> Result<(), DecodeError> {
        todo!()
    }

    fn decode_option(&mut self, visitor: &mut dyn ValueCodec<'a>) -> Result<bool, DecodeError> {
        todo!()
    }

    fn decode_account_id(&mut self) -> Result<AccountID, DecodeError> {
        todo!()
    }

    fn decode_time(&mut self) -> Result<Time, DecodeError> {
        todo!()
    }

    fn decode_duration(&mut self) -> Result<Duration, DecodeError> {
        todo!()
    }

    fn mem_manager(&self) -> &'a MemoryManager {
        self.mem
    }
}