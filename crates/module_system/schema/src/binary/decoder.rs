use crate::decoder::DecodeError;
use crate::enums::{EnumDecodeVisitor, EnumType};
use crate::list::ListDecodeVisitor;
use crate::mem::MemoryManager;
use crate::structs::{StructDecodeVisitor, StructType};
use crate::value::ValueCodec;
use alloc::string::String;
use alloc::vec::Vec;
use ixc_message_api::AccountID;
use simple_time::{Duration, Time};

pub fn decode_value<'a>(
    input: &'a [u8],
    memory_manager: &'a MemoryManager,
    visitor: &mut dyn ValueCodec<'a>,
) -> Result<(), DecodeError> {
    visitor.decode(&mut Decoder {
        buf: input,
        scope: memory_manager,
    })
}

pub(crate) struct Decoder<'a> {
    pub(crate) buf: &'a [u8],
    pub(crate) scope: &'a MemoryManager,
}

impl<'a> Decoder<'a> {
    fn read_bytes(&mut self, size: usize) -> Result<&'a [u8], DecodeError> {
        if self.buf.len() < size {
            return Err(DecodeError::OutOfData);
        }
        let bz = &self.buf[0..size];
        self.buf = &self.buf[size..];
        Ok(bz)
    }
}

impl<'a> crate::decoder::Decoder<'a> for Decoder<'a> {
    fn decode_u32(&mut self) -> Result<u32, DecodeError> {
        let bz = self.read_bytes(4)?;
        Ok(u32::from_le_bytes(bz.try_into().unwrap()))
    }

    fn decode_i32(&mut self) -> Result<i32, DecodeError> {
        let bz = self.read_bytes(4)?;
        Ok(i32::from_le_bytes(bz.try_into().unwrap()))
    }

    fn decode_u64(&mut self) -> Result<u64, DecodeError> {
        let bz = self.read_bytes(8)?;
        Ok(u64::from_le_bytes(bz.try_into().unwrap()))
    }

    fn decode_u128(&mut self) -> Result<u128, DecodeError> {
        let bz = self.read_bytes(16)?;
        Ok(u128::from_le_bytes(bz.try_into().unwrap()))
    }

    fn decode_borrowed_str(&mut self) -> Result<&'a str, DecodeError> {
        let bz = self.buf;
        self.buf = &[];
        core::str::from_utf8(bz).map_err(|_| DecodeError::InvalidData)
    }

    fn decode_owned_str(&mut self) -> Result<String, DecodeError> {
        let bz = self.buf;
        self.buf = &[];
        String::from_utf8(bz.to_vec()).map_err(|_| DecodeError::InvalidData)
    }

    fn decode_struct(
        &mut self,
        visitor: &mut dyn StructDecodeVisitor<'a>,
        struct_type: &StructType,
    ) -> Result<(), DecodeError> {
        let mut sub = Decoder {
            buf: self.buf,
            scope: self.scope,
        };
        let mut inner = InnerDecoder { outer: &mut sub };
        for (i, _) in struct_type.fields.iter().enumerate() {
            visitor.decode_field(i, &mut inner)?;
        }
        Ok(())
    }

    fn decode_list(&mut self, visitor: &mut dyn ListDecodeVisitor<'a>) -> Result<(), DecodeError> {
        let size = self.decode_u32()? as usize;
        visitor.reserve(size, self.scope)?;
        let mut sub = Decoder {
            buf: self.buf,
            scope: self.scope,
        };
        let mut inner = InnerDecoder { outer: &mut sub };
        for _ in 0..size {
            visitor.next(&mut inner)?;
        }
        Ok(())
    }

    fn decode_account_id(&mut self) -> Result<AccountID, DecodeError> {
        let id = self.decode_u128()?;
        Ok(AccountID::new(id))
    }

    fn mem_manager(&self) -> &'a MemoryManager {
        self.scope
    }

    fn decode_bool(&mut self) -> Result<bool, DecodeError> {
        let bz = self.read_bytes(1)?;
        Ok(bz[0] != 0)
    }

    fn decode_u8(&mut self) -> Result<u8, DecodeError> {
        let bz = self.read_bytes(1)?;
        Ok(bz[0])
    }

    fn decode_u16(&mut self) -> Result<u16, DecodeError> {
        let bz = self.read_bytes(2)?;
        Ok(u16::from_le_bytes(bz.try_into().unwrap()))
    }

    fn decode_i8(&mut self) -> Result<i8, DecodeError> {
        let bz = self.read_bytes(1)?;
        Ok(bz[0] as i8)
    }

    fn decode_i64(&mut self) -> Result<i64, DecodeError> {
        let bz = self.read_bytes(8)?;
        Ok(i64::from_le_bytes(bz.try_into().unwrap()))
    }

    fn decode_i128(&mut self) -> Result<i128, DecodeError> {
        let bz = self.read_bytes(16)?;
        Ok(i128::from_le_bytes(bz.try_into().unwrap()))
    }

    fn decode_borrowed_bytes(&mut self) -> Result<&'a [u8], DecodeError> {
        let bz = self.buf;
        self.buf = &[];
        Ok(bz)
    }

    fn decode_owned_bytes(&mut self) -> Result<Vec<u8>, DecodeError> {
        let bz = self.buf;
        self.buf = &[];
        Ok(bz.to_vec())
    }

    fn decode_time(&mut self) -> Result<Time, DecodeError> {
        Ok(Time::from_unix_nanos(self.decode_i128()?))
    }

    fn decode_duration(&mut self) -> Result<Duration, DecodeError> {
        Ok(Duration::from_nanos(self.decode_i128()?))
    }

    fn decode_i16(&mut self) -> Result<i16, DecodeError> {
        let bz = self.read_bytes(2)?;
        Ok(i16::from_le_bytes(bz.try_into().unwrap()))
    }

    fn decode_option(&mut self, visitor: &mut dyn ValueCodec<'a>) -> Result<bool, DecodeError> {
        if self.buf.is_empty() {
            Ok(false)
        } else {
            visitor.decode(&mut Decoder {
                buf: self.buf,
                scope: self.scope,
            })?;
            Ok(true)
        }
    }

    fn decode_enum_variant(
        &mut self,
        visitor: &mut dyn EnumDecodeVisitor<'a>,
        _enum_type: &EnumType,
    ) -> Result<(), DecodeError> {
        let discriminant = self.decode_i32()?;
        visitor.decode_variant(discriminant, self)
    }
}

