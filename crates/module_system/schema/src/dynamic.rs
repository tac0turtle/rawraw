use ixc_message_api::AccountID;
use crate::list::List;

pub enum DynamicValue<'a> {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Bool(bool),
    String(&'a str),
    Bytes(List<'a, u8>),
    AccountID(AccountID),
    Struct(DynamicStruct<'a>),
    List(DynamicList<'a>),
}

pub struct DynamicStruct<'a> {
    data: hashbrown::HashMap<&'a str, DynamicValue<'a>>,
}

pub struct DynamicList<'a> {
    data: List<'a,DynamicValue<'a>>,
}