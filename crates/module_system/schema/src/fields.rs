//! Special traits for fields.
//! This is sort of an internal module which is used
//! for constructing state object key and value tuples.
use crate::field::Field;
use crate::types::{to_field, Type};

/// A trait representing a set of fields.
pub trait FieldTypes {
    /// The number of fields in the set.
    const N: usize;
    /// The definition of the fields in the set.
    const FIELDS: &'static [Field<'static>];
}
impl FieldTypes for () {
    const N: usize = 0;
    const FIELDS: &'static [Field<'static>] = &[];
}
impl<A: Type> FieldTypes for (A,) {
    const N: usize = 1;
    const FIELDS: &'static [Field<'static>] = &[to_field::<A>()];
}
impl<A: Type, B: Type> FieldTypes for (A, B) {
    const N: usize = 2;
    const FIELDS: &'static [Field<'static>] = &[to_field::<A>(), to_field::<B>()];
}
impl<A: Type, B: Type, C: Type> FieldTypes for (A, B, C) {
    const N: usize = 3;
    const FIELDS: &'static [Field<'static>] = &[to_field::<A>(), to_field::<B>(), to_field::<C>()];
}

impl<A: Type, B: Type, C: Type, D: Type> FieldTypes for (A, B, C, D) {
    const N: usize = 4;
    const FIELDS: &'static [Field<'static>] = &[
        to_field::<A>(),
        to_field::<B>(),
        to_field::<C>(),
        to_field::<D>(),
    ];
}
