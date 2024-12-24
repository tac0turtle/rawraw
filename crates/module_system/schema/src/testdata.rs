use crate::codec::Codec;
use alloc::string::String;
use alloc::vec::Vec;
use ixc_schema_macros::SchemaValue;
use proptest::prelude::*;
use proptest_derive::Arbitrary;

#[derive(SchemaValue, Default, Debug, Eq, PartialEq, Arbitrary)]
#[non_exhaustive]
pub(crate) struct ABitOfEverything {
    primitives: Prims,
    s: String,
    // t: Time,
    // d: Duration,
    v: Vec<u8>,
    ls: Vec<String>,
    li: Vec<i32>,
    lp: Vec<Prims>,
    os: Option<String>,
    op: Option<Prims>,
    e: TestEnum,
}

#[derive(SchemaValue, Default, Debug, Eq, PartialEq, Arbitrary)]
#[non_exhaustive]
pub(crate) struct Prims {
    a_u8: u8,
    a_u16: u16,
    a_u32: u32,
    a_u64: u64,
    a_u128: u128,
    a_i8: i8,
    a_i16: i16,
    a_i32: i32,
    a_i64: i64,
    a_i128: i128,
    a_bool: bool,
}

#[derive(SchemaValue, Default, Debug, Eq, PartialEq, Arbitrary)]
#[non_exhaustive]
pub(crate) enum TestEnum {
    #[default]
    A,
    B = 10,
    C = 20,
    D,
}
