use crate::any::AnyMessage;
use crate::encoder::EncodeError;
use crate::enums::EnumType;
use crate::field::Field;
use crate::json::escape::escape_json;
use crate::list::ListEncodeVisitor;
use crate::structs::StructEncodeVisitor;
use crate::value::ValueCodec;
use allocator_api2::alloc::Allocator;
use base64::prelude::*;
use core::fmt::Write;
use ixc_message_api::AccountID;
use simple_time::{Duration, Time};
use crate::json::JSONCodec;
use crate::mem::MemoryManager;

impl JSONCodec<'_> {
    /// Encode the value to a JSON string.
    /// This method is intended to be deterministic and performant, so that it is suitable
    /// for signature verification.
    /// It avoids any intermediate allocations and simply writes its output to the provided buffer
    /// which can be configured with a custom allocator.
    pub fn encode_value<A: Allocator>(
        &self,
        value: &dyn ValueCodec,
        writer: &mut allocator_api2::vec::Vec<u8, A>,
    ) -> Result<(), EncodeError> {
        let mut encoder = Encoder {
            codec: self,
            writer: Writer(writer),
            memory_manager: MemoryManager::new(),
            num_nested_fields_written: 0,
        };
        value.encode(&mut encoder)?;
        Ok(())
    }
}

struct Encoder<'a, A: Allocator> {
    codec: &'a JSONCodec<'a>,
    writer: Writer<'a, A>,
    memory_manager: MemoryManager,
    // this is only used to avoid writing the field name if a nested object is empty
    num_nested_fields_written: usize,
}

pub(crate) struct Writer<'a, A: Allocator>(pub(crate) &'a mut allocator_api2::vec::Vec<u8, A>);

impl<A: Allocator> Write for Writer<'_, A> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.extend_from_slice(s.as_bytes());
        Ok(())
    }
}

// we override the write! macro to return a custom error type
macro_rules! write {
    ($writer:expr, $($arg:tt)*) => {
        $writer.write_fmt(format_args!($($arg)*)).map_err(|_| EncodeError::UnknownError)
    };
}

