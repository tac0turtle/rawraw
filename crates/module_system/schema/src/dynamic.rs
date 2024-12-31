//! Dynamic value encoding and decoding.
use crate::any::AnyMessage;
use crate::decoder::{DecodeError, Decoder};
use crate::encoder::{EncodeError, Encoder};
use crate::enums::{EnumDecodeVisitor, EnumType};
use crate::field::Field;
use crate::kind::Kind;
use crate::list::{List, ListDecodeVisitor, ListEncodeVisitor};
use crate::mem::MemoryManager;
use crate::schema::SchemaType;
use crate::structs::{StructDecodeVisitor, StructEncodeVisitor};
use crate::types::TypeMap;
use crate::value::ValueCodec;
use allocator_api2::alloc::Allocator;
use allocator_api2::boxed::Box;
use allocator_api2::vec::Vec;
use hashbrown::{DefaultHashBuilder, HashMap};
use ixc_message_api::AccountID;
use log::info;
use simple_time::{Duration, Time};

/// A dynamic value that can be encoded and decoded.
pub enum DynamicValue<'a> {
    /// An unsigned 8-bit integer.
    U8(u8),
    /// An unsigned 16-bit integer.
    U16(u16),
    /// An unsigned 32-bit integer.
    U32(u32),
    /// An unsigned 64-bit integer.
    U64(u64),
    /// An unsigned 128-bit integer.
    U128(u128),
    /// A signed 8-bit integer.
    I8(i8),
    /// A signed 16-bit integer.
    I16(i16),
    /// A signed 32-bit integer.
    I32(i32),
    /// A signed 64-bit integer.
    I64(i64),
    /// A signed 128-bit integer.
    I128(i128),
    /// A boolean.
    Bool(bool),
    /// A string.
    String(&'a str),
    /// A byte array.
    Bytes(List<'a, u8>),
    /// A time value.
    Time(Time),
    /// A duration value.
    Duration(Duration),
    /// An account ID.
    AccountID(AccountID),
    /// A struct value.
    Struct(DynamicStruct<'a>),
    /// A list value.
    List(DynamicList<'a>),
    /// An enum value.
    Enum(DynamicEnum<'a>),
    /// An any message.
    AnyMessage(AnyMessage<'a>),
    /// A nullable value.
    Nullable(DynamicNullable<'a>),
}

/// A dynamic struct value.
pub struct DynamicStruct<'a> {
    data: HashMap<usize, DynamicValue<'a>, DefaultHashBuilder, &'a dyn Allocator>,
    fields: &'a [Field<'a>],
    type_map: &'a TypeMap<'a>,
    allocator: &'a dyn Allocator,
}

unsafe impl StructEncodeVisitor for DynamicStruct<'_> {
    fn encode_field(&self, index: usize, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        log::debug!("encoding struct field {index}");
        if let Some(value) = self.data.get(&index) {
            value.encode(encoder)
        } else {
            let value =
                DynamicValue::default_for_field(&self.fields[index], self.type_map, self.allocator)
                    .map_err(|_| {
                        log::error!(
                            "failed to to get default value for struct field {}: {:?}",
                            index,
                            self.fields[index]
                        );
                        EncodeError::UnknownError
                    })?;
            value.encode(encoder)
        }
    }
}

unsafe impl<'a> StructDecodeVisitor<'a> for DynamicStruct<'a> {
    fn decode_field(
        &mut self,
        index: usize,
        decoder: &mut dyn Decoder<'a>,
    ) -> Result<(), DecodeError> {
        let field = &self.fields[index];
        log::debug!("decoding struct field {field:?}");
        let mut value = DynamicValue::default_for_field(field, self.type_map, self.allocator)
            .map_err(|_| DecodeError::UnknownField)?;
        value.decode(decoder)?;
        self.data.insert(index, value);
        Ok(())
    }
}

/// A dynamic list value.
pub struct DynamicList<'a> {
    data: List<'a, DynamicValue<'a>>,
    element_kind: Kind,
    ref_type: Option<&'a SchemaType<'a>>,
    type_map: &'a TypeMap<'a>,
    allocator: &'a dyn Allocator,
}

impl ListEncodeVisitor for DynamicList<'_> {
    fn size(&self) -> usize {
        self.data.len()
    }

    fn encode(&self, idx: usize, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        self.data.as_slice()[idx].encode(encoder)
    }
}

