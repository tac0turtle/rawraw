//! **WARNING: This is an API preview! Most code won't work or even type check properly!**
//! Macros for generating code for the schema crate.

mod enums;
mod mesage_selector;
mod structs;
mod util;

use crate::enums::derive_enum_schema;
use crate::structs::derive_struct_schema;
use manyhow::{bail, manyhow};
use proc_macro2::TokenStream as TokenStream2;
use syn::Data;

/// This derives a struct or enum codec.
/// The struct or enum must implement Default.
/// The default value of an enum must be its variant with discriminant 0.
/// Enums may be marked with repr u8, u16, i8, i16, or i32 (the default).
#[manyhow]
#[proc_macro_derive(SchemaValue, attributes(sealed, schema, proto))]
pub fn derive_schema_value(input: syn::DeriveInput) -> manyhow::Result<TokenStream2> {
    match &input.data {
        Data::Struct(str) => derive_struct_schema(&input, str),
        Data::Enum(enm) => derive_enum_schema(&input, enm),
        _ => bail!("only know how to derive SchemaValue for structs"),
    }
}
