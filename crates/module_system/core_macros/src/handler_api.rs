use crate::api_builder::APIBuilder;
use crate::handler::{PublishedFnInfo, PublishedFnType};
use manyhow::manyhow;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{ItemTrait, TraitItem};

/// Handles the #[handler_api] attribute.
pub(crate) fn handler_api(
    _attr: TokenStream2,
    item_trait: ItemTrait,
) -> manyhow::Result<TokenStream2> {
    let mut builder = APIBuilder::default();
    let trait_ident = &item_trait.ident;
    let dyn_trait = quote!(dyn #trait_ident);
    // we extract method data for each function in the trait using the APIBuilder
    for item in &item_trait.items {
        if let TraitItem::Fn(f) = item {
            let publish_target = PublishedFnInfo {
                signature: f.sig.clone(),
                ty: PublishedFnType::Publish { attr: None },
                attrs: f.attrs.clone(),
            };
            builder.extract_method_data(trait_ident, &dyn_trait, &publish_target)?;
        }
    }

    // the client struct and its trait implementation are generated here
    let client_trait_ident = format_ident!("{}Client", trait_ident);
    let client_impl_ident = format_ident!("{}Impl", client_trait_ident);
    builder.define_client(&client_impl_ident)?;
    builder.define_client_impl(
        &quote! {#client_trait_ident for #client_impl_ident},
        &quote! {},
    )?;
    builder.define_client_impl(
        &quote! {<T: ::ixc::core::handler::HandlerClient> #client_trait_ident for T
        where T::Handler: #trait_ident},
        &quote! {},
    )?;
    builder.define_client_service(&client_impl_ident, &dyn_trait)?;
    builder.define_client_service(&client_impl_ident, &quote! { #client_impl_ident})?;
    let dyn_trait = quote!(dyn #trait_ident);
    builder.impl_router(dyn_trait)?;

    let items = &mut builder.items;

    let client_signatures = &builder.client_signatures;
    Ok(quote! {
        #item_trait

        #(#items)*

        impl ::ixc::message_api::handler::RawHandler for dyn #trait_ident {
            fn handle_msg<'a>(&self, message_packet: &::ixc::message_api::message::Request, callbacks: &mut dyn ixc::message_api::handler::HostBackend, allocator: &'a dyn ::ixc::message_api::handler::Allocator) -> ::core::result::Result<::ixc::message_api::message::Response<'a>, ::ixc::message_api::code::ErrorCode> {
                ::ixc::core::routing::exec_route(self, message_packet, callbacks, allocator)
            }

            fn handle_query<'a>(&self, message_packet: &::ixc::message_api::message::Request, callbacks: &dyn ixc::message_api::handler::HostBackend, allocator: &'a dyn ::ixc::message_api::handler::Allocator) -> ::core::result::Result<::ixc::message_api::message::Response<'a>, ::ixc::message_api::code::ErrorCode> {
                ::ixc::core::routing::exec_query_route(self, message_packet, callbacks, allocator)
            }
        }

        pub trait #client_trait_ident {
            #( #client_signatures; )*
        }
    })
}