impl<'a> ListDecodeVisitor<'a> for DynamicList<'a> {
    fn reserve(&mut self, len: usize, scope: &'a MemoryManager) -> Result<(), DecodeError> {
        match &mut self.data {
            List::Empty => self.data = List::Owned(Vec::new_in(scope)),
            List::Borrowed(_) => self.data = List::Owned(Vec::new_in(scope)),
            List::Owned(ref mut vec) => vec.reserve(len),
        }
        Ok(())
    }

    fn next(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        let mut value = DynamicValue::default_for_kind(
            self.element_kind,
            self.ref_type,
            self.type_map,
            self.allocator,
        )
        .map_err(|_| DecodeError::UnknownField)?;
        
        match &mut self.data {
            List::Empty => {
                self.data = {
                    let mut vec = Vec::new_in(self.allocator);
                    vec.push(value);
                    List::Owned(vec)
                }
            }
            List::Borrowed(vec) => {
                log::error!("expected owned or empty list");
                return Err(DecodeError::InvalidData);
            }
            List::Owned(vec) => {
                vec.push(value);
            }
        }
        Ok(())
    }
}

/// A dynamic enum value.
pub struct DynamicEnum<'a> {
    discriminant: i32,
    enum_type: EnumType<'a>,
    value: Option<Box<DynamicValue<'a>, &'a dyn Allocator>>,
    type_map: &'a TypeMap<'a>,
    allocator: &'a dyn Allocator,
}

unsafe impl<'a> EnumDecodeVisitor<'a> for DynamicEnum<'a> {
    fn decode_variant(
        &mut self,
        discriminant: i32,
        decoder: &mut dyn Decoder<'a>,
    ) -> Result<(), DecodeError> {
        self.discriminant = discriminant;
        let variant = self
            .enum_type
            .variants
            .iter()
            .find(|v| v.discriminant == discriminant)
            .ok_or(DecodeError::UnknownField)?;
        if let Some(value_field) = variant.value {
            let mut value =
                DynamicValue::default_for_field(&value_field, self.type_map, self.allocator)
                    .map_err(|_| DecodeError::InvalidData)?;
            value.decode(decoder)?;
            self.value = Some(Box::new_in(value, self.allocator));
        }
        Ok(())
    }
}

/// A wrapper around a nullable value.
pub struct DynamicNullable<'a> {
    value: Option<Box<DynamicValue<'a>, &'a dyn Allocator>>,
    not_nullable_field: Field<'a>,
    type_map: &'a TypeMap<'a>,
    allocator: &'a dyn Allocator,
}

