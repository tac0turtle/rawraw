use crate::util::maybe_extract_attribute;
use manyhow::{bail, manyhow};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{Data, DeriveInput, Expr, Lit};

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
            // increment the automatic prefix
            prefix += 1;
            // TODO use the key and value attributes to populate the schema of the state object
        } else if let Some(client) = maybe_extract_attribute::<_, ClientAttr>(field)? {
            // extract the account ID from the client attribute
            match client.0 {
                Expr::Lit(lit) => {
                    if let Lit::Int(account_id) = lit.lit {
                        // the account ID is hard-coded as a u128
                        let account_id = account_id.base10_parse::<u128>()?;
                        field_inits.push(quote! {
                            #field_name: <#ty as ::ixc::core::handler::Client>::new(::ixc::message_api::AccountID::new(#account_id))
                        });
                    } else {
                        bail!("client attribute must be a u128 or identifier, got {lit:?}");
                    }
                }
                Expr::Path(path) => {
                    if path.path.segments.len() != 1 {
                        bail!("client attribute must be a u128 or identifier, got {path:?}");
                    }
                    let account_name = path.path.segments.first().unwrap().ident.to_string();
                    // the account ID needs to be resolved at compile time or runtime
                    field_inits.push(quote! {
                        #field_name: <#ty as ::ixc::core::handler::Client>::new(scope.resolve_account(#account_name, ::ixc::core::known_accounts::lookup_known_account(#account_name))?)
                    });
                }
                x => bail!("client attribute must be a u128 or identifier, got {x:?}"),
            }
        } else {
            bail!("only fields with #[state] or #[client] attributes are supported currently");
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
struct ClientAttr(Expr);
