#![allow(unused)]
extern crate std;

use crate::encoder::EncodeError;
use crate::list::ListEncodeVisitor;
use crate::structs::{StructEncodeVisitor, StructType};
use crate::value::ValueCodec;
use base64::prelude::*;
use ixc_message_api::AccountID;
use simple_time::{Duration, Time};
use std::io::Write;
use crate::enums::EnumType;

pub fn encode_value<'a>(
    value: &dyn ValueCodec,
    writer: &'a mut dyn Write,
) -> Result<(), EncodeError> {
    let mut encoder = Encoder { writer };
    value.encode(&mut encoder)
}

struct Encoder<'a> {
    writer: &'a mut dyn Write,
}

macro_rules! write {
    ($writer:expr, $($arg:tt)*) => {
        $writer.write_fmt(format_args!($($arg)*)).map_err(|_| EncodeError::UnknownError)
    };
}

impl crate::encoder::Encoder for Encoder<'_> {
    fn encode_u32(&mut self, x: u32) -> Result<(), EncodeError> {
        write!(self.writer, "{}", x)
    }

    fn encode_i32(&mut self, x: i32) -> Result<(), EncodeError> {
        write!(self.writer, "{}", x)
    }

    fn encode_u64(&mut self, x: u64) -> Result<(), EncodeError> {
        write!(self.writer, "\"{}\"", x)
    }

    fn encode_u128(&mut self, x: u128) -> Result<(), EncodeError> {
        write!(self.writer, "\"{}\"", x)
    }

    fn encode_str(&mut self, x: &str) -> Result<(), EncodeError> {
        // TODO escape the string
        write!(self.writer, "\"{}\"", x)
    }

    fn encode_list(&mut self, visitor: &dyn ListEncodeVisitor) -> Result<(), EncodeError> {
        write!(self.writer, "[")?;
        let size = visitor.size();
        for i in 0..size {
            visitor.encode(i, self)?;
            if i < size - 1 {
                write!(self.writer, ",")?;
            }
        }
        write!(self.writer, "]")
    }

    fn encode_struct(
        &mut self,
        visitor: &dyn StructEncodeVisitor,
        struct_type: &StructType,
    ) -> Result<(), EncodeError> {
        write!(self.writer, "{{")?;
        let mut first = true;
        for (i, field) in struct_type.fields.iter().enumerate() {
            if !first {
                write!(self.writer, ",")?;
            } else {
                first = false;
            }
            write!(self.writer, "\"{}\":", field.name)?;
            visitor.encode_field(i, self)?;
        }
        write!(self.writer, "}}")
    }

    fn encode_account_id(&mut self, x: AccountID) -> Result<(), EncodeError> {
        let id: u128 = x.into();
        self.encode_u128(id)
    }

    fn encode_bool(&mut self, x: bool) -> Result<(), EncodeError> {
        write!(self.writer, "{}", x)
    }

    fn encode_u8(&mut self, x: u8) -> Result<(), EncodeError> {
        write!(self.writer, "{}", x)
    }

    fn encode_u16(&mut self, x: u16) -> Result<(), EncodeError> {
        write!(self.writer, "{}", x)
    }

    fn encode_i8(&mut self, x: i8) -> Result<(), EncodeError> {
        write!(self.writer, "{}", x)
    }

    fn encode_i16(&mut self, x: i16) -> Result<(), EncodeError> {
        write!(self.writer, "{}", x)
    }

    fn encode_i64(&mut self, x: i64) -> Result<(), EncodeError> {
        write!(self.writer, "\"{}\"", x)
    }

    fn encode_i128(&mut self, x: i128) -> Result<(), EncodeError> {
        write!(self.writer, "{}", x)
    }

    fn encode_bytes(&mut self, x: &[u8]) -> Result<(), EncodeError> {
        write!(self.writer, "\"{}\"", BASE64_STANDARD.encode(x))
    }

    fn encode_time(&mut self, x: Time) -> Result<(), EncodeError> {
        todo!()
    }

    fn encode_duration(&mut self, x: Duration) -> Result<(), EncodeError> {
        todo!()
    }

    fn encode_option(&mut self, visitor: Option<&dyn ValueCodec>) -> Result<(), EncodeError> {
        if let Some(visitor) = visitor {
            visitor.encode(self)
        } else {
            write!(self.writer, "null")
        }
    }

    fn encode_enum_discriminant(&mut self, x: i32, enum_type: &EnumType) -> Result<(), EncodeError> {
        let variant = enum_type.variants.get(x as usize)
            .ok_or(EncodeError::UnknownError)?;
        write!(self.writer, "\"{}\"", variant.name)
    }
}
