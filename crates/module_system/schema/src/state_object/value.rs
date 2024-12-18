#![allow(unused_variables)]

use crate::binary::encoder::EncodeSizer;
use crate::buffer::WriterFactory;
use crate::decoder::{decode_one, DecodeError, Decoder};
use crate::encoder::{EncodeError, Encoder};
use crate::fields::FieldTypes;
use crate::mem::MemoryManager;
use crate::state_object::field_types::unnamed_struct_type;
use crate::state_object::value_field::ObjectFieldValue;
use crate::structs::StructType;
use crate::value::SchemaValue;
use crate::value::ValueCodec;
use allocator_api2::alloc::Allocator;

/// Encode an object value.
pub fn encode_object_value<'a, V: ObjectValue>(
    value: &V::In<'_>,
    writer_factory: &'a dyn Allocator,
) -> Result<&'a [u8], EncodeError> {
    let mut sizer = EncodeSizer { size: 0 };
    V::encode_reverse(value, &mut sizer)?;
    let mut writer = writer_factory.new_reverse(sizer.size)?;
    let mut encoder = crate::binary::encoder::Encoder {
        writer: &mut writer,
    };
    V::encode_reverse(value, &mut encoder)?;
    Ok(writer.finish())
}

/// Decode an object value.
pub fn decode_object_value<'a, V: ObjectValue>(
    input: &'a [u8],
    memory_manager: &'a MemoryManager,
) -> Result<V::Out<'a>, DecodeError> {
    let mut decoder = crate::binary::decoder::Decoder {
        buf: input,
        scope: memory_manager,
    };
    V::decode(&mut decoder, memory_manager)
}

/// This trait is implemented for types that can be used as tuples of value fields in state objects.
pub trait ObjectValue {
    /// The object value types as field types.
    type FieldTypes<'a>: FieldTypes;
    /// The type that is used when inputting object values to functions.
    type In<'a>;
    /// The type that is used in function return values.
    type Out<'a>;
    /// The associated "pseudo-struct" type for the object value.
    const PSEUDO_TYPE: StructType<'static>;

    /// Encode each part of the value in reverse order.
    fn encode_reverse(value: &Self::In<'_>, encoder: &mut dyn Encoder) -> Result<(), EncodeError>;

    /// Decode the value from the decoder.
    fn decode<'a>(
        decoder: &mut dyn Decoder<'a>,
        mem: &'a MemoryManager,
    ) -> Result<Self::Out<'a>, DecodeError>;
}

impl ObjectValue for () {
    type FieldTypes<'a> = ();
    type In<'a> = ();
    type Out<'a> = ();
    const PSEUDO_TYPE: StructType<'static> = unnamed_struct_type::<Self::FieldTypes<'static>>();

    fn encode_reverse(value: &Self::In<'_>, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        Ok(())
    }

    fn decode<'a>(
        decoder: &mut dyn Decoder<'a>,
        mem: &'a MemoryManager,
    ) -> Result<Self::Out<'a>, DecodeError> {
        Ok(())
    }
}

impl<A: ObjectFieldValue> ObjectValue for A {
    type FieldTypes<'a> = (<<A as ObjectFieldValue>::In<'a> as SchemaValue<'a>>::Type,);
    type In<'a> = A::In<'a>;
    type Out<'a> = A::Out<'a>;
    const PSEUDO_TYPE: StructType<'static> = unnamed_struct_type::<Self::FieldTypes<'static>>();

    fn encode_reverse(value: &Self::In<'_>, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        value.encode(encoder)
    }

    fn decode<'a>(
        decoder: &mut dyn Decoder<'a>,
        mem: &'a MemoryManager,
    ) -> Result<Self::Out<'a>, DecodeError> {
        decode_one(decoder)
    }
}

impl<A: ObjectFieldValue> ObjectValue for (A,) {
    type FieldTypes<'a> = (<<A as ObjectFieldValue>::In<'a> as SchemaValue<'a>>::Type,);
    type In<'a> = (A::In<'a>,);
    type Out<'a> = (A::Out<'a>,);
    const PSEUDO_TYPE: StructType<'static> = unnamed_struct_type::<Self::FieldTypes<'static>>();

    fn encode_reverse(value: &Self::In<'_>, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        value.0.encode(encoder)
    }

    fn decode<'a>(
        decoder: &mut dyn Decoder<'a>,
        mem: &'a MemoryManager,
    ) -> Result<Self::Out<'a>, DecodeError> {
        Ok((decode_one(decoder)?,))
    }
}