impl<'a> ValueCodec<'a> for DynamicValue<'a> {
    fn decode(&mut self, decoder: &mut dyn Decoder<'a>) -> Result<(), DecodeError> {
        match self {
            DynamicValue::U8(x) => *x = decoder.decode_u8()?,
            DynamicValue::U16(x) => *x = decoder.decode_u16()?,
            DynamicValue::U32(x) => *x = decoder.decode_u32()?,
            DynamicValue::U64(x) => *x = decoder.decode_u64()?,
            DynamicValue::U128(x) => *x = decoder.decode_u128()?,
            DynamicValue::I8(x) => *x = decoder.decode_i8()?,
            DynamicValue::I16(x) => *x = decoder.decode_i16()?,
            DynamicValue::I32(x) => *x = decoder.decode_i32()?,
            DynamicValue::I64(x) => *x = decoder.decode_i64()?,
            DynamicValue::I128(x) => *x = decoder.decode_i128()?,
            DynamicValue::Bool(x) => *x = decoder.decode_bool()?,
            DynamicValue::String(x) => *x = decoder.decode_borrowed_str()?,
            DynamicValue::Bytes(x) => *x = List::Borrowed(decoder.decode_borrowed_bytes()?),
            DynamicValue::AccountID(x) => *x = decoder.decode_account_id()?,
            DynamicValue::Time(x) => *x = decoder.decode_time()?,
            DynamicValue::Duration(x) => *x = decoder.decode_duration()?,
            DynamicValue::Struct(x) => {
                let mut visitor = DynamicStruct {
                    data: HashMap::new_in(x.allocator),
                    fields: x.fields,
                    type_map: x.type_map,
                    allocator: x.allocator,
                };
                decoder.decode_struct_fields(&mut visitor, x.fields)?;
                *x = visitor;
            }
            DynamicValue::Enum(x) => {
                let mut visitor = DynamicEnum {
                    discriminant: 0,
                    enum_type: x.enum_type.clone(),
                    type_map: x.type_map,
                    allocator: x.allocator,
                    value: None,
                };
                decoder.decode_enum_variant(&mut visitor, &x.enum_type)?;
                *x = visitor;
            }
            DynamicValue::List(x) => decoder.decode_list(x)?,
            DynamicValue::AnyMessage(x) => *x = decoder.decode_any_message()?,
            DynamicValue::Nullable(x) => {
                let mut value =
                    DynamicValue::default_for_field(&x.not_nullable_field, x.type_map, x.allocator)
                        .map_err(|_| DecodeError::UnknownField)?;
                let present = decoder.decode_option(&mut value)?;
                if present {
                    x.value = Some(Box::new_in(value, x.allocator));
                }
            }
        };
        Ok(())
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        match self {
            DynamicValue::U8(x) => encoder.encode_u8(*x),
            DynamicValue::U16(x) => encoder.encode_u16(*x),
            DynamicValue::U32(x) => encoder.encode_u32(*x),
            DynamicValue::U64(x) => encoder.encode_u64(*x),
            DynamicValue::U128(x) => encoder.encode_u128(*x),
            DynamicValue::I8(x) => encoder.encode_i8(*x),
            DynamicValue::I16(x) => encoder.encode_i16(*x),
            DynamicValue::I32(x) => encoder.encode_i32(*x),
            DynamicValue::I64(x) => encoder.encode_i64(*x),
            DynamicValue::I128(x) => encoder.encode_i128(*x),
            DynamicValue::Bool(x) => encoder.encode_bool(*x),
            DynamicValue::String(x) => encoder.encode_str(x),
            DynamicValue::Bytes(x) => encoder.encode_bytes(x.as_slice()),
            DynamicValue::AccountID(x) => encoder.encode_account_id(*x),
            DynamicValue::Struct(x) => encoder.encode_struct_fields(x, x.fields),
            DynamicValue::Enum(x) => {
                let value: Option<&dyn ValueCodec> =
                    x.value.as_ref().map(|x| x.as_ref() as &dyn ValueCodec);
                encoder.encode_enum_variant(x.discriminant, &x.enum_type, value)
            }
            DynamicValue::List(x) => encoder.encode_list(x),
            DynamicValue::AnyMessage(x) => encoder.encode_any_message(x),
            DynamicValue::Time(x) => encoder.encode_time(*x),
            DynamicValue::Duration(x) => encoder.encode_duration(*x),
            DynamicValue::Nullable(x) => {
                let value = x.value.as_ref().map(|x| x.as_ref() as &dyn ValueCodec);
                encoder.encode_option(value)
            }
        }
    }
}

impl<'a> DynamicValue<'a> {
    fn default_for_field(
        field: &Field<'a>,
        type_map: &'a TypeMap<'a>,
        allocator: &'a dyn Allocator,
    ) -> Result<Self, ()> {
        let ref_type = field
            .referenced_type
            .map(|s| type_map.lookup_type_by_name(s))
            .flatten();
        if field.nullable {
            let mut not_nullable_field = field.clone();
            not_nullable_field.nullable = false;
            Ok(DynamicValue::Nullable(DynamicNullable {
                value: None,
                not_nullable_field,
                type_map,
                allocator,
            }))
        } else {
            match field.kind {
                Kind::List => Ok(DynamicValue::List(DynamicList {
                    data: List::Empty,
                    element_kind: field.element_kind.ok_or(())?,
                    ref_type,
                    type_map,
                    allocator,
                })),
                _ => Self::default_for_kind(field.kind, ref_type, type_map, allocator),
            }
        }
    }

