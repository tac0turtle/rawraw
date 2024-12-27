use crate::util::maybe_extract_attribute;
use manyhow::{bail, manyhow};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{Data, DeriveInput};

/// Derive the `Resources` trait for a struct.
pub(crate) fn derive_resources(input: DeriveInput) -> manyhow::Result<TokenStream2> {
    let name = input.ident;
    let mut str = match input.data {
        Data::Struct(str) => str,
        _ => bail!("can only derive Resources on structs"),
    };
    // these are the field initializers for the struct
    let mut field_inits = vec![];
    // this tracks automatically assigned prefixes for state fields
    // if no prefix is assigned, we use this
    let mut prefix = 0u8;
    let mut visit_state_objects = vec![];
    let mut visit_clients = vec![];
    // we iterator over each field in the struct and extract the #[state] and #[client] attributes
    for field in str.fields.iter_mut() {
        let field_name = field.ident.as_ref().unwrap().clone();
        let ty = &field.ty.clone();
        if let Some(state) = maybe_extract_attribute::<_, StateAttr>(field)? {
            if let Some(client) = maybe_extract_attribute::<_, ClientAttr>(field)? {
                bail!("only one of #[state] or #[client] can be specified per field");
            }
            // update the automatic prefix if it was manually assigned
            prefix = state.prefix.unwrap_or(prefix);
            // add the state field to the initializers
            field_inits.push(quote! {
                #field_name: <#ty as ::ixc::core::resource::StateObjectResource>::new(scope.state_scope, #prefix)?
            });
            let key_names = state.key.iter().map(|s| {
                quote! { stringify!(#s) }
            });
            let value_names = state.value.iter().map(|s| {
                quote! { stringify!(#s) }
            });
            visit_state_objects.push(quote! {
               ::ixc::core::resource::extract_state_object_descriptor::<#ty, V>(visitor, #prefix,
                    stringify!(#field_name),
                    &[#(#key_names),*],
                    &[#(#value_names),*]
                );
            });
            // increment the automatic prefix
            prefix += 1;
            // TODO use the key and value attributes to populate the schema of the state object
        } else if let Some(client) = maybe_extract_attribute::<_, ClientAttr>(field)? {
            // extract the account ID from the client attribute
            // TODO read the account ID from the environment based on a name
            let account_id = client.0;
            // add the client field to the initializers
            field_inits.push(quote! {
                #field_name: <#ty as ::ixc::core::handler::Client>::new(::ixc::message_api::AccountID::new(#account_id))
            });
            visit_clients.push(quote! {
                visitor.visit_client::<#ty>(&::ixc::schema::client::ClientDescriptor::new(stringify!(#field_name), #account_id.into()));
            });
        } else if let Some(client_factory) = maybe_extract_attribute::<_, ClientFactoryAttr>(field)? {
            field_inits.push(quote! {
                #field_name: ::core::default::Default::default()
            });
            visit_clients.push(quote! {
                instance.#field_name.visit_client_schema(visitor, stringify!(#field_name));
            });
        } else {
            bail!("only fields with #[state], #[client] or #[client_factory] attributes are supported currently");
        }
    }
    // return the Resources trait implementation
    Ok(quote! {
        unsafe impl ::ixc::core::resource::Resources for #name {
            unsafe fn new(scope: &::ixc::core::resource::ResourceScope) -> ::core::result::Result<Self, ::ixc::core::resource::InitializationError> {
                Ok(Self {
                    #(#field_inits),*
                })
            }

            fn visit_resources<'c, V: ::ixc::core::resource::ResourcesVisitor<'c>>(visitor: &mut V) {
                let scope = ::ixc::core::resource::ResourceScope::default();
                let instance = unsafe { Self::new(&scope).unwrap() };
                #(#visit_state_objects)*
                #(#visit_clients)*
            }
        }
    })
}

/// The data in a #[state] attribute.
#[derive(deluxe::ExtractAttributes, Debug)]
#[deluxe(attributes(state))]
struct StateAttr {
    prefix: Option<u8>,
    #[deluxe(default)]
    key: Vec<Ident>,
    #[deluxe(default)]
    value: Vec<Ident>,
}

/// The data in a #[client] attribute.
#[derive(deluxe::ExtractAttributes, Debug)]
#[deluxe(attributes(client))]
struct ClientAttr(u128);

/// The data in a #[client_factory] attribute.
#[derive(deluxe::ExtractAttributes, Debug)]
#[deluxe(attributes(client_factory))]
struct ClientFactoryAttr;
