use alloc::string::String;
use alloc::vec::Vec;
use ixc_schema_macros::SchemaValue;
use proptest_derive::Arbitrary;

#[derive(SchemaValue, Default, Debug, Eq, PartialEq, Arbitrary)]
#[non_exhaustive]
pub(crate) struct ABitOfEverything {
    pub(crate) primitives: Prims,
    pub(crate) s: String,
    // pub(crate) t: Time,
    // pub(crate) d: Duration,
    pub(crate) v: Vec<u8>,
    pub(crate) ls: Vec<String>,
    pub(crate) li: Vec<i32>,
    pub(crate) lp: Vec<Prims>,
    pub(crate) os: Option<String>,
    pub(crate) op: Option<Prims>,
    pub(crate) e: TestEnum,
    pub(crate) ef: TestEnumWithFields,
}

#[derive(SchemaValue, Default, Debug, Eq, PartialEq, Arbitrary)]
#[non_exhaustive]
pub(crate) struct Prims {
    pub(crate) a_u8: u8,
    pub(crate) a_u16: u16,
    pub(crate) a_u32: u32,
    pub(crate) a_u64: u64,
    pub(crate) a_u128: u128,
    pub(crate) a_i8: i8,
    pub(crate) a_i16: i16,
    pub(crate) a_i32: i32,
    pub(crate) a_i64: i64,
    pub(crate) a_i128: i128,
    pub(crate) a_bool: bool,
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

#[derive(SchemaValue, Default, Debug, Eq, PartialEq, Arbitrary)]
#[non_exhaustive]
pub(crate) enum TestEnumWithFields {
    #[default]
    Default,
    X(u8),
    Y(Prims),
}
