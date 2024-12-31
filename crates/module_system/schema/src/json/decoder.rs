use crate::any::AnyMessage;
use crate::decoder::DecodeError;
use crate::enums::{EnumDecodeVisitor, EnumType, EnumVariantDefinition};
use crate::field::Field;
use crate::json::JSONCodec;
use crate::list::ListDecodeVisitor;
use crate::mem::MemoryManager;
use crate::structs::StructDecodeVisitor;
use crate::value::ValueCodec;
use alloc::string::String;
use alloc::vec::Vec;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use core::str::FromStr;
use ixc_message_api::alloc_util::{copy_bytes, copy_str};
use ixc_message_api::AccountID;
use simple_time::{Duration, Time};

impl JSONCodec<'_> {
    /// Decode the value from the JSON input string.
    pub fn decode_value<'a, V: ValueCodec<'a> + Default>(
        &self,
        input: &'a str,
        memory_manager: &'a MemoryManager,
    ) -> Result<V, DecodeError> {
        let value = serde_json::from_str(input).map_err(|_| DecodeError::InvalidData)?;
        let mut decoder = Decoder {
            codec: self,
            value,
            mem: memory_manager,
        };
        let mut res = V::default();
        res.decode(&mut decoder)?;
        Ok(res)
    }
}

struct Decoder<'a, 'b> {
    codec: &'b JSONCodec<'b>,
    value: serde_json::Value,
    mem: &'a MemoryManager,
}

impl<'a> crate::decoder::Decoder<'a> for Decoder<'a, '_> {
    fn decode_bool(&mut self) -> Result<bool, DecodeError> {
        self.value.as_bool().ok_or(DecodeError::InvalidData)
    }

    fn decode_u8(&mut self) -> Result<u8, DecodeError> {
        self.value
            .as_u64()
            .ok_or(DecodeError::InvalidData)?
            .try_into()
            .map_err(|_| DecodeError::InvalidData)
    }

    fn decode_u16(&mut self) -> Result<u16, DecodeError> {
        self.value
            .as_u64()
            .ok_or(DecodeError::InvalidData)?
            .try_into()
            .map_err(|_| DecodeError::InvalidData)
    }

    fn decode_u32(&mut self) -> Result<u32, DecodeError> {
        self.value
            .as_u64()
            .ok_or(DecodeError::InvalidData)?
            .try_into()
            .map_err(|_| DecodeError::InvalidData)
    }

    fn decode_u64(&mut self) -> Result<u64, DecodeError> {
        let s = self.value.as_str().ok_or(DecodeError::InvalidData)?;
        u64::from_str(s).map_err(|_| DecodeError::InvalidData)
    }

    fn decode_u128(&mut self) -> Result<u128, DecodeError> {
        let s = self.value.as_str().ok_or(DecodeError::InvalidData)?;
        u128::from_str(s).map_err(|_| DecodeError::InvalidData)
    }

    fn decode_i8(&mut self) -> Result<i8, DecodeError> {
        self.value
            .as_i64()
            .ok_or(DecodeError::InvalidData)?
            .try_into()
            .map_err(|_| DecodeError::InvalidData)
    }

    fn decode_i16(&mut self) -> Result<i16, DecodeError> {
        self.value
            .as_i64()
            .ok_or(DecodeError::InvalidData)?
            .try_into()
            .map_err(|_| DecodeError::InvalidData)
    }

    fn decode_i32(&mut self) -> Result<i32, DecodeError> {
        self.value
            .as_i64()
            .ok_or(DecodeError::InvalidData)?
            .try_into()
            .map_err(|_| DecodeError::InvalidData)
    }

    fn decode_i64(&mut self) -> Result<i64, DecodeError> {
        let s = self.value.as_str().ok_or(DecodeError::InvalidData)?;
        i64::from_str(s).map_err(|_| DecodeError::InvalidData)
    }

    fn decode_i128(&mut self) -> Result<i128, DecodeError> {
        let s = self.value.as_str().ok_or(DecodeError::InvalidData)?;
        i128::from_str(s).map_err(|_| DecodeError::InvalidData)
    }

