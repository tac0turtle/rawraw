use deluxe::ParseAttributes;
use crate::util::{is_sealed, mk_ixc_schema_path};
use manyhow::bail;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Attribute, DataEnum, Expr, ExprLit, Lit};

pub(crate) fn derive_enum_schema(
    input: &syn::DeriveInput,
    enm: &DataEnum,
) -> manyhow::Result<TokenStream2> {
    let ixc_schema_path = mk_ixc_schema_path();
    let enum_name = &input.ident;
    let is_sealed = is_sealed(input)?;
    // extract repr attribute
    let mut repr = "i32";
    for attr in &input.attrs {
        if <ReprAttr as ParseAttributes<Attribute>>::path_matches(attr.path()) {
            let repr_attr = ReprAttr::parse_attributes(attr)?;
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
    let repr = format_ident!("{}", repr);
    let mut discriminant: i32 = 0;
    let mut variants = vec![];
    let mut variant_decoders = vec![];
    let mut variant_encoders = vec![];
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

        // generate the variant definition
        let variant_def = quote! {
            #ixc_schema_path::enums::EnumVariantDefinition {
                name: stringify!(#variant_name),
                discriminant: #discriminant,
            }
        };
        variants.push(variant_def);

        // generate the variant encoder
        let encode_matcher = quote! {
            #enum_name::#variant_name => #discriminant,
        };
        variant_encoders.push(encode_matcher);

        // generate the variant decoder
        let decode_matcher = quote! {
            #discriminant => #enum_name::#variant_name,
        };
        variant_decoders.push(decode_matcher);

        // increment the discriminant for the next variant
        discriminant += 1;
    }
    let res = quote! {
        unsafe impl #ixc_schema_path::enums::EnumSchema for #enum_name {
            const NAME: &'static str = stringify!(#enum_name);
            const VARIANTS: &'static [#ixc_schema_path::enums::EnumVariantDefinition<'static>] = &[
                #(#variants),*
            ];
            const SEALED: bool = #is_sealed;
            type NumericType = #repr;
        }

        unsafe impl #ixc_schema_path::types::ReferenceableType for #enum_name {
            const SCHEMA_TYPE: Option<#ixc_schema_path::schema::SchemaType<'static>> = Some(
                #ixc_schema_path::schema::SchemaType::Enum(<Self as #ixc_schema_path::enums::EnumSchema>::ENUM_TYPE)
            );
        }

        impl < 'a > #ixc_schema_path::value::ValueCodec < 'a > for #enum_name {
            fn decode(
                &mut self,
                decoder: &mut dyn #ixc_schema_path::decoder::Decoder< 'a >,
            ) -> ::core::result::Result<(), #ixc_schema_path::decoder::DecodeError> {
                let discriminant = decoder.decode_enum_discriminant(&<Self as #ixc_schema_path::enums::EnumSchema>::ENUM_TYPE)?;
                 *self = match discriminant {
                    #(#variant_decoders)*
                    _ => return Err(#ixc_schema_path::decoder::DecodeError::UnknownFieldNumber),
                };
                Ok(())
            }

            fn encode(&self, encoder: &mut dyn #ixc_schema_path::encoder::Encoder) -> ::core::result::Result<(), #ixc_schema_path::encoder::EncodeError> {
                let discriminant = match self {
                    #(#variant_encoders)*
                    _ => return Err(#ixc_schema_path::encoder::EncodeError::UnknownError),
                };
                encoder.encode_enum_discriminant(discriminant, &<Self as #ixc_schema_path::enums::EnumSchema>::ENUM_TYPE)
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
    };
    Ok(res)
}

#[derive(deluxe::ParseAttributes, Clone, Debug)]
#[deluxe(attributes(repr))]
struct ReprAttr(Ident);