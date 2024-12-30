//! **WARNING: This is an API preview! Most code won't work or even type check properly!**
//! Macros for generating code for the schema crate.

mod mesage_selector;

use crate::mesage_selector::type_selector_from_str;
use manyhow::{bail, manyhow};
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Attribute, Data, DataStruct, Lifetime};

/// This derives a struct codec. The struct must implement Default.
#[manyhow]
#[proc_macro_derive(SchemaValue, attributes(sealed, schema, proto))]
pub fn derive_schema_value(input: syn::DeriveInput) -> manyhow::Result<TokenStream2> {
    match &input.data {
        Data::Struct(str) => {
            return derive_struct_schema(&input, str);
        }
        _ => bail!("only know how to derive SchemaValue for structs"),
    }
}

fn derive_struct_schema(
    input: &syn::DeriveInput,
    str: &DataStruct,
) -> manyhow::Result<TokenStream2> {
    let ixc_schema_path = mk_ixc_schema_path();
    let struct_name = &input.ident;
    // extract struct lifetime
    let generics = &input.generics;
    if generics.lifetimes().count() > 1 {
        bail!("only one lifetime parameter is allowed")
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let lifetime = if let Some(lifetime) = generics.lifetimes().next() {
        lifetime.lifetime.clone()
    } else {
        Lifetime::new("'a", Span::call_site())
    };
    let lifetime2 = if lifetime.ident == "b" {
        Lifetime::new("'c", Span::call_site())
    } else {
        Lifetime::new("'b", Span::call_site())
    };
    let ty_generics2 = if let Some(_lifetime) = generics.lifetimes().next() {
        quote! { < #lifetime2 > }
    } else {
        quote! {}
    };

    let sealed = has_attribute(&input.attrs, "sealed");
    let non_exhaustive = has_attribute(&input.attrs, "non_exhaustive");
    if !sealed && !non_exhaustive {
        bail!("struct must have either a #[sealed] or #[non_exhaustive] attribute to indicate whether adding new fields is or is not a breaking change. Only sealed structs can be used as input types and cannot have new fields added.")
    }
    if sealed && non_exhaustive {
        bail!("struct cannot be both sealed and non_exhaustive")
    }

    let fields = str.fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        quote! {
               #ixc_schema_path::types::to_field::<<#field_type as #ixc_schema_path::SchemaValue< '_ >>::Type>().with_name(stringify!(#field_name)),
        }
    });
    let encode_matchers = str.fields.iter().enumerate().map(|(index, field)| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        quote! {
            #index => <#field_type as #ixc_schema_path::value::ValueCodec<'_>>::encode(&self.#field_name, encoder),
        }
    });
    let decode_matchers = str.fields.iter().enumerate().map(|(index, field)| {
        let field_type = &field.ty;
        let field_name = field.ident.as_ref().unwrap();
        quote! {
            #index => <#field_type as #ixc_schema_path::value::ValueCodec< #lifetime >>::decode(&mut self.#field_name, decoder),
        }
    });

    let type_selector = type_selector_from_str(&struct_name.to_string());
    Ok(quote! {
        unsafe impl #impl_generics #ixc_schema_path::structs::StructSchema for #struct_name #ty_generics #where_clause {
            const STRUCT_TYPE: #ixc_schema_path::structs::StructType<'static> = #ixc_schema_path::structs::StructType {
                name: stringify!(#struct_name),
                fields: &[#(#fields)*],
                sealed: #sealed,
            };

            const TYPE_SELECTOR: u64 = #type_selector;
        }

        unsafe impl #impl_generics #ixc_schema_path::types::ReferenceableType for #struct_name #ty_generics #where_clause {
            const SCHEMA_TYPE: Option<#ixc_schema_path::schema::SchemaType<'static>> = Some(
                #ixc_schema_path::schema::SchemaType::Struct(<Self as #ixc_schema_path::structs::StructSchema>::STRUCT_TYPE)
            );
        }

        unsafe impl #impl_generics #ixc_schema_path::structs::StructEncodeVisitor for #struct_name #ty_generics #where_clause {
            fn encode_field(&self, index: usize, encoder: &mut dyn #ixc_schema_path::encoder::Encoder) -> ::core::result::Result<(), #ixc_schema_path::encoder::EncodeError> {
                match index {
                    #(#encode_matchers)*
                    _ => Err(#ixc_schema_path::encoder::EncodeError::UnknownError),
                }
            }
        }

        unsafe impl< #lifetime > #ixc_schema_path::structs::StructDecodeVisitor< #lifetime > for #struct_name #ty_generics #where_clause {
            fn decode_field(&mut self, index: usize, decoder: &mut dyn #ixc_schema_path::decoder::Decoder< #lifetime >) -> ::core::result::Result<(), #ixc_schema_path::decoder::DecodeError> {
                match index {
                    #(#decode_matchers)*
                    _ => Err(#ixc_schema_path::decoder::DecodeError::UnknownFieldNumber),
                }
            }
        }

        impl < #lifetime > #ixc_schema_path::value::ValueCodec < #lifetime > for #struct_name #ty_generics #where_clause {
            fn decode(
                &mut self,
                decoder: &mut dyn #ixc_schema_path::decoder::Decoder< #lifetime >,
            ) -> ::core::result::Result<(), #ixc_schema_path::decoder::DecodeError> {
                decoder.decode_struct(self, &<Self as #ixc_schema_path::structs::StructSchema>::STRUCT_TYPE)
            }

            fn encode(&self, encoder: &mut dyn #ixc_schema_path::encoder::Encoder) -> ::core::result::Result<(), #ixc_schema_path::encoder::EncodeError> {
                encoder.encode_struct(self, &<Self as #ixc_schema_path::structs::StructSchema>::STRUCT_TYPE)
            }
        }

        impl < #lifetime > #ixc_schema_path::SchemaValue < #lifetime > for #struct_name #ty_generics #where_clause {
            type Type = #ixc_schema_path::types::StructT< #struct_name #ty_generics >;
        }


        impl < #lifetime > #ixc_schema_path::value::ListElementValue < #lifetime > for #struct_name #ty_generics #where_clause {}
        impl #impl_generics #ixc_schema_path::state_object::ObjectFieldValue for #struct_name #ty_generics #where_clause {
            type In< #lifetime2 > = #struct_name #ty_generics2;
            type Out< #lifetime2 > = #struct_name #ty_generics2;
        }
    })
}

fn has_attribute<I>(attrs: &Vec<Attribute>, ident: &I) -> bool
where
    I: ?Sized,
    Ident: PartialEq<I>,
{
    for attr in attrs {
        if attr.path().is_ident(ident) {
            return true;
        }
    }
    false
}

// this is to make macros work with a single import of the ixc crate
fn mk_ixc_schema_path() -> TokenStream2 {
    #[cfg(feature = "use_ixc_macro_path")]
    quote! {::ixc::schema}
    #[cfg(not(feature = "use_ixc_macro_path"))]
    quote! {::ixc_schema}
}
