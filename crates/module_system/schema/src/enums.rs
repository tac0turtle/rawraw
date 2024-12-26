use ixc_schema_macros::SchemaValue;
use crate::kind::Kind;
use crate::types::{Type, TypeVisitor};
use crate::decoder::{DecodeError, Decoder};
use crate::field::Field;

#[derive(Debug, Clone, Eq, PartialEq, SchemaValue, Default)]
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
    pub value: Option<Field<'a>>,
}

impl<'a> EnumVariantDefinition<'a> {
    pub const fn new(name: &'a str, discriminant: i32, value: Option<Field<'a>>) -> Self {
        Self { name, discriminant, value }
    }
}

/// # Safety
/// the function is marked as unsafe to detour users from calling it directly
pub unsafe trait EnumSchema: Sized {
    const NAME: &'static str;
    const VARIANTS: &'static [EnumVariantDefinition<'static>];
    const SEALED: bool;
    #[allow(private_bounds)]
    type NumericType: EnumNumericType;
    const ENUM_TYPE: EnumType<'static> = to_enum_type::<Self>();

    fn visit_variant_types<V: TypeVisitor>(visitor: &mut V);
}

pub unsafe trait EnumDecodeVisitor<'a> {
    /// Decode a field from the input data.
    fn decode_variant(
        &mut self,
        discriminant: i32,
        decoder: &mut dyn Decoder<'a>,
    ) -> Result<(), DecodeError>;
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
