//! This module defines the types that can be used in the schema at a type-level.
//!
//! Unless you are working with the implementation details of schema encoding, then you
//! should consider this module as something that ensures type safety.
//! This module uses a programming style known as type-level programming where types
//! are defined to build other types.
//! None of the types in this module are expected to be instantiated other than as type-level
//! parameters.

use crate::enums::EnumSchema;
use crate::field::Field;
use crate::kind::Kind;
use crate::schema::SchemaType;
use crate::SchemaValue;
use crate::structs::StructSchema;

/// The `Type` trait is implemented for all types that can be used in the schema.
pub trait Type {
    /// The kind of the type.
    const KIND: Kind;

    /// Whether the type is nullable.
    const NULLABLE: bool = false;

    /// The size limit of the type.
    const SIZE_LIMIT: Option<usize> = None;

    /// The element kind of a list type.
    const ELEMENT_KIND: Option<Kind> = None;

    /// The schema type of this type that can be referred to by other types.
    const SCHEMA_TYPE: Option<SchemaType<'static>> = None;

    fn visit_referenced_types<V: TypeVisitor>(_visitor: &mut V) {}
}

pub trait TypeVisitor {
    fn visit<T: Type>(&mut self);
}

/// Converts a type to a field.
pub const fn to_field<T: Type>() -> Field<'static> {
    Field {
        name: "",
        kind: T::KIND,
        nullable: T::NULLABLE,
        element_kind: None,
        referenced_type: "", // TODO
    }
}

#[allow(unused)]
trait Private {}

impl Type for () {
    const KIND: Kind = Kind::Invalid;
}

/// Get the name of the type that is referenced by the given type.
/// Used in macros to generate code for enums and structs.
pub const fn reference_type_name<'a, V: SchemaValue<'a>>() -> &'static str {
    if let Some(t) = <V::Type as Type>::SCHEMA_TYPE {
        t.name()
    } else {
        ""
    }
}

impl Private for u8 {}
impl Type for u8 {
    const KIND: Kind = Kind::Uint8;
}

/// Represents a type that can be used as an element in a list.
pub(crate) trait ListElementType: Type {}

impl Private for u16 {}
impl Type for u16 {
    const KIND: Kind = Kind::Uint16;
}
impl ListElementType for u16 {}

impl Private for u32 {}
impl Type for u32 {
    const KIND: Kind = Kind::Uint32;
}
impl ListElementType for u32 {}

impl Private for u64 {}
impl Type for u64 {
    const KIND: Kind = Kind::Uint64;
}
impl ListElementType for u64 {}

/// The `UIntNT` type represents an unsigned N-bit integer.
pub struct UIntNT<const N: usize>;
impl<const N: usize> Private for UIntNT<N> {}
impl<const N: usize> Type for UIntNT<N> {
    const KIND: Kind = Kind::UIntN;
    const SIZE_LIMIT: Option<usize> = Some(N);
}
impl<const N: usize> ListElementType for UIntNT<N> {}

impl Private for i8 {}
impl Type for i8 {
    const KIND: Kind = Kind::Int8;
}
impl ListElementType for i8 {}

impl Private for i16 {}
impl Type for i16 {
    const KIND: Kind = Kind::Int16;
}
impl ListElementType for i16 {}

impl Private for i32 {}
impl Type for i32 {
    const KIND: Kind = Kind::Int32;
}
impl ListElementType for i32 {}

impl Private for i64 {}
impl Type for i64 {
    const KIND: Kind = Kind::Int64;
}
impl ListElementType for i64 {}

/// The `IntNT` type represents a signed integer represented by N bytes (not bits).
pub struct IntNT<const N: usize>;
impl<const N: usize> Private for IntNT<N> {}
impl<const N: usize> Type for IntNT<N> {
    const KIND: Kind = Kind::IntN;
    const SIZE_LIMIT: Option<usize> = Some(N);
}
impl<const N: usize> ListElementType for IntNT<N> {}

impl Private for bool {}
impl Type for bool {
    const KIND: Kind = Kind::Bool;
}
impl ListElementType for bool {}

/// The `StrT` type represents a string.
pub struct StrT;
impl Private for StrT {}
impl Type for StrT {
    const KIND: Kind = Kind::String;
}
impl ListElementType for StrT {}

