use crate::mesage_selector::type_selector_from_str;
use crate::util::{extract_generics, is_sealed, mk_ixc_schema_path, GenericInfo};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DataStruct;

pub(crate) fn derive_struct_schema(
    input: &syn::DeriveInput,
    str: &DataStruct,
) -> manyhow::Result<TokenStream2> {
    let ixc_schema_path = mk_ixc_schema_path();
    let struct_name = &input.ident;

    let GenericInfo {
        lifetime,
        lifetime2,
        ty_generics2,
        impl_generics,
        where_clause,
        ty_generics,
    } = extract_generics(input)?;

    let sealed = is_sealed(input)?;

    let visit_field_types = str.fields.iter().map(|field| {
        let field_type = &field.ty;
        quote! { visitor.visit::< < #field_type as #ixc_schema_path::SchemaValue< '_ >>::Type >(); }
    });
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
            const STRUCT_TYPE: #ixc_schema_path::structs::StructType<'static> = #ixc_schema_path::structs::StructType::new(
                stringify!(#struct_name),
                 &[#(#fields)*],
                 #sealed,
                 #type_selector,
            );

            const TYPE_SELECTOR: u64 = #type_selector;

            fn visit_field_types<V: #ixc_schema_path::types::TypeVisitor>(visitor: &mut V) {
                #(#visit_field_types);*
            }
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
                    _ => Err(#ixc_schema_path::decoder::DecodeError::UnknownField),
                }
            }
        }

        impl < #lifetime > #ixc_schema_path::value::ValueCodec < #lifetime > for #struct_name #ty_generics #where_clause {
            fn decode(
                &mut self,
                decoder: &mut dyn #ixc_schema_path::decoder::Decoder< #lifetime >,
            ) -> ::core::result::Result<(), #ixc_schema_path::decoder::DecodeError> {
                decoder.decode_struct_fields(self, &<Self as #ixc_schema_path::structs::StructSchema>::STRUCT_TYPE.fields)
            }

            fn encode(&self, encoder: &mut dyn #ixc_schema_path::encoder::Encoder) -> ::core::result::Result<(), #ixc_schema_path::encoder::EncodeError> {
                encoder.encode_struct_fields(self, &<Self as #ixc_schema_path::structs::StructSchema>::STRUCT_TYPE.fields)
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