    fn decode_borrowed_str(&mut self) -> Result<&'a str, DecodeError> {
        let s = self.value.as_str().ok_or(DecodeError::InvalidData)?;
        unsafe { copy_str(self.mem, s).map_err(|_| DecodeError::InvalidData) }
    }

    fn decode_owned_str(&mut self) -> Result<String, DecodeError> {
        Ok(self.value.as_str().ok_or(DecodeError::InvalidData)?.into())
    }

    fn decode_borrowed_bytes(&mut self) -> Result<&'a [u8], DecodeError> {
        let bz = self.decode_owned_bytes()?;
        unsafe { copy_bytes(self.mem, bz.as_slice()).map_err(|_| DecodeError::InvalidData) }
    }

    fn decode_owned_bytes(&mut self) -> Result<Vec<u8>, DecodeError> {
        let s = self.value.as_str().ok_or(DecodeError::InvalidData)?;
        BASE64_STANDARD
            .decode(s)
            .map_err(|_| DecodeError::InvalidData)
    }

    fn decode_struct_fields(
        &mut self,
        visitor: &mut dyn StructDecodeVisitor<'a>,
        fields: &[Field],
    ) -> Result<(), DecodeError> {
        let obj = self.value.as_object().ok_or(DecodeError::InvalidData)?;
        for (field_name, field_value) in obj.iter() {
            let field_idx = fields
                .iter()
                .position(|f| f.name == field_name)
                .ok_or(DecodeError::UnknownField)?;
            let mut inner = Decoder {
                codec: self.codec,
                value: field_value.clone(),
                mem: self.mem,
            };
            visitor.decode_field(field_idx, &mut inner)?;
        }
        Ok(())
    }

    fn decode_list(&mut self, visitor: &mut dyn ListDecodeVisitor<'a>) -> Result<(), DecodeError> {
        let arr = self.value.as_array().ok_or(DecodeError::InvalidData)?;
        for value in arr.iter() {
            let mut inner = Decoder {
                codec: self.codec,
                value: value.clone(),
                mem: self.mem,
            };
            visitor.next(&mut inner)?;
        }
        Ok(())
    }

    fn decode_option(&mut self, visitor: &mut dyn ValueCodec<'a>) -> Result<bool, DecodeError> {
        if self.value.is_null() {
            return Ok(false);
        }
        visitor.decode(self)?;
        Ok(true)
    }

    fn decode_account_id(&mut self) -> Result<AccountID, DecodeError> {
        let s = self.value.as_str().ok_or(DecodeError::InvalidData)?;
        Ok(self.codec.account_id_codec.decode_str(s)?)
    }

    fn decode_enum_variant(
        &mut self,
        visitor: &mut dyn EnumDecodeVisitor<'a>,
        enum_type: &EnumType,
    ) -> Result<(), DecodeError> {
        match self.value {
            serde_json::Value::Object(ref obj) => {
                let typ = obj.get("type").ok_or(DecodeError::InvalidData)?;
                let variant =
                    find_variant(enum_type, typ.as_str().ok_or(DecodeError::InvalidData)?)?;
                let value = obj.get("value").ok_or(DecodeError::InvalidData)?;
                let mut inner = Decoder {
                    codec: self.codec,
                    value: value.clone(),
                    mem: self.mem,
                };
                visitor.decode_variant(variant.discriminant, &mut inner)
            }
            serde_json::Value::String(ref s) => {
                let variant = find_variant(enum_type, s)?;
                // we pass a decoder with null because we don't have a value to decode
                let mut inner = Decoder {
                    codec: self.codec,
                    value: serde_json::Value::Null,
                    mem: self.mem,
                };
                visitor.decode_variant(variant.discriminant, &mut inner)
            }
            _ => Err(DecodeError::InvalidData),
        }
    }

    fn decode_time(&mut self) -> Result<Time, DecodeError> {
        todo!()
    }

    fn decode_duration(&mut self) -> Result<Duration, DecodeError> {
        todo!()
    }

    fn decode_any_message(&mut self) -> Result<AnyMessage<'a>, DecodeError> {
        todo!()
    }

    fn mem_manager(&self) -> &'a MemoryManager {
        self.mem
    }
}

fn find_variant<'a>(
    enum_type: &EnumType<'a>,
    name: &str,
) -> Result<&'a EnumVariantDefinition<'a>, DecodeError> {
    enum_type
        .variants
        .iter()
        .find(|v| v.name == name)
        .ok_or(DecodeError::InvalidData)
}