/// The `BytesT` type represents a byte array.
pub struct BytesT;
impl Private for BytesT {}
impl Type for BytesT {
    const KIND: Kind = Kind::Bytes;
}
impl ListElementType for BytesT {}

/// The `AddressT` type represents an address.
pub struct AccountIdT;
impl Private for AccountIdT {}
impl Type for AccountIdT {
    const KIND: Kind = Kind::AccountID;
}
impl ListElementType for AccountIdT {}

/// The `TimeT` type represents a time.
pub struct TimeT;
impl Private for TimeT {}
impl Type for TimeT {
    const KIND: Kind = Kind::Time;
}
impl ListElementType for TimeT {}

/// The `DurationT` type represents a duration.
pub struct DurationT;
impl Private for DurationT {}
impl Type for DurationT {
    const KIND: Kind = Kind::Duration;
}
impl ListElementType for DurationT {}

impl<T> Private for Option<T> {}
impl<T: Type> Type for Option<T> {
    const KIND: Kind = T::KIND;
    const NULLABLE: bool = true;
    const SCHEMA_TYPE: Option<SchemaType<'static>> = T::SCHEMA_TYPE;
    fn visit_referenced_types<V: TypeVisitor>(visitor: &mut V) {
        visitor.visit::<T>();
    }
}
impl<T: ListElementType> ListElementType for Option<T> {}

/// The `ListT` type represents a list type.
#[allow(private_bounds)]
pub struct ListT<T: ListElementType> {
    _phantom: core::marker::PhantomData<T>,
}
impl<T: ListElementType> Private for ListT<T> {}
impl<T: ListElementType> Type for ListT<T> {
    const KIND: Kind = Kind::List;
    const ELEMENT_KIND: Option<Kind> = Some(T::KIND);
    const SCHEMA_TYPE: Option<SchemaType<'static>> = T::SCHEMA_TYPE;
    fn visit_referenced_types<V: TypeVisitor>(visitor: &mut V) {
        visitor.visit::<T>();
    }
}

/// The `StructT` type represents a struct type.
pub struct StructT<T> {
    _phantom: core::marker::PhantomData<T>,
}
impl<T> Private for StructT<T> {}
impl<T: StructSchema> Type for StructT<T> {
    const KIND: Kind = Kind::Struct;
    const SCHEMA_TYPE: Option<SchemaType<'static>> = Some(SchemaType::Struct(T::STRUCT_TYPE));
    fn visit_referenced_types<V: TypeVisitor>(visitor: &mut V) {
        T::visit_field_types(visitor);
    }
}
impl<T: StructSchema> ListElementType for StructT<T> {}

/// The `EnumT` type represents an enum type.
pub struct EnumT<T> {
    _phantom: core::marker::PhantomData<T>,
}
impl<T> Private for EnumT<T> {}
impl<T: EnumSchema> Type for EnumT<T> {
    const KIND: Kind = Kind::Enum;
    const SCHEMA_TYPE: Option<SchemaType<'static>> = Some(SchemaType::Enum(T::ENUM_TYPE));

    fn visit_referenced_types<V: TypeVisitor>(visitor: &mut V) {
        T::visit_variant_types(visitor);
    }
}
impl<T: EnumSchema> ListElementType for EnumT<T> {}

#[cfg(feature = "std")]

#[cfg(feature = "std")]
pub fn collect_types<'a, T: SchemaValue<'a>>() -> Result<alloc::collections::BTreeMap<&'static str, SchemaType<'static>>, alloc::vec::Vec<&'static str>> {
    #[derive(Default)]
    struct Visitor {
        types: alloc::collections::BTreeMap<&'static str, SchemaType<'static>>,
        errors: alloc::vec::Vec<&'static str>,
    }
    impl TypeVisitor for Visitor {
        fn visit<T: Type>(&mut self) {
            if let Some(t) = T::SCHEMA_TYPE {
                if let Some(existing) = self.types.get(t.name()) {
                    if existing != &t {
                        self.errors.push(t.name());
                    }
                } else {
                    self.types.insert(t.name(), t);
                }
            }
            T::visit_referenced_types(self);
        }
    }
    let mut visitor = Visitor::default();
    <T::Type>::visit_referenced_types(&mut visitor);
    if visitor.errors.is_empty() {
        Ok(visitor.types)
    } else {
        Err(visitor.errors)
    }
}