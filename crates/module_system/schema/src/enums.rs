use crate::kind::Kind;
use crate::types::{ReferenceableType, Type};
use ixc_schema_macros::SchemaValue;

#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub struct EnumType<'a> {
    pub name: &'a str,
    pub variants: &'a [EnumVariantDefinition<'a>],
    pub numeric_kind: Kind,
    pub sealed: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, SchemaValue)]
#[non_exhaustive]
pub struct EnumVariantDefinition<'a> {
    pub name: &'a str,
    pub discriminant: i32,
}

impl<'a> EnumVariantDefinition<'a> {
    pub const fn new(name: &'a str, value: i32) -> Self {
        Self { name, discriminant: value }
    }
}

/// # Safety
/// the function is marked as unsafe to detour users from calling it directly
pub unsafe trait EnumSchema: ReferenceableType + Sized {
    const NAME: &'static str;
    const VARIANTS: &'static [EnumVariantDefinition<'static>];
    const SEALED: bool;
    #[allow(private_bounds)]
    type NumericType: EnumNumericType;
    const ENUM_TYPE: EnumType<'static> = to_enum_type::<Self>();
}

pub const fn to_enum_type<E: EnumSchema>() -> EnumType<'static> {
    EnumType {
        name: E::NAME,
        variants: E::VARIANTS,
        numeric_kind: E::NumericType::KIND,
        sealed: E::SEALED,
    }
}

trait EnumNumericType: Type {}
impl EnumNumericType for i32 {}
impl EnumNumericType for u16 {}
impl EnumNumericType for i16 {}
impl EnumNumericType for u8 {}
impl EnumNumericType for i8 {}

// TODO
// fn encode_enum<E: EnumSchema>(x: &E, encoder: &mut dyn Encoder) -> Result<(), EncodeError>
// where
//     E::NumericType: Into<i32>,
// {
//     encoder.encode_i32(E::into(x.clone()).into())
// }
//
// fn decode_enum<E: EnumSchema>(decoder: &mut dyn Decoder) -> Result<E, DecodeError>
// where
//     E::NumericType: From<i32>,
// {
//     let x = decoder.decode_enum(&E::ENUM_TYPE)?;
//     E::try_from(x.into()).map_err(|_| DecodeError::InvalidData)
// }
