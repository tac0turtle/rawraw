use manyhow::{bail, manyhow};
use quote::{format_ident, quote};
use syn::{Attribute, Item, ItemMod, Signature, Type};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use crate::api_builder::{extract_method_data, APIBuilder};
use crate::util::{maybe_extract_attribute, push_item};
use core::borrow::Borrow;

#[derive(deluxe::ParseMetaItem)]
struct HandlerArgs(Ident);

pub(crate) fn handler(attr: TokenStream2, mut item: ItemMod) -> manyhow::Result<TokenStream2> {
    // first we parse the #[handler] attribute itself which must be attached to a mod block
    let HandlerArgs(handler) = deluxe::parse2(attr)?;
    // then we get a mutable reference to the items in the module
    let items = &mut item.content.as_mut().unwrap().1;

    // any functions annotated directly with #[publish], #[on_create] or in bare impl blocks will be collected here
    let mut publish_fns = vec![];
    // any traits implementations annotated with #[publish] will be collected here
    let mut publish_traits = vec![];
    // now we iterate over all of the items in the module
    // to collect the functions or impl blocks that are annotated with #[publish]
    for item in items.iter_mut() {
        collect_publish_targets(&handler, item, &mut publish_fns, &mut publish_traits)?;
    }

    // the APIBuilder is used to turn the collection functions and traits
    // into the actual code that will be generated
    let mut builder = APIBuilder::default();
    for publish_target in publish_fns.iter() {
        extract_method_data(&handler, &quote! {#handler}, publish_target, &mut builder)?;
    }

    // the client struct and its trait implementation are generated here
    let client_ident = format_ident!("{}Client", handler);
    builder.define_client(&client_ident)?;
    builder.define_client_impl(&quote! {#client_ident}, &quote! {pub})?;
    builder.define_client_service(&client_ident, &quote! {#handler})?;

    // if there is a function annotated with #[on_create] then we generate a message type for it
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

// this function is called on every Item in the mod
// to collect the functions and traits that are annotated with #[publish] or #[on_create]
fn collect_publish_targets(
    // the name of the handler struct
    self_name: &Ident,
    // the item we're examining
    item: &mut Item,
    // the collected functions are added to this vector
    collected_fns: &mut Vec<PublishedFnInfo>,
    // the collected traits are added to this vector
    collected_traits: &mut Vec<PublishedTraitInfo>,
) -> manyhow::Result<()> {
    // if the item is an impl block
    if let Item::Impl(imp) = item {
        // first we check if the impl block is for the handler struct
        if let Type::Path(self_path) = imp.self_ty.borrow() {
            let ident = match self_path.path.get_ident() {
                None => return Ok(()),
                Some(ident) => ident,
            };
            if ident != self_name {
                return Ok(());
            }

            // if the impl block itself has #[publish] attribute
            // we set publish_all to Some(publish_attr)
            let publish_all = maybe_extract_attribute(imp)?;

            // if the impl block is a trait implementation
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
                collected_traits.push(PublishedTraitInfo { ident: trait_ident });
                return Ok(());
            }

            // if we're here then this is a bare impl block (rather than a trait impl)
            // so we iterate over all of the items in the impl block
            for item in &mut imp.items {
                // if the item is a function
                if let syn::ImplItem::Fn(impl_fn) = item {
                    // check if the function has the #[on_create] attribute
                    let on_create = maybe_extract_attribute(impl_fn)?;
                    // check if the function has the #[publish] attribute
                    let publish = maybe_extract_attribute(impl_fn)?;
                    if publish.is_some() && on_create.is_some() {
                        bail!("on_create and publish attributes must not be attached to the same function");
                    }
                    // we define a publish attribute for the fn if it is annotated directly with #[publish] or if the impl block has #[publish]
                    let publish = publish_all.clone().or(publish);
                    // if it's either a publish fn or an on_create fn
                    if publish.is_some() || on_create.is_some() {
                        // TODO check visibility - we should probably only allow pub fns
                        // collect the signature and attributes of the fn and add it to the collected_fns vector
                        collected_fns.push(PublishedFnInfo {
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

/// Represents the data in a #[publish] attribute.
#[derive(deluxe::ExtractAttributes, Clone, Debug)]
#[deluxe(attributes(publish))]
pub(crate) struct PublishAttr {
    package: Option<String>,
    name: Option<String>,
}

/// Represents the data in an #[on_create] attribute.
#[derive(deluxe::ExtractAttributes, Debug)]
#[deluxe(attributes(on_create))]
pub(crate) struct OnCreateAttr {
    message_name: Option<String>,
}

/// Describes a function that is published as a message handler.
/// Contains the raw signature, the #[on_create] attribute, the #[publish] attribute,
/// and any other attributes on the function.
#[derive(Debug)]
pub(crate) struct PublishedFnInfo {
    pub(crate) signature: Signature,
    pub(crate) on_create: Option<OnCreateAttr>,
    pub(crate) publish: Option<PublishAttr>,
    pub(crate) attrs: Vec<Attribute>,
}

/// Describes a trait that is implemented by a handler.
#[derive(Debug)]
struct PublishedTraitInfo {
    ident: Ident,
}

