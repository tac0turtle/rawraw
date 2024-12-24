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

/// This derives a struct codec. The struct must implement Default.
#[manyhow]
#[proc_macro_derive(SchemaValue, attributes(sealed, schema, proto))]
pub fn derive_schema_value(input: syn::DeriveInput) -> manyhow::Result<TokenStream2> {
    match &input.data {
        Data::Struct(str) => derive_struct_schema(&input, str),
        Data::Enum(enm) => derive_enum_schema(&input, enm),
        _ => bail!("only know how to derive SchemaValue for structs"),
    }
}
