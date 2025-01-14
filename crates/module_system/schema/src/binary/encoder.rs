#![allow(unused)]

use crate::buffer::{Writer, WriterFactory};
use crate::encoder::EncodeError;
use crate::enums::EnumType;
use crate::list::ListEncodeVisitor;
use crate::structs::{StructEncodeVisitor, StructType};
use crate::value::SchemaValue;
use crate::value::ValueCodec;
use allocator_api2::alloc::Allocator;
use ixc_message_api::AccountID;
use simple_time::{Duration, Time};

pub fn encode_value<'a>(
    value: &dyn ValueCodec,
    writer_factory: &'a dyn Allocator,
) -> Result<&'a [u8], EncodeError> {
    let mut sizer = EncodeSizer { size: 0 };
    value.encode(&mut sizer)?;
    let mut writer = writer_factory.new_reverse(sizer.size)?;
    let mut encoder = Encoder {
        writer: &mut writer,
    };
    value.encode(&mut encoder)?;
    Ok(writer.finish())
}

pub(crate) struct Encoder<'a, W> {
    pub(crate) writer: &'a mut W,
}

impl<W: Writer> crate::encoder::Encoder for Encoder<'_, W> {
    fn encode_u32(&mut self, x: u32) -> Result<(), EncodeError> {
        self.writer.write(&x.to_le_bytes())
    }

    fn encode_i32(&mut self, x: i32) -> Result<(), EncodeError> {
        self.writer.write(&x.to_le_bytes())
    }

    fn encode_u64(&mut self, x: u64) -> Result<(), EncodeError> {
        self.writer.write(&x.to_le_bytes())
    }

    fn encode_u128(&mut self, x: u128) -> Result<(), EncodeError> {
        self.writer.write(&x.to_le_bytes())
    }

    fn encode_str(&mut self, x: &str) -> Result<(), EncodeError> {
        self.writer.write(x.as_bytes())
    }

    fn encode_list(&mut self, visitor: &dyn ListEncodeVisitor) -> Result<(), EncodeError> {
        let mut sub = Encoder {
            writer: self.writer,
        };
        let mut inner = InnerEncoder::<W> { outer: &mut sub };
        let size = visitor.size();
        for i in 0..size {
            visitor.encode(size - i - 1, &mut inner)?;
        }
        self.encode_u32(size as u32)?;
        Ok(())
    }

    fn encode_struct(
        &mut self,
        visitor: &dyn StructEncodeVisitor,
        struct_type: &StructType,
    ) -> Result<(), EncodeError> {
        let mut i = struct_type.fields.len();
        let mut sub = Encoder {
            writer: self.writer,
        };
        let mut inner = InnerEncoder::<W> { outer: &mut sub };
        for f in struct_type.fields.iter().rev() {
            i -= 1;
            visitor.encode_field(i, &mut inner)?;
        }
        Ok(())
    }

    fn encode_account_id(&mut self, x: AccountID) -> Result<(), EncodeError> {
        self.encode_u128(x.into())
    }

    fn encode_bool(&mut self, x: bool) -> Result<(), EncodeError> {
        self.encode_u8(if x { 1 } else { 0 })
    }

    fn encode_u8(&mut self, x: u8) -> Result<(), EncodeError> {
        self.writer.write(&[x])
    }

    fn encode_u16(&mut self, x: u16) -> Result<(), EncodeError> {
        self.writer.write(&x.to_le_bytes())
    }

    fn encode_i8(&mut self, x: i8) -> Result<(), EncodeError> {
        self.writer.write(&[x as u8])
    }

    fn encode_i16(&mut self, x: i16) -> Result<(), EncodeError> {
        self.writer.write(&x.to_le_bytes())
    }

    fn encode_i64(&mut self, x: i64) -> Result<(), EncodeError> {
        self.writer.write(&x.to_le_bytes())
    }

    fn encode_i128(&mut self, x: i128) -> Result<(), EncodeError> {
        self.writer.write(&x.to_le_bytes())
    }

    fn encode_bytes(&mut self, x: &[u8]) -> Result<(), EncodeError> {
        self.writer.write(x)
    }

    fn encode_time(&mut self, x: Time) -> Result<(), EncodeError> {
        /// TODO find a more efficient way to encode time
        self.encode_i128(x.unix_nanos())
    }

    fn encode_duration(&mut self, x: Duration) -> Result<(), EncodeError> {
        /// TODO find a more efficient way to encode duration
        self.encode_i128(x.nanos())
    }

    fn encode_option(&mut self, visitor: Option<&dyn ValueCodec>) -> Result<(), EncodeError> {
        if let Some(visitor) = visitor {
            visitor.encode(self)
        } else {
            Ok(())
        }
    }

    fn encode_enum_variant(
        &mut self,
        discriminant: i32,
        enum_type: &EnumType,
        value: Option<&dyn ValueCodec>,
    ) -> Result<(), EncodeError> {
        if let Some(value) = value {
            value.encode(self)?;
        }
        self.encode_i32(discriminant)
    }
}

