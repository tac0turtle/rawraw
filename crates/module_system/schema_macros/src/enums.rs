use deluxe::ParseAttributes;
use crate::util::{is_sealed, maybe_extract_attribute, mk_ixc_schema_path};
use manyhow::bail;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{DataEnum, Expr, ExprLit, Lit};

pub(crate) fn derive_enum_schema(
    input: &syn::DeriveInput,
    enm: &DataEnum,
) -> manyhow::Result<TokenStream2> {
    let ixc_schema_path = mk_ixc_schema_path();
    let enum_name = &input.ident;
    let is_sealed = is_sealed(input)?;
    // extract repr attribute
    let mut repr = "i32";
    for attr in input.attrs {
        if ReprAttr::path_matches(attr.meta.path()) {
            let repr_attr = ReprAttr::parse_attributes(&attr)?;
            let repr_str = repr_attr.0.to_string();
            match repr_str.as_str() {
                "u8" => repr = "u8",
                "i8" => repr = "i8",
                "u16" => repr = "u16",
                "i16" => repr = "i16",
                "i32" => repr = "i32",
                _ => bail!("unsupported repr {:?}", repr_str),
            }
        }
    }
    //     has_attribute(&input.attrs, "repr") {
    //     if attr.path().is_ident("repr") {
    //         if let syn::Meta::List(list) = &attr.tokens {
    //             if list.path.is_ident("u8") {
    //                 repr = "u8";
    //             } else if list.path.is_ident("i8") {
    //                 repr = "i8";
    //             } else if list.path.is_ident("u16") {
    //                 repr = "u16";
    //             } else if list.path.is_ident("i16") {
    //                 repr = "i16";
    //             } else if list.path.is_ident("i32") {
    //                 repr = "i32";
    //             } else {
    //                 bail!("unsupported repr {:?}", list);
    //             }
    //         } else {
    //             bail!("unsupported repr {:?}", attr.meta);
    //         }
    //         break;
    //     }
    // }
    let mut discriminant: i32 = 0;
    for variant in &enm.variants {
        if variant.fields.len() != 0 {
            bail!("currently, only enums with no fields are supported");
        }
        let variant_name = &variant.ident;
        if let Some(variant_discriminant) = &variant.discriminant {
            if let Expr::Lit(ExprLit { lit: Lit::Int(int), .. }) = &variant_discriminant.1 {
                discriminant = int.base10_parse::<i32>()?;
            } else {
                bail!("unsupported discriminant {:?}", variant_discriminant);
            }
        }

        // increment the discriminant for the next variant
        discriminant += 1;
    }
    Ok(quote! {
        unsafe impl #ixc_schema_path::enums::EnumSchema for #enum_name {
            const NAME: &'static str = stringify!(#enum_name);
            const CASES: &'static [] = &[];
            const SEALED: bool = #is_sealed;
            type NumericType = #ixc_schema_path::kind::Int32;
        }

        unsafe impl #ixc_schema_path::types::ReferenceableType for #enum_name {
            const SCHEMA_TYPE: Option<#ixc_schema_path::schema::SchemaType<'static>> = Some(
                #ixc_schema_path::schema::SchemaType::Enum(<Self as #ixc_schema_path::structs::EnumSchema>::ENUM_TYPE)
            );
        }

        impl < 'a > #ixc_schema_path::value::ValueCodec < 'a > for #enum_name {
            fn decode(
                &mut self,
                decoder: &mut dyn #ixc_schema_path::decoder::Decoder< 'a >,
            ) -> ::core::result::Result<(), #ixc_schema_path::decoder::DecodeError> {
                todo!()
            }

            fn encode(&self, encoder: &mut dyn #ixc_schema_path::encoder::Encoder) -> ::core::result::Result<(), #ixc_schema_path::encoder::EncodeError> {
                todo!()
            }
        }

        impl < 'a > #ixc_schema_path::SchemaValue < 'a > for #enum_name {
            type Type = #ixc_schema_path::types::EnumT< #enum_name >;
        }

        impl < 'a > #ixc_schema_path::value::ListElementValue < 'a > for #enum_name {}
        impl #ixc_schema_path::state_object::ObjectFieldValue for #enum_name {
            type In< 'b > = #enum_name;
            type Out< 'b > = #enum_name;
        }
    })
}

#[derive(deluxe::ParseAttributes, Clone, Debug)]
#[deluxe(attributes(repr))]
struct ReprAttr(Ident);