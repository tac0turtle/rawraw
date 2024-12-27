//! Enum types.
use crate::decoder::{DecodeError, Decoder};
use crate::field::Field;
use crate::kind::Kind;
use crate::types::{Type, TypeVisitor};
use ixc_schema_macros::SchemaValue;

/// An enum type.
#[derive(Debug, Clone, Eq, PartialEq, SchemaValue, Default)]
#[non_exhaustive]
pub struct EnumType<'a> {
    /// The name of the enum.
    pub name: &'a str,
    /// The variants of the enum.
    pub variants: &'a [EnumVariantDefinition<'a>],
    /// The numeric kind of the enum's discriminant.
    pub numeric_kind: Kind,
    /// Whether the enum is sealed or not.
    pub sealed: bool,
}

/// A definition of an enum variant.
#[derive(Debug, Clone, Default, PartialEq, Eq, SchemaValue)]
#[non_exhaustive]
pub struct EnumVariantDefinition<'a> {
    /// The name of the variant.
    pub name: &'a str,
    /// The discriminant of the variant.
    pub discriminant: i32,
    /// The value of the variant, if any. Variants can have zero or one value associated with them.
    pub value: Option<Field<'a>>,
}

impl<'a> EnumVariantDefinition<'a> {
    /// Create a new enum variant definition.
    pub const fn new(name: &'a str, discriminant: i32, value: Option<Field<'a>>) -> Self {
        Self {
            name,
            discriminant,
            value,
        }
    }
}

/// A type which has an enum schema.
/// # Safety
/// This trait is marked as unsafe because it is meant to be implemented by macros.
pub unsafe trait EnumSchema: Sized {
    /// The name of the enum.
    const NAME: &'static str;
    /// The variants of the enum.
    const VARIANTS: &'static [EnumVariantDefinition<'static>];
    /// Whether the enum is sealed or not.
    const SEALED: bool;
    /// The numeric type of the enum's discriminant.
    #[allow(private_bounds)]
    type NumericType: EnumNumericType;
    /// The enum type definition.
    const ENUM_TYPE: EnumType<'static> = to_enum_type::<Self>();

    /// Visit the enum's variant types.
    fn visit_variant_types<V: TypeVisitor>(visitor: &mut V);
}

/// A visitor for decoding enums.
pub unsafe trait EnumDecodeVisitor<'a> {
    /// Decode a field from the input data.
    fn decode_variant(
        &mut self,
        discriminant: i32,
        decoder: &mut dyn Decoder<'a>,
    ) -> Result<(), DecodeError>;
}

/// Extract the enum type definition from a type which implements [`EnumSchema`].
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