pub(crate) struct EncodeSizer {
    pub(crate) size: usize,
}

impl crate::encoder::Encoder for EncodeSizer {
    fn encode_u32(&mut self, x: u32) -> Result<(), EncodeError> {
        self.size += 4;
        Ok(())
    }

    fn encode_i32(&mut self, x: i32) -> Result<(), EncodeError> {
        self.size += 4;
        Ok(())
    }

    fn encode_u64(&mut self, x: u64) -> Result<(), EncodeError> {
        self.size += 8;
        Ok(())
    }

    fn encode_u128(&mut self, x: u128) -> Result<(), EncodeError> {
        self.size += 16;
        Ok(())
    }

    fn encode_str(&mut self, x: &str) -> Result<(), EncodeError> {
        self.size += x.len();
        Ok(())
    }

    fn encode_list(&mut self, visitor: &dyn ListEncodeVisitor) -> Result<(), EncodeError> {
        self.size += 4;
        for i in 0..visitor.size() {
            visitor.encode(i, &mut InnerEncodeSizer { outer: self })?;
        }
        Ok(())
    }

    fn encode_struct(
        &mut self,
        visitor: &dyn StructEncodeVisitor,
        struct_type: &StructType,
    ) -> Result<(), EncodeError> {
        let mut sub = InnerEncodeSizer { outer: self };
        for (i, f) in struct_type.fields.iter().enumerate() {
            visitor.encode_field(i, &mut sub)?;
        }
        Ok(())
    }

    fn encode_account_id(&mut self, x: AccountID) -> Result<(), EncodeError> {
        self.size += 16;
        Ok(())
    }

    fn encode_bool(&mut self, x: bool) -> Result<(), EncodeError> {
        self.size += 1;
        Ok(())
    }

    fn encode_u8(&mut self, x: u8) -> Result<(), EncodeError> {
        self.size += 1;
        Ok(())
    }

    fn encode_u16(&mut self, x: u16) -> Result<(), EncodeError> {
        self.size += 2;
        Ok(())
    }

    fn encode_i8(&mut self, x: i8) -> Result<(), EncodeError> {
        self.size += 1;
        Ok(())
    }

    fn encode_i16(&mut self, x: i16) -> Result<(), EncodeError> {
        self.size += 2;
        Ok(())
    }

    fn encode_i64(&mut self, x: i64) -> Result<(), EncodeError> {
        self.size += 8;
        Ok(())
    }

    fn encode_i128(&mut self, x: i128) -> Result<(), EncodeError> {
        self.size += 16;
        Ok(())
    }

    fn encode_bytes(&mut self, x: &[u8]) -> Result<(), EncodeError> {
        self.size += x.len();
        Ok(())
    }

    fn encode_time(&mut self, x: Time) -> Result<(), EncodeError> {
        self.encode_i128(x.unix_nanos())
    }

    fn encode_duration(&mut self, x: Duration) -> Result<(), EncodeError> {
        self.encode_i128(x.nanos())
    }

    fn encode_option(&mut self, visitor: Option<&dyn ValueCodec>) -> Result<(), EncodeError> {
        if let Some(visitor) = visitor {
            visitor.encode(self)
        } else {
            Ok(())
        }
    }

    fn encode_enum_variant(
        &mut self,
        discriminant: i32,
        enum_type: &EnumType,
        value: Option<&dyn ValueCodec>,
    ) -> Result<(), EncodeError> {
        if let Some(value) = value {
            value.encode(self)?;
        }
        self.encode_i32(discriminant)
    }
}

