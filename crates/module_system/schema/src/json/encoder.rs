#![allow(unused)]
extern crate std;

use crate::encoder::EncodeError;
use crate::enums::EnumType;
use crate::list::ListEncodeVisitor;
use crate::structs::{StructEncodeVisitor, StructType};
use crate::value::ValueCodec;
use alloc::string::String;
use alloc::vec::Vec;
use base64::prelude::*;
use core::ops::Index;
use ixc_message_api::AccountID;
use simple_time::{Duration, Time};
use std::io::Write;

/// Encode the value to a JSON string.
pub fn encode_value<'a>(value: &dyn ValueCodec) -> Result<String, EncodeError> {
    let mut writer = Vec::new();
    let mut encoder = Encoder {
        writer,
        num_nested_fields_written: 0,
    };
    value.encode(&mut encoder)?;
    Ok(String::from_utf8(encoder.writer).map_err(|_| EncodeError::UnknownError)?)
}

struct Encoder {
    writer: Vec<u8>,
    // this is only used to avoid writing the field name if a nested object is empty
    num_nested_fields_written: usize,
}

macro_rules! write {
    ($writer:expr, $($arg:tt)*) => {
        $writer.write_fmt(format_args!($($arg)*)).map_err(|_| EncodeError::UnknownError)
    };
}

impl crate::encoder::Encoder for Encoder {
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
        let escaped = escape8259::escape(x);
        write!(self.writer, "\"{}\"", escaped)
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
        let mut pos = self.writer.len();
        for (i, field) in struct_type.fields.iter().enumerate() {
            pos = self.writer.len();
            if !first {
                write!(self.writer, ",")?;
            }
            write!(self.writer, "\"{}\":", field.name)?;
            let mut inner = FieldEncoder {
                outer: self,
                present: true,
            };
            visitor.encode_field(i, &mut inner)?;
            if !inner.present {
                self.writer.truncate(pos);
            } else {
                first = false;
                self.num_nested_fields_written += 1;
            }
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
        write!(self.writer, "\"{}\"", x)
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

    fn encode_enum_variant(
        &mut self,
        discriminant: i32,
        enum_type: &EnumType,
        value: Option<&dyn ValueCodec>,
    ) -> Result<(), EncodeError> {
        let variant = enum_type
            .variants
            .iter()
            .find(|v| v.discriminant == discriminant)
            .ok_or(EncodeError::UnknownError)?;
        if let Some(value) = value {
            write!(self.writer, "{{\"type\":\"{}\",\"value\":", variant.name)?;
            value.encode(self)?;
            write!(self.writer, "}}")
        } else {
            write!(self.writer, "\"{}\"", variant.name)
        }
    }
}

struct FieldEncoder<'a> {
    outer: &'a mut Encoder,
    present: bool,
}

impl FieldEncoder<'_> {
    fn mark_not_present(&mut self) -> Result<(), EncodeError> {
        self.present = false;
        Ok(())
    }
}

impl crate::encoder::Encoder for FieldEncoder<'_> {
    fn encode_bool(&mut self, x: bool) -> Result<(), EncodeError> {
        if !x {
            return self.mark_not_present();
        }
        self.outer.encode_bool(x)
    }

    fn encode_u8(&mut self, x: u8) -> Result<(), EncodeError> {
        if x == 0 {
            return self.mark_not_present();
        }
        self.outer.encode_u8(x)
    }

    fn encode_u16(&mut self, x: u16) -> Result<(), EncodeError> {
        if x == 0 {
            return self.mark_not_present();
        }
        self.outer.encode_u16(x)
    }

    fn encode_u32(&mut self, x: u32) -> Result<(), EncodeError> {
        if x == 0 {
            return self.mark_not_present();
        }
        self.outer.encode_u32(x)
    }

    fn encode_u64(&mut self, x: u64) -> Result<(), EncodeError> {
        if x == 0 {
            return self.mark_not_present();
        }
        self.outer.encode_u64(x)
    }

    fn encode_u128(&mut self, x: u128) -> Result<(), EncodeError> {
        if x == 0 {
            return self.mark_not_present();
        }
        self.outer.encode_u128(x)
    }

    fn encode_i8(&mut self, x: i8) -> Result<(), EncodeError> {
        if x == 0 {
            return self.mark_not_present();
        }
        self.outer.encode_i8(x)
    }

    fn encode_i16(&mut self, x: i16) -> Result<(), EncodeError> {
        if x == 0 {
            return self.mark_not_present();
        }
        self.outer.encode_i16(x)
    }

    fn encode_i32(&mut self, x: i32) -> Result<(), EncodeError> {
        if x == 0 {
            return self.mark_not_present();
        }
        self.outer.encode_i32(x)
    }

    fn encode_i64(&mut self, x: i64) -> Result<(), EncodeError> {
        if x == 0 {
            return self.mark_not_present();
        }
        self.outer.encode_i64(x)
    }

    fn encode_i128(&mut self, x: i128) -> Result<(), EncodeError> {
        if x == 0 {
            return self.mark_not_present();
        }
        self.outer.encode_i128(x)
    }

    fn encode_str(&mut self, x: &str) -> Result<(), EncodeError> {
        if x.is_empty() {
            return self.mark_not_present();
        }
        self.outer.encode_str(x)
    }

    fn encode_bytes(&mut self, x: &[u8]) -> Result<(), EncodeError> {
        if x.is_empty() {
            return self.mark_not_present();
        }
        self.outer.encode_bytes(x)
    }

    fn encode_list(&mut self, visitor: &dyn ListEncodeVisitor) -> Result<(), EncodeError> {
        if visitor.size() == 0 {
            return self.mark_not_present();
        }
        self.outer.encode_list(visitor)
    }

    fn encode_struct(
        &mut self,
        visitor: &dyn StructEncodeVisitor,
        struct_type: &StructType,
    ) -> Result<(), EncodeError> {
        let cur_fields_written = self.outer.num_nested_fields_written;
        self.outer.encode_struct(visitor, struct_type)?;
        // if we've written no fields, then we need to tell the parent writer to truncate the field name
        if self.outer.num_nested_fields_written == cur_fields_written {
            self.mark_not_present();
        }
        Ok(())
    }

    fn encode_option(&mut self, visitor: Option<&dyn ValueCodec>) -> Result<(), EncodeError> {
        if visitor.is_none() {
            return self.mark_not_present();
        }
        self.outer.encode_option(visitor)
    }

    fn encode_account_id(&mut self, x: AccountID) -> Result<(), EncodeError> {
        if x.is_empty() {
            return self.mark_not_present();
        }
        self.outer.encode_account_id(x)
    }

    fn encode_enum_variant(
        &mut self,
        discriminant: i32,
        enum_type: &EnumType,
        value: Option<&dyn ValueCodec>,
    ) -> Result<(), EncodeError> {
        if discriminant == 0 && value.is_none() {
            return self.mark_not_present();
        }
        self.outer
            .encode_enum_variant(discriminant, enum_type, value)
    }

    fn encode_time(&mut self, x: Time) -> Result<(), EncodeError> {
        todo!()
    }

    fn encode_duration(&mut self, x: Duration) -> Result<(), EncodeError> {
        todo!()
    }
}