    fn default_for_kind(
        kind: Kind,
        ref_type: Option<&'a SchemaType<'a>>,
        type_map: &'a TypeMap<'a>,
        allocator: &'a dyn Allocator,
    ) -> Result<Self, ()> {
        Ok(match kind {
            Kind::Invalid => panic!("invalid kind"),
            Kind::String => DynamicValue::String(""),
            Kind::Bytes => DynamicValue::Bytes(List::Empty),
            Kind::Int8 => DynamicValue::I8(0),
            Kind::Uint8 => DynamicValue::U8(0),
            Kind::Int16 => DynamicValue::I16(0),
            Kind::Uint16 => DynamicValue::U16(0),
            Kind::Int32 => DynamicValue::I32(0),
            Kind::Uint32 => DynamicValue::U32(0),
            Kind::Int64 => DynamicValue::I64(0),
            Kind::Uint64 => DynamicValue::U64(0),
            Kind::Int128 => DynamicValue::I128(0),
            Kind::UInt128 => DynamicValue::U128(0),
            Kind::Bool => DynamicValue::Bool(false),
            Kind::Time => DynamicValue::Time(Time::default()),
            Kind::Duration => DynamicValue::Duration(Duration::default()),
            Kind::AccountID => DynamicValue::AccountID(AccountID::EMPTY),
            Kind::AnyMessage => DynamicValue::AnyMessage(AnyMessage::Empty),
            Kind::Struct => {
                let ref_type = ref_type.ok_or(())?;
                return if let SchemaType::Struct(struct_type) = ref_type {
                    Ok(DynamicValue::Struct(DynamicStruct {
                        data: HashMap::new_in(allocator),
                        fields: struct_type.fields,
                        type_map,
                        allocator,
                    }))
                } else {
                    Err(())
                };
            }
            Kind::Enum => {
                let ref_type = ref_type.ok_or(())?;
                return if let SchemaType::Enum(enum_type) = ref_type {
                    Ok(DynamicValue::Enum(DynamicEnum {
                        discriminant: 0,
                        enum_type: enum_type.clone(),
                        type_map,
                        allocator,
                        value: None,
                    }))
                } else {
                    Err(())
                };
            }
            Kind::List => {
                // lists shouldn't end up here
                return Err(());
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handler::EmptyHandlerSchemaResolver;
    use crate::json;
    use crate::json::account_id::DefaultAccountIDStringCodec;
    use crate::structs::StructSchema;
    use crate::testdata::{ABitOfEverything, Prims};
    use crate::types::collect_types;
    use allocator_api2::vec::Vec;
    use proptest::proptest;

    proptest! {
        #[test]
        fn test_roundtrip(value: ABitOfEverything) {
            let mem = MemoryManager::new();
            let type_map = collect_types::<ABitOfEverything>(&mem).unwrap();
            let cdc = json::JSONCodec::new(&DefaultAccountIDStringCodec, &EmptyHandlerSchemaResolver);
            let mut encoded = Vec::new_in(&mem);
            cdc.encode_value(&value, &mut encoded).unwrap();
            let mut dynamic = DynamicValue::Struct(DynamicStruct {
                data: HashMap::new_in(&mem),
                fields: ABitOfEverything::STRUCT_TYPE.fields,
                type_map: &type_map,
                allocator: &mem,
            });
            cdc.decode_value(encoded.as_slice(), &mem, &mut dynamic)
                .unwrap();
            let mut reencoded = Vec::new_in(&mem);
            cdc.encode_value(&dynamic, &mut reencoded).unwrap();
            assert_eq!(encoded, reencoded);
            let mut decoded = ABitOfEverything::default();
            cdc.decode_value(reencoded.as_slice(), &mem, &mut decoded)
                .unwrap();
            assert_eq!(value, decoded);
        }
    }
}