pub(crate) struct InnerEncoder<'b, 'a: 'b, W> {
    pub(crate) outer: &'b mut Encoder<'a, W>,
}

impl<'b, 'a: 'b, W: Writer> crate::encoder::Encoder for InnerEncoder<'a, 'b, W> {
    fn encode_u32(&mut self, x: u32) -> Result<(), EncodeError> {
        self.outer.encode_u32(x)
    }

    fn encode_i32(&mut self, x: i32) -> Result<(), EncodeError> {
        self.outer.encode_i32(x)
    }

    fn encode_u64(&mut self, x: u64) -> Result<(), EncodeError> {
        self.outer.encode_u64(x)
    }

    fn encode_u128(&mut self, x: u128) -> Result<(), EncodeError> {
        self.outer.encode_u128(x)
    }

    fn encode_str(&mut self, x: &str) -> Result<(), EncodeError> {
        self.outer.encode_str(x)?;
        self.encode_u32(x.len() as u32)
    }

    fn encode_list(&mut self, visitor: &dyn ListEncodeVisitor) -> Result<(), EncodeError> {
        let end_pos = self.outer.writer.pos(); // this is a reverse writer so we start at the end
        self.outer.encode_list(visitor)?;
        let start_pos = self.outer.writer.pos(); // now we know the start position
        let size = (end_pos - start_pos) as u32;
        self.outer.encode_u32(size)
    }

    fn encode_struct(
        &mut self,
        visitor: &dyn StructEncodeVisitor,
        struct_type: &StructType,
    ) -> Result<(), EncodeError> {
        let end_pos = self.outer.writer.pos(); // this is a reverse writer so we start at the end
        self.outer.encode_struct(visitor, struct_type)?;
        let start_pos = self.outer.writer.pos(); // now we know the start position
        let size = (end_pos - start_pos) as u32;
        self.outer.encode_u32(size)
    }

    fn encode_account_id(&mut self, x: AccountID) -> Result<(), EncodeError> {
        self.outer.encode_account_id(x)
    }

    fn encode_bool(&mut self, x: bool) -> Result<(), EncodeError> {
        self.outer.encode_bool(x)
    }

    fn encode_u8(&mut self, x: u8) -> Result<(), EncodeError> {
        self.outer.encode_u8(x)
    }

    fn encode_u16(&mut self, x: u16) -> Result<(), EncodeError> {
        self.outer.encode_u16(x)
    }

    fn encode_i8(&mut self, x: i8) -> Result<(), EncodeError> {
        self.outer.encode_i8(x)
    }

    fn encode_i16(&mut self, x: i16) -> Result<(), EncodeError> {
        self.outer.encode_i16(x)
    }

    fn encode_i64(&mut self, x: i64) -> Result<(), EncodeError> {
        self.outer.encode_i64(x)
    }

    fn encode_i128(&mut self, x: i128) -> Result<(), EncodeError> {
        self.outer.encode_i128(x)
    }

    fn encode_bytes(&mut self, x: &[u8]) -> Result<(), EncodeError> {
        self.outer.encode_bytes(x)?;
        self.encode_u32(x.len() as u32)
    }

    fn encode_time(&mut self, x: Time) -> Result<(), EncodeError> {
        self.outer.encode_time(x)
    }

    fn encode_duration(&mut self, x: Duration) -> Result<(), EncodeError> {
        self.outer.encode_duration(x)
    }

    fn encode_option(&mut self, visitor: Option<&dyn ValueCodec>) -> Result<(), EncodeError> {
        if let Some(visitor) = visitor {
            visitor.encode(self)?;
            self.encode_bool(true)?;
        } else {
            self.encode_bool(false)?;
        }
        Ok(())
    }

    fn encode_enum_variant(
        &mut self,
        discriminant: i32,
        enum_type: &EnumType,
        value: Option<&dyn ValueCodec>,
    ) -> Result<(), EncodeError> {
        self.outer
            .encode_enum_variant(discriminant, enum_type, value)
    }
}

pub(crate) struct InnerEncodeSizer<'a> {
    pub(crate) outer: &'a mut EncodeSizer,
}