impl<A: ObjectFieldValue, B: ObjectFieldValue> ObjectValue for (A, B) {
    type FieldTypes<'a> = (
        <<A as ObjectFieldValue>::In<'a> as SchemaValue<'a>>::Type,
        <<B as ObjectFieldValue>::In<'a> as SchemaValue<'a>>::Type,
    );
    type In<'a> = (A::In<'a>, B::In<'a>);
    type Out<'a> = (A::Out<'a>, B::Out<'a>);
    const PSEUDO_TYPE: StructType<'static> = unnamed_struct_type::<Self::FieldTypes<'static>>();

    fn encode_reverse(value: &Self::In<'_>, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        // encoding always happens in reverse order
        value.1.encode(encoder)?;
        value.0.encode(encoder)
    }

    fn decode<'a>(
        decoder: &mut dyn Decoder<'a>,
        mem: &'a MemoryManager,
    ) -> Result<Self::Out<'a>, DecodeError> {
        Ok((decode_one(decoder)?, decode_one(decoder)?))
    }
}

impl<A: ObjectFieldValue, B: ObjectFieldValue, C: ObjectFieldValue> ObjectValue for (A, B, C) {
    type FieldTypes<'a> = (
        <<A as ObjectFieldValue>::In<'a> as SchemaValue<'a>>::Type,
        <<B as ObjectFieldValue>::In<'a> as SchemaValue<'a>>::Type,
        <<C as ObjectFieldValue>::In<'a> as SchemaValue<'a>>::Type,
    );
    type In<'a> = (A::In<'a>, B::In<'a>, C::In<'a>);
    type Out<'a> = (A::Out<'a>, B::Out<'a>, C::Out<'a>);
    const PSEUDO_TYPE: StructType<'static> = unnamed_struct_type::<Self::FieldTypes<'static>>();

    fn encode_reverse(value: &Self::In<'_>, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        // encoding always happens in reverse order
        value.2.encode(encoder)?;
        value.1.encode(encoder)?;
        value.0.encode(encoder)
    }

    fn decode<'a>(
        decoder: &mut dyn Decoder<'a>,
        mem: &'a MemoryManager,
    ) -> Result<Self::Out<'a>, DecodeError> {
        Ok((
            decode_one(decoder)?,
            decode_one(decoder)?,
            decode_one(decoder)?,
        ))
    }
}

impl<A: ObjectFieldValue, B: ObjectFieldValue, C: ObjectFieldValue, D: ObjectFieldValue> ObjectValue
    for (A, B, C, D)
{
    type FieldTypes<'a> = (
        <<A as ObjectFieldValue>::In<'a> as SchemaValue<'a>>::Type,
        <<B as ObjectFieldValue>::In<'a> as SchemaValue<'a>>::Type,
        <<C as ObjectFieldValue>::In<'a> as SchemaValue<'a>>::Type,
        <<D as ObjectFieldValue>::In<'a> as SchemaValue<'a>>::Type,
    );
    type In<'a> = (A::In<'a>, B::In<'a>, C::In<'a>, D::In<'a>);
    type Out<'a> = (A::Out<'a>, B::Out<'a>, C::Out<'a>, D::Out<'a>);
    const PSEUDO_TYPE: StructType<'static> = unnamed_struct_type::<Self::FieldTypes<'static>>();

    fn encode_reverse(value: &Self::In<'_>, encoder: &mut dyn Encoder) -> Result<(), EncodeError> {
        // encoding always happens in reverse order
        value.3.encode(encoder)?;
        value.2.encode(encoder)?;
        value.1.encode(encoder)?;
        value.0.encode(encoder)
    }

    fn decode<'a>(
        decoder: &mut dyn Decoder<'a>,
        mem: &'a MemoryManager,
    ) -> Result<Self::Out<'a>, DecodeError> {
        Ok((
            decode_one(decoder)?,
            decode_one(decoder)?,
            decode_one(decoder)?,
            decode_one(decoder)?,
        ))
    }
}
