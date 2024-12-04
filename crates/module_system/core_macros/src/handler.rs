use manyhow::{bail, manyhow};
use quote::{format_ident, quote};
use syn::{Attribute, Item, ItemMod, Signature, Type};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use crate::api_builder::{derive_api_method, APIBuilder};
use crate::util::{maybe_extract_attribute, push_item};
use core::borrow::Borrow;

#[derive(deluxe::ParseMetaItem)]
struct HandlerArgs(Ident);

/// This derives an account handler.
pub(crate) fn handler(attr: TokenStream2, mut item: ItemMod) -> manyhow::Result<TokenStream2> {
    let HandlerArgs(handler) = deluxe::parse2(attr)?;
    let items = &mut item.content.as_mut().unwrap().1;

    let mut publish_fns = vec![];
    let mut publish_traits = vec![];
    for item in items.iter_mut() {
        collect_publish_targets(&handler, item, &mut publish_fns, &mut publish_traits)?;
    }
    let mut builder = APIBuilder::default();
    for publish_target in publish_fns.iter() {
        derive_api_method(&handler, &quote! {#handler}, publish_target, &mut builder)?;
    }

    let client_ident = format_ident!("{}Client", handler);
    builder.define_client(&client_ident)?;
    builder.define_client_impl(&quote! {#client_ident}, &quote! {pub})?;
    builder.define_client_factory(&client_ident, &quote! {#handler})?;

    let on_create_msg = match builder.create_msg_name {
        Some(msg) => quote! {#msg},
        None => quote! {()},
    };
    let create_msg_lifetime = &builder.create_msg_lifetime;
    push_item(
        items,
        quote! {
            impl ::ixc::core::handler::Handler for #handler {
                const NAME: &'static str = stringify!(#handler);
                type Init<'a> = #on_create_msg #create_msg_lifetime;
            }
        },
    )?;

    push_item(
        items,
        quote! {
            impl <'a> ::ixc::core::handler::InitMessage<'a> for #on_create_msg #create_msg_lifetime {
                type Codec = ::ixc::schema::binary::NativeBinaryCodec;
            }
        },
    )?;

    let routes = &builder.routes;
    let query_routes = &builder.query_routes;
    let system_routes = &builder.system_routes;
    push_item(
        items,
        quote! {
            unsafe impl ::ixc::core::routing::Router for #handler {
                const SORTED_ROUTES: &'static [::ixc::core::routing::Route<Self>] =
                    &::ixc::core::routing::sort_routes([
                        #(#routes)*
                    ]);

                const SORTED_QUERY_ROUTES: &'static [::ixc::core::routing::Route<Self>] =
                    &::ixc::core::routing::sort_routes([
                        #(#query_routes)*
                    ]);

                const SORTED_SYSTEM_ROUTES: &'static [::ixc::core::routing::Route<Self>] =
                    &::ixc::core::routing::sort_routes([
                        #(#system_routes)*
                    ]);
            }
        },
    )?;

    // TODO it would nice to be able to combine the routes rather than needing to check one by one
    let mut trait_routers = vec![];
    for publish_trait in publish_traits.iter() {
        let trait_ident = &publish_trait.ident;
        trait_routers.push(quote! {
            if let Some(rt) = ::ixc::core::routing::find_route::<dyn #trait_ident>(sel) {
                return rt.1(self, message_packet, callbacks, allocator)
            }
        })
    }

    push_item(
        items,
        quote! {
            impl ::ixc::message_api::handler::RawHandler for #handler {
                fn handle(&self, message_packet: &mut ::ixc::message_api::packet::MessagePacket, callbacks: &dyn ::ixc::message_api::handler::HostBackend, allocator: &dyn ::ixc::message_api::handler::Allocator) -> ::core::result::Result<(), ::ixc::message_api::code::ErrorCode> {
                    let sel = message_packet.header().message_selector;
                    if let Some(rt) = ::ixc::core::routing::find_route(sel) {
                        return rt.1(self, message_packet, callbacks, allocator)
                    }

                    #(#trait_routers)*

                    Err(::ixc::message_api::code::ErrorCode::SystemCode(::ixc::message_api::code::SystemCode::MessageNotHandled))
                }
            }
        },
    )?;

    push_item(
        items,
        quote! {
            impl ::ixc::core::handler::HandlerClient for #client_ident {
                type Handler = #handler;
            }
        },
    )?;

    items.append(&mut builder.items);

    let expanded = quote! {
        #item
    };
    Ok(expanded)
}

fn collect_publish_targets(
    self_name: &syn::Ident,
    item: &mut Item,
    targets: &mut Vec<PublishFn>,
    traits: &mut Vec<PublishTrait>,
) -> manyhow::Result<()> {
    if let Item::Impl(imp) = item {
        if let Type::Path(self_path) = imp.self_ty.borrow() {
            let ident = match self_path.path.get_ident() {
                None => return Ok(()),
                Some(ident) => ident,
            };
            if ident != self_name {
                return Ok(());
            }

            let publish_all = maybe_extract_attribute(imp)?;

            // TODO check for trait implementation
            if imp.trait_.is_some() && publish_all.is_some() {
                let trait_ident = imp
                    .trait_
                    .as_ref()
                    .unwrap()
                    .1
                    .segments
                    .first()
                    .unwrap()
                    .ident
                    .clone();
                traits.push(PublishTrait { ident: trait_ident });
                return Ok(());
            }

            for item in &mut imp.items {
                if let syn::ImplItem::Fn(impl_fn) = item {
                    let on_create = maybe_extract_attribute(impl_fn)?;
                    let publish = maybe_extract_attribute(impl_fn)?;
                    if publish.is_some() && on_create.is_some() {
                        bail!("on_create and publish attributes must not be attached to the same function");
                    }
                    let publish = publish_all.clone().or(publish);
                    if publish.is_some() || on_create.is_some() {
                        // TODO check visibility
                        targets.push(PublishFn {
                            signature: impl_fn.sig.clone(),
                            on_create,
                            publish,
                            attrs: impl_fn.attrs.clone(),
                        });
                    }
                }
            }
        }
    }
    Ok(())
}

#[derive(deluxe::ExtractAttributes, Clone, Debug)]
#[deluxe(attributes(publish))]
pub(crate) struct Publish {
    package: Option<String>,
    name: Option<String>,
}

#[derive(deluxe::ExtractAttributes, Debug)]
#[deluxe(attributes(on_create))]
pub(crate) struct OnCreate {
    message_name: Option<String>,
}

#[derive(Debug)]
pub(crate) struct PublishFn {
    pub(crate) signature: Signature,
    pub(crate) on_create: Option<OnCreate>,
    pub(crate) publish: Option<Publish>,
    pub(crate) attrs: Vec<Attribute>,
}

#[derive(Debug)]
struct PublishTrait {
    ident: Ident,
}

