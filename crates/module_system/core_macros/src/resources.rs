use manyhow::{bail, manyhow};
use quote::quote;
use syn::{Data, DeriveInput};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use crate::util::maybe_extract_attribute;

pub(crate) fn derive_resources(input: DeriveInput) -> manyhow::Result<TokenStream2> {
    let name = input.ident;
    let mut str = match input.data {
        Data::Struct(str) => str,
        _ => bail!("can only derive Resources on structs"),
    };
    let mut field_inits = vec![];
    let mut prefix = 0u8;
    for field in str.fields.iter_mut() {
        let field_name = field.ident.as_ref().unwrap().clone();
        let ty = &field.ty.clone();
        if let Some(state) = maybe_extract_attribute::<_, State>(field)? {
            prefix = state.prefix.unwrap_or(prefix);
            field_inits.push(quote! {
                #field_name: <#ty as ::ixc::core::resource::StateObjectResource>::new(scope.state_scope, #prefix)?
            });
            prefix += 1;
        } else if let Some(client) = maybe_extract_attribute::<_, Client>(field)? {
            let account_id = client.0;
            field_inits.push(quote! {
                #field_name: <#ty as ::ixc::core::handler::Client>::new(::ixc::message_api::AccountID::new(#account_id))
            });
        } else {
            // TODO handle case where both #[state] and #[client] are present
            bail!("only fields with #[state] or #[client] attributes are supported currently");
        }
    }
    Ok(quote! {
        unsafe impl ::ixc::core::resource::Resources for #name {
            unsafe fn new(scope: &::ixc::core::resource::ResourceScope) -> ::core::result::Result<Self, ::ixc::core::resource::InitializationError> {
                Ok(Self {
                    #(#field_inits),*
                })
            }
        }
    })
}

#[derive(deluxe::ExtractAttributes, Debug)]
#[deluxe(attributes(state))]
struct State {
    prefix: Option<u8>,
    #[deluxe(default)]
    key: Vec<Ident>,
    #[deluxe(default)]
    value: Vec<Ident>,
}

#[derive(deluxe::ExtractAttributes, Debug)]
#[deluxe(attributes(client))]
struct Client(u128);