impl crate::encoder::Encoder for InnerEncodeSizer<'_> {
    fn encode_u32(&mut self, x: u32) -> Result<(), EncodeError> {
        self.outer.size += 4;
        Ok(())
    }

    fn encode_i32(&mut self, x: i32) -> Result<(), EncodeError> {
        self.outer.size += 4;
        Ok(())
    }

    fn encode_u64(&mut self, x: u64) -> Result<(), EncodeError> {
        self.outer.size += 8;
        Ok(())
    }

    fn encode_u128(&mut self, x: u128) -> Result<(), EncodeError> {
        self.outer.encode_u128(x)
    }

    fn encode_str(&mut self, x: &str) -> Result<(), EncodeError> {
        self.outer.size += 4;
        self.outer.encode_str(x)
    }

    fn encode_list(&mut self, visitor: &dyn ListEncodeVisitor) -> Result<(), EncodeError> {
        self.outer.size += 4; // for the for bytes size
        self.outer.encode_list(visitor)
    }

    fn encode_struct(
        &mut self,
        visitor: &dyn StructEncodeVisitor,
        struct_type: &StructType,
    ) -> Result<(), EncodeError> {
        self.outer.size += 4;
        self.outer.encode_struct(visitor, struct_type)
    }

    fn encode_account_id(&mut self, x: AccountID) -> Result<(), EncodeError> {
        self.outer.size += 16;
        Ok(())
    }

    fn encode_bool(&mut self, x: bool) -> Result<(), EncodeError> {
        self.outer.size += 1;
        Ok(())
    }

    fn encode_u8(&mut self, x: u8) -> Result<(), EncodeError> {
        self.outer.size += 1;
        Ok(())
    }

    fn encode_u16(&mut self, x: u16) -> Result<(), EncodeError> {
        self.outer.size += 2;
        Ok(())
    }

    fn encode_i8(&mut self, x: i8) -> Result<(), EncodeError> {
        self.outer.size += 1;
        Ok(())
    }

    fn encode_i16(&mut self, x: i16) -> Result<(), EncodeError> {
        self.outer.size += 2;
        Ok(())
    }

    fn encode_i64(&mut self, x: i64) -> Result<(), EncodeError> {
        self.outer.size += 8;
        Ok(())
    }

    fn encode_i128(&mut self, x: i128) -> Result<(), EncodeError> {
        self.outer.size += 16;
        Ok(())
    }

    fn encode_bytes(&mut self, x: &[u8]) -> Result<(), EncodeError> {
        self.outer.size += 4;
        self.outer.encode_bytes(x)
    }

    fn encode_time(&mut self, x: Time) -> Result<(), EncodeError> {
        self.outer.encode_time(x)
    }

    fn encode_duration(&mut self, x: Duration) -> Result<(), EncodeError> {
        self.outer.encode_duration(x)
    }

    fn encode_option(&mut self, visitor: Option<&dyn ValueCodec>) -> Result<(), EncodeError> {
        self.outer.size += 1;
        if let Some(visitor) = visitor {
            visitor.encode(self)?;
        }
        Ok(())
    }

    fn encode_enum_variant(
        &mut self,
        discriminant: i32,
        enum_type: &EnumType,
        value: Option<&dyn ValueCodec>,
    ) -> Result<(), EncodeError> {
        self.outer
            .encode_enum_variant(discriminant, enum_type, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary::encoder::encode_value;
    use crate::encoder::Encoder;
    use crate::mem::MemoryManager;
    use simple_time::Duration;

    // Existing tests
    #[test]
    fn test_u32_size() {
        let mut sizer = EncodeSizer { size: 0 };
        sizer.encode_u32(10).unwrap();
        assert_eq!(sizer.size, 4);
    }

    // Integer type tests
    #[test]
    fn test_u8_encode() {
        let x = 255u8;
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        assert_eq!(res, &[255]);
    }

    #[test]
    fn test_u16_encode() {
        let x = 1000u16;
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        assert_eq!(res, &[232, 3]); // 1000 in little-endian
    }

    #[test]
    fn test_u32_encode() {
        let x = 10u32;
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        assert_eq!(res, &[10, 0, 0, 0]);
    }

    #[test]
    fn test_u64_encode() {
        let x = 1000000u64;
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        assert_eq!(res, &[64, 66, 15, 0, 0, 0, 0, 0]); // 1000000 in little-endian
    }

    #[test]
    fn test_u128_encode() {
        let x = 340282366920938463463374607431768211455u128;
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        assert_eq!(res, &[255; 16]); // Max u128 value
    }

    #[test]
    fn test_i8_encode() {
        let x = -128i8;
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        assert_eq!(res, &[128]); // -128 in two's complement
    }

    #[test]
    fn test_i16_encode() {
        let x = -1000i16;
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        assert_eq!(res, &[24, 252]); // -1000 in little-endian
    }

    #[test]
    fn test_i32_encode() {
        let x = -1000000i32;
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        assert_eq!(res, &[192, 189, 240, 255]); // -1000000 in little-endian
    }

    #[test]
    fn test_i64_encode() {
        let x = -1000000i64;
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        assert_eq!(res, &[192, 189, 240, 255, 255, 255, 255, 255]); // -1000000 in little-endian
    }

    #[test]
    fn test_i128_encode() {
        let x = -170141183460469231731687303715884105728i128;
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        let mut expected = [0; 16];
        expected[15] = 128; // Minimum i128 value
        assert_eq!(res, expected);
    }

    // String and bytes tests
    #[test]
    fn test_str_encode() {
        let x = "hello";
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        assert_eq!(res, b"hello");
    }

    #[test]
    fn test_bytes_encode() {
        let x: &[u8] = &[1, 2, 3, 4, 5];
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        assert_eq!(res, &[1, 2, 3, 4, 5]);
    }

    // Boolean test
    #[test]
    fn test_bool_encode() {
        let x = true;
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        assert_eq!(res, &[1]);

        let x = false;
        let res = encode_value(&x, &mem).unwrap();
        assert_eq!(res, &[0]);
    }

    // Account ID test
    #[test]
    fn test_account_id_encode() {
        let x = AccountID::from(12345u128);
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        let mut expected = [0u8; 16];
        expected[0] = 57;
        expected[1] = 48;
        assert_eq!(res, expected);
    }

    // Time and Duration tests
    #[test]
    fn test_time_encode() {
        let x = Time::from_unix_nanos(1000000000);
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        assert_eq!(res.len(), 16); // Time is encoded as i128
    }

    #[test]
    fn test_duration_encode() {
        let x = Duration::from_nanos(5000000000);
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        assert_eq!(res.len(), 16); // Duration is encoded as i128
    }

    // Option tests
    #[test]
    fn test_option_encode() {
        let x: Option<u32> = Some(42);
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        assert_eq!(res, &[42, 0, 0, 0]); // Value followed by true flag

        let x: Option<u32> = None;
        let res = encode_value(&x, &mem).unwrap();
        let empty: [u8; 0] = [];
        assert_eq!(res, &empty); // Just false flag
    }

    // List test
    #[test]
    fn test_list_encode() {
        let x: Vec<u8> = vec![1, 2, 3];
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        assert_eq!(res, &[1, 2, 3]); // Values followed by length
    }

    // Size tests
    #[test]
    fn test_sizes() {
        let mut sizer = EncodeSizer { size: 0 };

        sizer.encode_u8(0).unwrap();
        assert_eq!(sizer.size, 1);

        sizer.size = 0;
        sizer.encode_u16(0).unwrap();
        assert_eq!(sizer.size, 2);

        sizer.size = 0;
        sizer.encode_u32(0).unwrap();
        assert_eq!(sizer.size, 4);

        sizer.size = 0;
        sizer.encode_u64(0).unwrap();
        assert_eq!(sizer.size, 8);

        sizer.size = 0;
        sizer.encode_u128(0).unwrap();
        assert_eq!(sizer.size, 16);

        sizer.size = 0;
        sizer.encode_str("hello").unwrap();
        assert_eq!(sizer.size, 5);

        sizer.size = 0;
        sizer.encode_bool(true).unwrap();
        assert_eq!(sizer.size, 1);
    }

    // Error cases
    #[test]
    fn test_encode_error_handling() {
        struct BadWriter;
        impl Writer for BadWriter {
            fn write(&mut self, _bytes: &[u8]) -> Result<(), EncodeError> {
                Err(EncodeError::BufferTooSmall)
            }
            fn pos(&self) -> usize {
                0
            }
        }

        let mut encoder = crate::binary::encoder::Encoder {
            writer: &mut BadWriter,
        };
        assert!(encoder.encode_u32(42).is_err());
    }
}
