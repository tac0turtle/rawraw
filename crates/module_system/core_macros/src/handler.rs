use crate::api_builder::APIBuilder;
use crate::migration::{build_on_migrate_handler, collect_on_migrate_info, OnMigrateInfo};
use crate::util::{maybe_extract_attribute, push_item};
use core::borrow::Borrow;
use manyhow::{bail, manyhow};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Attribute, FnArg, ImplItemFn, Item, ItemMod, Signature, Type};

#[derive(deluxe::ParseMetaItem)]
struct HandlerArgs(Ident);

/// Handles the #[handler] attribute.
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
        builder.extract_method_data(&handler, &quote! {#handler}, publish_target)?;
    }

    // handles for all on_migrate functions are generated here
    build_on_migrate_handler(&mut builder, &publish_fns)?;

    // the client struct and its trait implementation are generated here
    let client_ident = format_ident!("{}Client", handler);
    builder.define_client(&client_ident)?;
    builder.define_client_impl(&quote! {#client_ident}, &quote! {pub})?;
    builder.define_client_service(&client_ident, &quote! {#handler})?;
    builder.impl_router(quote! {#handler})?;

    // if there is a function annotated with #[on_create] then we generate a message type for it
    let on_create_msg = match builder.create_msg_name {
        Some(msg) => quote! {#msg},
        None => quote! {()},
    };
    let create_msg_lifetime = &builder.create_msg_lifetime;
    push_item(
        items,
        quote! {
            impl ::ixc::core::handler::HandlerResources for #handler {
                const NAME: &'static str = ::core::concat!(::core::module_path!(), "::", ::core::stringify!(#handler));
            }
        },
    )?;

    let mut visit_trait_schemas = vec![];
    for publish_trait in publish_traits.iter() {
        let trait_ident = &publish_trait.ident;
        visit_trait_schemas.push(quote! {
            <<dyn #trait_ident as ::ixc::core::handler::Service>::Client as ::ixc::core::handler::Client>::visit_schema(visitor);
        });
    }
    push_item(
        items,
        quote! {
            impl ::ixc::core::handler::Handler for #handler {
                type Init<'a> = #on_create_msg #create_msg_lifetime;

                fn visit_schema<'a, V: ::ixc::core::handler::APISchemaVisitor<'a>>(visitor: &mut V) {
                    ::ixc::core::message::visit_init_descriptor::<#on_create_msg #create_msg_lifetime, V>(visitor);
                    <#client_ident as ::ixc::core::handler::Client>::visit_schema(visitor);
                    #(#visit_trait_schemas)*
                }
            }
        },
    )?;

    push_item(
        items,
        quote! {
            impl <'a> ::ixc::core::message::InitMessage<'a> for #on_create_msg #create_msg_lifetime {
                type Codec = ::ixc::schema::binary::NativeBinaryCodec;
            }
        },
    )?;

    // TODO it would nice to be able to combine the routes rather than needing to check one by one
    let mut trait_msg_routers = vec![];
    let mut trait_query_routers = vec![];
    for publish_trait in publish_traits.iter() {
        let trait_ident = &publish_trait.ident;
        trait_msg_routers.push(quote! {
            if let Some(rt) = ::ixc::core::routing::find_route(<dyn #trait_ident as ::ixc::core::routing::Router>::SORTED_MSG_ROUTES, sel) {
                return rt(self, caller, message_packet, callbacks, allocator)
            }
        });
        trait_query_routers.push(quote! {
            if let Some(rt) = ::ixc::core::routing::find_route(<dyn #trait_ident as ::ixc::core::routing::Router>::SORTED_QUERY_ROUTES, sel) {
                return rt(self, message_packet, callbacks, allocator)
            }
        });
    }

    push_item(
        items,
        quote! {
            impl ::ixc::message_api::handler::RawHandler for #handler {
                fn handle_msg<'a>(&self, caller: &::ixc::message_api::AccountID, message_packet: &::ixc::message_api::message::Message, callbacks: &mut dyn ::ixc::message_api::handler::HostBackend, allocator: &'a dyn ::ixc::message_api::handler::Allocator) -> ::core::result::Result<::ixc::message_api::message::Response<'a>, ::ixc::message_api::code::ErrorCode> {
                    let sel = message_packet.request().message_selector();
                    if let Some(rt) = ::ixc::core::routing::find_route(<#handler as ::ixc::core::routing::Router>::SORTED_MSG_ROUTES, sel) {
                        return rt(self, caller, message_packet, callbacks, allocator)
                    }

                    #(#trait_msg_routers)*

                    Err(::ixc::message_api::code::ErrorCode::SystemCode(::ixc::message_api::code::SystemCode::MessageNotHandled))
                }

                fn handle_query<'a>(&self, message_packet: &::ixc::message_api::message::Message, callbacks: &dyn ::ixc::message_api::handler::HostBackend, allocator: &'a dyn ::ixc::message_api::handler::Allocator) -> ::core::result::Result<::ixc::message_api::message::Response<'a>, ::ixc::message_api::code::ErrorCode> {
                    let sel = message_packet.request().message_selector();
                    if let Some(rt) = ::ixc::core::routing::find_route(<#handler as ::ixc::core::routing::Router>::SORTED_QUERY_ROUTES, sel) {
                        return rt(self, message_packet, callbacks, allocator)
                    }

                    #(#trait_query_routers)*

                    Err(::ixc::message_api::code::ErrorCode::SystemCode(::ixc::message_api::code::SystemCode::MessageNotHandled))
                }

                fn handle_system<'a>(&self, caller: &::ixc::message_api::AccountID, message_packet: &::ixc::message_api::message::Message, callbacks: &mut dyn ::ixc::message_api::handler::HostBackend, allocator: &'a dyn ::ixc::message_api::handler::Allocator) -> ::core::result::Result<::ixc::message_api::message::Response<'a>, ::ixc::message_api::code::ErrorCode> {
                    let sel = message_packet.request().message_selector();
                    if let Some(rt) = ::ixc::core::routing::find_route(<#handler as ::ixc::core::routing::Router>::SORTED_SYSTEM_ROUTES, sel) {
                        return rt(self, caller, message_packet, callbacks, allocator)
                    }

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
                    // check if the function has the #[on_migrate] attribute
                    let on_migrate = maybe_extract_attribute(impl_fn)?;
                    let attr_count = on_create.is_some() as usize
                        + publish.is_some() as usize
                        + on_migrate.is_some() as usize;
                    if attr_count > 1 {
                        bail!("only one of #[on_create], #[publish], or #[on_migrate] can be attached to a function");
                    }
                    // we define a publish attribute for the fn if it is annotated directly with #[publish] or if the impl block has #[publish]
                    let publish = publish_all.clone().or(publish);
                    // if it's either a publish fn or an on_create fn
                    if publish.is_some() || on_create.is_some() || on_migrate.is_some() {
                        let ty = if let Some(on_create) = on_create {
                            PublishedFnType::OnCreate { attr: on_create }
                        } else if let Some(publish) = publish {
                            PublishedFnType::Publish {
                                attr: Some(publish),
                            }
                        } else if let Some(on_migrate) = on_migrate {
                            PublishedFnType::OnMigrate(collect_on_migrate_info(
                                impl_fn, on_migrate,
                            )?)
                        } else {
                            unreachable!()
                        };
                        // TODO check visibility - we should probably only allow pub fns
                        // collect the signature and attributes of the fn and add it to the collected_fns vector
                        collected_fns.push(PublishedFnInfo {
                            signature: impl_fn.sig.clone(),
                            attrs: impl_fn.attrs.clone(),
                            ty,
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

/// Represents the data in an #[on_migrate] attribute.
#[derive(deluxe::ExtractAttributes, Debug, Clone)]
#[deluxe(attributes(on_migrate))]
pub(crate) struct OnMigrateAttr {}

/// Represents the data in an #[on_migrate] attribute.
#[derive(deluxe::ExtractAttributes, Debug)]
#[deluxe(attributes(from))]
pub(crate) struct FromAttr {}

/// Describes a function that is published as a message handler.
/// Contains the raw signature, the #[on_create] attribute, the #[publish] attribute,
/// and any other attributes on the function.
#[derive(Debug)]
pub(crate) struct PublishedFnInfo {
    pub(crate) signature: Signature,
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) ty: PublishedFnType,
}

#[derive(Debug)]
pub(crate) enum PublishedFnType {
    Publish { attr: Option<PublishAttr> },
    OnCreate { attr: OnCreateAttr },
    OnMigrate(OnMigrateInfo),
}

/// Describes a trait that is implemented by a handler.
#[derive(Debug)]
struct PublishedTraitInfo {
    ident: Ident,
}
