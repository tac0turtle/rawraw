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
        // Find the minimum number of bytes needed
        let significant_bytes = if x == 0 {
            1
        } else {
            let bits_needed = 128 - x.leading_zeros();
            (bits_needed + 7) / 8
        };

        // Then write the significant bytes in little-endian order
        let bytes = x.to_le_bytes();
        if significant_bytes == 16 {
            // Create a buffer of 17 bytes (16 data + 1 length)
            let mut full_buffer = [0u8; 17];
            // Copy the data bytes into position after the length byte
            full_buffer[0] = significant_bytes as u8; // Length prefix first
            full_buffer[1..].copy_from_slice(&bytes); // Then data
            self.writer.write(&full_buffer)?;
        } else {
            // Then write the data
            self.writer.write(&bytes[..significant_bytes as usize])?;

            // Normal case - write length prefix first
            self.writer.write(&[significant_bytes as u8])?;
        }

        Ok(())
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
    use crate::binary::encoder::encode_value;
    use crate::encoder::Encoder;
    use crate::mem::MemoryManager;

    #[test]
    fn test_u32_size() {
        let mut sizer = crate::binary::encoder::EncodeSizer { size: 0 };
        sizer.encode_u32(10).unwrap();
        assert_eq!(sizer.size, 4);
    }

    #[test]
    fn test_u32_encode() {
        let x = 10u32;
        let mem = MemoryManager::new();
        let res = encode_value(&x, &mem).unwrap();
        assert_eq!(res, &[10, 0, 0, 0]);
    }
    #[test]
    fn test_u128_encode() {
        let test_cases = [
            (0u128, vec![1, 0]),      // Zero needs 0 bytes + 1 byte length prefix
            (1u128, vec![1, 1]),      // One needs 1 byte + 1 byte length prefix
            (255u128, vec![1, 255]),  // Max u8 needs 1 byte
            (256u128, vec![2, 0, 1]), // First u16 needs 2 bytes
            (0xFFFFFFFFu128, vec![4, 255, 255, 255, 255]), // Max u32 needs 4 bytes
            // Test a larger number that needs more bytes
            (
                0x1234567890ABCDEFu128,
                vec![8, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12],
            ),
            (
                0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128,
                vec![
                    16, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                    0xFF, 0xFF, 0xFF, 0xFF,
                ],
            ),
        ];

        let mem = MemoryManager::new();
        for (value, expected) in test_cases {
            let encoded = encode_value(&value, &mem).unwrap();
            assert_eq!(
                encoded,
                expected.as_slice(),
                "Failed encoding {}: got {:?}, expected {:?}",
                value,
                encoded,
                expected
            );
        }
    }

    #[test]
    fn test_i128_encode() {
        let test_cases = [
            (0i128, vec![1, 0]),         // Zero needs 1 byte + length prefix
            (1i128, vec![1, 1]),         // One needs 1 byte + length prefix
            (-1i128, vec![1, 255]),      // Negative one needs 1 byte
            (127i128, vec![1, 127]),     // Max positive i8 needs 1 byte
            (-128i128, vec![1, 128]),    // Min negative i8 needs 1 byte
            (255i128, vec![2, 255, 0]),  // Needs 2 bytes
            (-256i128, vec![2, 0, 255]), // Needs 2 bytes
            (0x7FFFFFFFi128, vec![4, 255, 255, 255, 127]), // Max i32 needs 4 bytes
            (-0x80000000i128, vec![4, 0, 0, 0, 128]), // Min i32 needs 4 bytes
            // Test a larger positive number
            (
                0x1234567890ABCDEFi128,
                vec![8, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12],
            ),
            // Test a larger negative number
            (
                -0x1234567890ABCDEFi128,
                vec![8, 0x11, 0x32, 0x54, 0x6F, 0x87, 0xA9, 0xCB, 0xED],
            ),
        ];

        let mem = MemoryManager::new();
        for (value, expected) in test_cases {
            let encoded = encode_value(&value, &mem).unwrap();
            assert_eq!(
                encoded,
                expected.as_slice(),
                "Failed encoding {}: got {:?}, expected {:?}",
                value,
                encoded,
                expected
            );
        }
    }
}