struct InnerDecoder<'b, 'a: 'b> {
    outer: &'b mut Decoder<'a>,
}
impl<'b, 'a: 'b> crate::decoder::Decoder<'a> for InnerDecoder<'b, 'a> {
    fn decode_u32(&mut self) -> Result<u32, DecodeError> {
        self.outer.decode_u32()
    }

    fn decode_i32(&mut self) -> Result<i32, DecodeError> {
        self.outer.decode_i32()
    }

    fn decode_u64(&mut self) -> Result<u64, DecodeError> {
        self.outer.decode_u64()
    }

    fn decode_u128(&mut self) -> Result<u128, DecodeError> {
        self.outer.decode_u128()
    }

    fn decode_borrowed_str(&mut self) -> Result<&'a str, DecodeError> {
        let size = self.decode_u32()? as usize;
        let bz = self.outer.read_bytes(size)?;
        core::str::from_utf8(bz).map_err(|_| DecodeError::InvalidData)
    }

    fn decode_owned_str(&mut self) -> Result<String, DecodeError> {
        let size = self.decode_u32()? as usize;
        let bz = self.outer.read_bytes(size)?;
        String::from_utf8(bz.to_vec()).map_err(|_| DecodeError::InvalidData)
    }

    fn decode_struct(
        &mut self,
        visitor: &mut dyn StructDecodeVisitor<'a>,
        struct_type: &StructType,
    ) -> Result<(), DecodeError> {
        let size = self.decode_u32()? as usize;
        let bz = self.outer.read_bytes(size)?;
        let mut sub = Decoder {
            buf: bz,
            scope: self.outer.scope,
        };
        sub.decode_struct(visitor, struct_type)
    }

    fn decode_list(&mut self, visitor: &mut dyn ListDecodeVisitor<'a>) -> Result<(), DecodeError> {
        let size = self.decode_u32()? as usize;
        let bz = self.outer.read_bytes(size)?;
        let mut sub = Decoder {
            buf: bz,
            scope: self.outer.scope,
        };
        sub.decode_list(visitor)
    }

    fn decode_account_id(&mut self) -> Result<AccountID, DecodeError> {
        self.outer.decode_account_id()
    }

    fn mem_manager(&self) -> &'a MemoryManager {
        self.outer.scope
    }

    fn decode_bool(&mut self) -> Result<bool, DecodeError> {
        self.outer.decode_bool()
    }

    fn decode_u8(&mut self) -> Result<u8, DecodeError> {
        self.outer.decode_u8()
    }

    fn decode_u16(&mut self) -> Result<u16, DecodeError> {
        self.outer.decode_u16()
    }

    fn decode_i8(&mut self) -> Result<i8, DecodeError> {
        self.outer.decode_i8()
    }

    fn decode_i64(&mut self) -> Result<i64, DecodeError> {
        self.outer.decode_i64()
    }

    fn decode_i128(&mut self) -> Result<i128, DecodeError> {
        self.outer.decode_i128()
    }

    fn decode_borrowed_bytes(&mut self) -> Result<&'a [u8], DecodeError> {
        let size = self.decode_u32()? as usize;
        let bz = self.outer.read_bytes(size)?;
        Ok(bz)
    }

    fn decode_owned_bytes(&mut self) -> Result<Vec<u8>, DecodeError> {
        let size = self.decode_u32()? as usize;
        let bz = self.outer.read_bytes(size)?;
        Ok(bz.to_vec())
    }

    fn decode_time(&mut self) -> Result<Time, DecodeError> {
        self.outer.decode_time()
    }

    fn decode_duration(&mut self) -> Result<Duration, DecodeError> {
        self.outer.decode_duration()
    }

    fn decode_i16(&mut self) -> Result<i16, DecodeError> {
        self.outer.decode_i16()
    }

    fn decode_option(&mut self, visitor: &mut dyn ValueCodec<'a>) -> Result<bool, DecodeError> {
        let present = self.decode_bool()?;
        if present {
            visitor.decode(self)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn decode_enum_variant(
        &mut self,
        visitor: &mut dyn EnumDecodeVisitor<'a>,
        enum_type: &EnumType,
    ) -> Result<(), DecodeError> {
        self.outer.decode_enum_variant(visitor, enum_type)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use crate::binary::encoder::encode_value;
    use crate::binary::NativeBinaryCodec;
    use crate::mem::MemoryManager;
    use alloc::string::String;
    use alloc::vec;
    use ixc_message_api::AccountID;
    use ixc_schema_macros::*;
    use simple_time::{Duration, Time};

    // Integer Decoding Tests
    #[test]
    fn test_u32_decode() {
        let buf: [u8; 4] = [10, 0, 0, 0];
        let mem = MemoryManager::new();
        let x = crate::codec::decode_value::<u32>(&NativeBinaryCodec, &buf, &mem).unwrap();
        assert_eq!(x, 10);
    }

    #[test]
    fn test_u8_decode() {
        let buf: [u8; 1] = [255];
        let mem = MemoryManager::new();
        let x = crate::codec::decode_value::<u8>(&NativeBinaryCodec, &buf, &mem).unwrap();
        assert_eq!(x, 255);
    }

    #[test]
    fn test_u16_decode() {
        let buf: [u8; 2] = [232, 3]; // 1000 in little-endian
        let mem = MemoryManager::new();
        let x = crate::codec::decode_value::<u16>(&NativeBinaryCodec, &buf, &mem).unwrap();
        assert_eq!(x, 1000);
    }

    #[test]
    fn test_u64_decode() {
        let buf: [u8; 8] = [64, 66, 15, 0, 0, 0, 0, 0]; // 1000000 in little-endian
        let mem = MemoryManager::new();
        let x = crate::codec::decode_value::<u64>(&NativeBinaryCodec, &buf, &mem).unwrap();
        assert_eq!(x, 1000000);
    }

    #[test]
    fn test_u128_decode() {
        let buf: [u8; 16] = [255; 16]; // Max u128 value
        let mem = MemoryManager::new();
        let x = crate::codec::decode_value::<u128>(&NativeBinaryCodec, &buf, &mem).unwrap();
        assert_eq!(x, u128::MAX);
    }

    // Signed Integer Tests
    #[test]
    fn test_i8_decode() {
        let buf: [u8; 1] = [128]; // -128 in two's complement
        let mem = MemoryManager::new();
        let x = crate::codec::decode_value::<i8>(&NativeBinaryCodec, &buf, &mem).unwrap();
        assert_eq!(x, -128);
    }

    #[test]
    fn test_i16_decode() {
        let buf: [u8; 2] = [24, 252]; // -1000 in little-endian
        let mem = MemoryManager::new();
        let x = crate::codec::decode_value::<i16>(&NativeBinaryCodec, &buf, &mem).unwrap();
        assert_eq!(x, -1000);
    }

    #[test]
    fn test_i32_decode() {
        let buf: [u8; 4] = [192, 189, 240, 255]; // -1000000 in little-endian
        let mem = MemoryManager::new();
        let x = crate::codec::decode_value::<i32>(&NativeBinaryCodec, &buf, &mem).unwrap();
        assert_eq!(x, -1000000);
    }

    #[test]
    fn test_i64_decode() {
        let buf: [u8; 8] = [192, 189, 240, 255, 255, 255, 255, 255]; // -1000000 in little-endian
        let mem = MemoryManager::new();
        let x = crate::codec::decode_value::<i64>(&NativeBinaryCodec, &buf, &mem).unwrap();
        assert_eq!(x, -1000000);
    }

    #[test]
    fn test_i128_decode() {
        let mut buf = [0u8; 16];
        buf[15] = 128; // Minimum i128 value
        let mem = MemoryManager::new();
        let x = crate::codec::decode_value::<i128>(&NativeBinaryCodec, &buf, &mem).unwrap();
        assert_eq!(x, i128::MIN);
    }

    // String Tests
    #[test]
    fn test_decode_borrowed_string() {
        let str = "hello";
        let mem = MemoryManager::new();
        let x =
            crate::codec::decode_value::<&str>(&NativeBinaryCodec, str.as_bytes(), &mem).unwrap();
        assert_eq!(x, "hello");
    }

    #[test]
    fn test_decode_owned_string() {
        let str = "hello";
        let mem = MemoryManager::new();
        let x =
            crate::codec::decode_value::<String>(&NativeBinaryCodec, str.as_bytes(), &mem).unwrap();
        assert_eq!(x, "hello");
    }

    // Boolean Tests
    #[test]
    fn test_bool_decode() {
        let mem = MemoryManager::new();
        let true_val = crate::codec::decode_value::<bool>(&NativeBinaryCodec, &[1], &mem).unwrap();
        assert!(true_val);
        let false_val = crate::codec::decode_value::<bool>(&NativeBinaryCodec, &[0], &mem).unwrap();
        assert!(!false_val);
    }

    // Time and Duration Tests
    #[test]
    fn test_time_decode() {
        let nanos = 1_000_000_000i128; // 1 second after epoch
        let buf = nanos.to_le_bytes();
        let mem = MemoryManager::new();
        let time = crate::codec::decode_value::<Time>(&NativeBinaryCodec, &buf, &mem).unwrap();
        assert_eq!(time.unix_nanos(), nanos);
    }

    #[test]
    fn test_duration_decode() {
        let nanos = 1_000_000_000i128; // 1 second
        let buf = nanos.to_le_bytes();
        let mem = MemoryManager::new();
        let duration =
            crate::codec::decode_value::<Duration>(&NativeBinaryCodec, &buf, &mem).unwrap();
        assert_eq!(duration.nanos(), nanos);
    }

    // Account ID Tests
    #[test]
    fn test_account_id_decode() {
        let id = 12345u128;
        let buf = id.to_le_bytes();
        let mem = MemoryManager::new();
        let account_id =
            crate::codec::decode_value::<AccountID>(&NativeBinaryCodec, &buf, &mem).unwrap();
        assert_eq!(account_id, AccountID::from(id));
    }

    // Collection Tests
    #[test]
    fn test_vec_decode() {
        let mem = MemoryManager::new();
        let encoded = encode_value(&vec![1u8, 2, 3], &mem).unwrap();
        let decoded =
            crate::codec::decode_value::<Vec<u8>>(&NativeBinaryCodec, encoded, &mem).unwrap();
        assert_eq!(decoded, vec![1, 2, 3]);
    }

    #[test]
    fn test_option_decode() {
        let mem = MemoryManager::new();

        // Test Some value
        let some_val = Some(42u32);
        let encoded = encode_value(&some_val, &mem).unwrap();
        let decoded =
            crate::codec::decode_value::<Option<u32>>(&NativeBinaryCodec, encoded, &mem).unwrap();
        assert_eq!(decoded, Some(42));

        // Test None value
        let none_val: Option<u32> = None;
        let encoded = encode_value(&none_val, &mem).unwrap();
        let decoded =
            crate::codec::decode_value::<Option<u32>>(&NativeBinaryCodec, encoded, &mem).unwrap();
        assert_eq!(decoded, None);
    }

    // Error Cases
    #[test]
    fn test_decode_errors() {
        let mem = MemoryManager::new();

        // Test insufficient data
        let result = crate::codec::decode_value::<u32>(&NativeBinaryCodec, &[1, 2], &mem);
        assert!(matches!(
            result,
            Err(crate::decoder::DecodeError::OutOfData)
        ));

        // Test invalid UTF-8
        let result = crate::codec::decode_value::<&str>(&NativeBinaryCodec, &[0xFF, 0xFF], &mem);
        assert!(matches!(
            result,
            Err(crate::decoder::DecodeError::InvalidData)
        ));
    }

    // Complex Struct Tests
    #[derive(Debug, PartialEq, Default, SchemaValue)]
    #[sealed]
    struct TestStruct<'b> {
        field_str: &'b str,
        field_u64: u64,
        field_vec: Vec<u8>,
        field_opt: Option<u32>,
    }

    #[test]
    fn test_struct_decode() {
        let test_struct = TestStruct {
            field_str: "test",
            field_u64: 12345,
            field_vec: vec![1, 2, 3],
            field_opt: Some(42),
        };

        let mem = MemoryManager::new();
        let encoded = encode_value(&test_struct, &mem).unwrap();
        let decoded =
            crate::codec::decode_value::<TestStruct>(&NativeBinaryCodec, encoded, &mem).unwrap();
        assert_eq!(decoded, test_struct);
    }

    // Existing Coin tests
    #[derive(Debug, PartialEq, Default, SchemaValue)]
    #[sealed]
    struct Coin<'b> {
        denom: &'b str,
        amount: u128,
    }

    #[test]
    fn test_coin() {
        let coin = Coin {
            denom: "uatom",
            amount: 1234567890,
        };
        let mem = MemoryManager::new();
        let res = encode_value(&coin, &mem).unwrap();
        let decoded = crate::codec::decode_value::<Coin>(&NativeBinaryCodec, res, &mem).unwrap();
        assert_eq!(decoded, coin);
    }

    #[test]
    fn test_coins() {
        let coins = vec![
            Coin {
                denom: "uatom",
                amount: 1234567890,
            },
            Coin {
                denom: "foo",
                amount: 9876543210,
            },
        ];
        let mem = MemoryManager::new();
        let res = encode_value(&coins, &mem).unwrap();
        let decoded = crate::codec::decode_value::<&[Coin]>(&NativeBinaryCodec, res, &mem).unwrap();
        assert_eq!(decoded, coins);
    }
}