impl<A: Allocator> crate::encoder::Encoder for Encoder<'_, A> {
    fn encode_bool(&mut self, x: bool) -> Result<(), EncodeError> {
        write!(self.writer, "{}", x)
    }

    fn encode_u8(&mut self, x: u8) -> Result<(), EncodeError> {
        write!(self.writer, "{}", x)
    }

    fn encode_u16(&mut self, x: u16) -> Result<(), EncodeError> {
        write!(self.writer, "{}", x)
    }

    fn encode_u32(&mut self, x: u32) -> Result<(), EncodeError> {
        write!(self.writer, "{}", x)
    }

    fn encode_u64(&mut self, x: u64) -> Result<(), EncodeError> {
        write!(self.writer, "\"{}\"", x)
    }

    fn encode_u128(&mut self, x: u128) -> Result<(), EncodeError> {
        write!(self.writer, "\"{}\"", x)
    }

    fn encode_i8(&mut self, x: i8) -> Result<(), EncodeError> {
        write!(self.writer, "{}", x)
    }

    fn encode_i16(&mut self, x: i16) -> Result<(), EncodeError> {
        write!(self.writer, "{}", x)
    }

    fn encode_i32(&mut self, x: i32) -> Result<(), EncodeError> {
        write!(self.writer, "{}", x)
    }

    fn encode_i64(&mut self, x: i64) -> Result<(), EncodeError> {
        write!(self.writer, "\"{}\"", x)
    }

    fn encode_i128(&mut self, x: i128) -> Result<(), EncodeError> {
        write!(self.writer, "\"{}\"", x)
    }

    fn encode_str(&mut self, x: &str) -> Result<(), EncodeError> {
        escape_json(x, &mut self.writer).map_err(|_| EncodeError::UnknownError)
    }

    fn encode_bytes(&mut self, x: &[u8]) -> Result<(), EncodeError> {
        write!(self.writer, "\"")?;
        
        // calculate the number of bytes needed to encode the string
        let num_bytes_needed = x.len() * 4 / 3 + 4;
        // resize the buffer to accommodate the encoded string
        let cur_buf_len = self.writer.0.len();
        let proposed_buf_len = cur_buf_len + num_bytes_needed;
        self.writer.0.resize(proposed_buf_len, 0);
        // get a mutable slice of the buffer
        let mut writeable_buf = &mut self.writer.0[cur_buf_len..];
        let written = BASE64_STANDARD
            .encode_slice(x, &mut writeable_buf)
            .map_err(|_| EncodeError::UnknownError)?;
        // resize the buffer back to the correct size (original + encoded bytes)
        let real_new_buf_len = cur_buf_len + written;
        self.writer.0.truncate(real_new_buf_len);
        
        write!(self.writer, "\"")
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

    fn encode_struct_fields(
        &mut self,
        visitor: &dyn StructEncodeVisitor,
        fields: &[Field],
    ) -> Result<(), EncodeError> {
        write!(self.writer, "{{")?;
        let mut first = true;
        let mut pos;
        for (i, field) in fields.iter().enumerate() {
            pos = self.writer.0.len();
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
                self.writer.0.truncate(pos);
            } else {
                first = false;
                self.num_nested_fields_written += 1;
            }
        }
        write!(self.writer, "}}")
    }

    fn encode_option(&mut self, visitor: Option<&dyn ValueCodec>) -> Result<(), EncodeError> {
        if let Some(visitor) = visitor {
            visitor.encode(self)
        } else {
            write!(self.writer, "null")
        }
    }

    fn encode_account_id(&mut self, x: AccountID) -> Result<(), EncodeError> {
        write!(self.writer, "\"")?;
        self.codec.account_id_codec.encode_str(&x, &mut self.writer)?;
        write!(self.writer, "\"")?;
        Ok(())
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

    fn encode_time(&mut self, _x: Time) -> Result<(), EncodeError> {
        todo!()
    }

    fn encode_duration(&mut self, _x: Duration) -> Result<(), EncodeError> {
        todo!()
    }

    fn encode_any_message(&mut self, x: &AnyMessage) -> Result<(), EncodeError> {
        match x {
            AnyMessage::Empty => {
                write!(self.writer, "null")?;
            }
            AnyMessage::ExecMessage {
                account,
                selector,
                bytes,
            } => {
                let schema = self.codec.schema_resolver.schema_for_account(account, &self.memory_manager)
                    .ok_or(EncodeError::UnknownError)?;
                let type_map = &schema.type_map;
                let struct_type = type_map.lookup_type_by_selector(*selector)
                    .ok_or(EncodeError::UnknownError)?;
                write!(self.writer, "{{")?;
                write!(self.writer, "\"account\":")?;
                self.encode_account_id(*account)?;
                write!(self.writer, ",")?;
                write!(self.writer, "\"type\":\"{}\",", struct_type.name)?;
                todo!();
                // write!(self.writer, "\"value\":")?;
                // self.encode_struct_fields(&struct_type.fields, bytes.as_slice())?;
                write!(self.writer, "}}")?;
            }
            AnyMessage::CreateAccount {
                handler_id,
                init_data,
            } => {
                todo!()
            }
            AnyMessage::Migrate {
                account,
                new_handler_id,
            } => {
                todo!()
            }
        }
        Ok(())
    }
}

struct FieldEncoder<'a, 'b, A: Allocator> {
    outer: &'b mut Encoder<'a, A>,
    present: bool,
}

impl<A: Allocator> FieldEncoder<'_, '_, A> {
    fn mark_not_present(&mut self) -> Result<(), EncodeError> {
        self.present = false;
        Ok(())
    }
}

impl<A: Allocator> crate::encoder::Encoder for FieldEncoder<'_, '_, A> {
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

    fn encode_struct_fields(
        &mut self,
        visitor: &dyn StructEncodeVisitor,
        fields: &[Field],
    ) -> Result<(), EncodeError> {
        let cur_fields_written = self.outer.num_nested_fields_written;
        self.outer.encode_struct_fields(visitor, fields)?;
        // if we've written no fields, then we need to tell the parent writer to truncate the field name
        if self.outer.num_nested_fields_written == cur_fields_written {
            self.mark_not_present()?;
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

    fn encode_time(&mut self, _x: Time) -> Result<(), EncodeError> {
        todo!()
    }

    fn encode_duration(&mut self, _x: Duration) -> Result<(), EncodeError> {
        todo!()
    }

    fn encode_any_message(&mut self, x: &AnyMessage) -> Result<(), EncodeError> {
        todo!()
    }
}
