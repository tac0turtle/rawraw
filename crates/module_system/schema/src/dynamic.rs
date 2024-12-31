//! Dynamic value encoding and decoding.
use crate::any::AnyMessage;
use crate::decoder::{DecodeError, Decoder};
use crate::encoder::{EncodeError, Encoder};
use crate::enums::EnumType;
use crate::field::Field;
use crate::handler::HandlerSchemaResolver;
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
use simple_time::{Duration, Time};

/// A dynamic value that can be encoded and decoded.
#[derive(Debug, Clone)]
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
    Nullable(Option<Box<DynamicValue<'a>, &'a dyn Allocator>>),
}

/// A dynamic struct value.
#[derive(Debug, Clone)]
pub struct DynamicStruct<'a> {
    data: HashMap<usize, DynamicValue<'a>, DefaultHashBuilder, &'a dyn Allocator>,
    fields: List<'a, Field<'a>>,
    type_map: &'a TypeMap<'a>,
    allocator: &'a dyn Allocator,
}

unsafe impl StructEncodeVisitor for DynamicStruct<'_> {
    fn encode_field(&self, index: usize, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        if let Some(value) = self.data.get(&index) {
            value.encode(encoder)
        } else {
            let value = DynamicValue::default_for_field(
                &self.fields.as_slice()[index],
                self.type_map,
                self.allocator,
            )
            .map_err(|_| EncodeError::UnknownError)?;
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
        let field = &self.fields.as_slice()[index];
        let mut value = DynamicValue::default_for_field(field, self.type_map, self.allocator)
            .map_err(|_| DecodeError::UnknownField)?;
        value.decode(decoder)?;
        self.data.insert(index, value);
        Ok(())
    }
}

/// A dynamic list value.
#[derive(Debug, Clone)]
pub struct DynamicList<'a> {
    data: List<'a, DynamicValue<'a>>,
    element_default_value: Box<DynamicValue<'a>, &'a dyn Allocator>,
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

impl ListDecodeVisitor<'_> for DynamicList<'_> {
    fn reserve(&mut self, len: usize, scope: &'_ MemoryManager) -> Result<(), DecodeError> {
        match &mut self.data {
            List::Empty => *self.data = List::Owned(Vec::new_in(scope)),
            List::Borrowed(_) => *self.data = List::Owned(Vec::new_in(scope)),
            List::Owned(vec) => vec.reserve(len),
        }
        Ok(())
    }

    fn next(&mut self, decoder: &mut dyn Decoder<'_>) -> Result<(), DecodeError> {
        if let List::Owned(vec) = &mut self.data {
        } else {
            // expected owned list
            Err(DecodeError::InvalidData)
        }
    }
}

/// A dynamic enum value.
#[derive(Debug, Clone)]
pub struct DynamicEnum<'a> {
    discriminant: i32,
    enum_type: EnumType<'a>,
    value: Option<Box<DynamicValue<'a>, &'a dyn Allocator>>,
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
                todo!()
            }
            DynamicValue::Enum(x) => {
                todo!()
            }
            DynamicValue::List(x) => {
                todo!()
            }
            DynamicValue::AnyMessage(x) => {
                todo!()
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
            DynamicValue::Struct(x) => encoder.encode_struct_fields(x, x.fields.as_slice()),
            DynamicValue::Enum(x) => {
                let value: Option<&dyn ValueCodec> =
                    x.value.as_ref().map(|x| x.as_ref() as &dyn ValueCodec);
                encoder.encode_enum_variant(x.discriminant, &x.enum_type, value)
            }
            DynamicValue::List(x) => encoder.encode_list(x),
            DynamicValue::AnyMessage(x) => encoder.encode_any_message(x),
            DynamicValue::Time(x) => encoder.encode_time(*x),
            DynamicValue::Duration(x) => encoder.encode_duration(*x),
        }
    }
}

impl<'a> DynamicValue<'a> {
    fn default_for_field(
        field: &Field<'a>,
        type_map: &'a TypeMap<'a>,
        allocator: &'a dyn Allocator,
    ) -> Result<Self, ()> {
        if field.nullable {
            let mut field = field.clone();
            field.nullable = false;
            let def_for_field = Self::default_for_field(&field, type_map, allocator)?;
            Ok(DynamicValue::Nullable(Some(Box::new_in(def_for_field, allocator))))
        } else {
            match field.kind {
                Kind::List => {
                    let element_default_value = Self::default_for_kind(
                        field.element_kind.unwrap(),
                        field.referenced_type,
                        type_map,
                        allocator,
                    )?;
                    Ok(DynamicValue::List(DynamicList {
                        data: List::Empty,
                        element_default_value: Box::new_in(element_default_value, allocator),
                        type_map,
                        allocator,
                    }))
                }
                _ => Self::default_for_kind(field.kind, field.referenced_type, type_map, allocator),
            }
        }
    }

    fn default_for_kind(
        kind: Kind,
        ref_type: Option<&'a str>,
        type_map: &'a TypeMap<'a>,
        allocator: &'a dyn Allocator,
    ) -> Result<Self, ()> {
        Ok(match kind {
            Kind::Invalid => DynamicValue::Nullable(None),
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
            Kind::AccountID => { DynamicValue::AccountID(AccountID::EMPTY) }
            Kind::Struct => {
                let ref_type = ref_type.ok_or(())?;
                let struct_type = type_map.lookup_type_by_name(ref_type).ok_or(())?;
                if let SchemaType::Struct(struct_type) = struct_type {
                    return Ok(DynamicValue::Struct(DynamicStruct {
                        data: HashMap::new_in(allocator),
                        fields: List::Borrowed(struct_type.fields),
                        type_map,
                        allocator,
                    }));
                } else {
                    return Err(());
                }
            }
            Kind::Enum => {
                todo!()
            }
            Kind::List => {
                return Err(());
            }
            Kind::AnyMessage => {}
        })
    }
}
