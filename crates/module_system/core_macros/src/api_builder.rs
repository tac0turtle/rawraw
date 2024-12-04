use heck::ToUpperCamelCase;
use manyhow::{bail, ensure};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{parse_quote, Item, ReturnType, Signature, Type};
use syn::punctuated::Punctuated;
use core::borrow::Borrow;
use crate::handler::PublishFn;
use crate::message_selector::message_selector_from_str;
use crate::util::push_item;

#[derive(Default)]
pub(crate) struct APIBuilder {
    pub(crate) items: Vec<Item>,
    pub(crate) routes: Vec<TokenStream2>,
    pub(crate) query_routes: Vec<TokenStream2>,
    pub(crate) system_routes: Vec<TokenStream2>,
    pub(crate) client_signatures: Vec<Signature>,
    client_methods: Vec<TokenStream2>,
    pub(crate) create_msg_name: Option<Ident>,
    pub(crate) create_msg_lifetime: TokenStream2,
}

impl APIBuilder {
    pub(crate) fn define_client(&mut self, client_ident: &Ident) -> manyhow::Result<()> {
        push_item(
            &mut self.items,
            quote! {
                pub struct #client_ident(::ixc::message_api::AccountID);
            },
        )?;
        push_item(
            &mut self.items,
            quote! {
                impl ::ixc::core::handler::Client for #client_ident {
                    fn new(account_id: ::ixc::message_api::AccountID) -> Self {
                        Self(account_id)
                    }

                    fn account_id(&self) -> ::ixc::message_api::AccountID {
                        self.0
                    }
                }
            },
        )
    }

    pub(crate) fn define_client_impl(
        &mut self,
        impl_target: &TokenStream2,
        visibility: &TokenStream2,
    ) -> manyhow::Result<()> {
        let client_methods = &self.client_methods;
        push_item(
            &mut self.items,
            quote! {
                impl #impl_target {
                    #(#visibility #client_methods)*
                }
            },
        )
    }

    pub(crate) fn define_client_factory(
        &mut self,
        client_ident: &Ident,
        factory_target: &TokenStream2,
    ) -> manyhow::Result<()> {
        push_item(
            &mut self.items,
            quote! {
                impl ::ixc::core::handler::Service for #factory_target {
                    type Client = #client_ident;
                }
            },
        )
    }
}

pub(crate) fn derive_api_method(
    handler_ident: &Ident,
    _handler_ty: &TokenStream2,
    publish_target: &PublishFn,
    builder: &mut APIBuilder,
) -> manyhow::Result<()> {
    let signature = &publish_target.signature;
    let fn_name = &signature.ident;
    let ident_camel = fn_name.to_string().to_upper_camel_case();
    let msg_struct_name = format_ident!("{}{}", handler_ident, ident_camel);
    let mut signature = signature.clone();
    let mut new_inputs = Punctuated::new();
    let mut msg_fields = vec![];
    let mut msg_deconstruct = vec![];
    let mut fn_ctr_args = vec![];
    let mut msg_fields_init = vec![];
    let mut have_lifetimes = false;
    let mut context_name: Option<Ident> = None;
    let mut is_query = false;
    for field in &signature.inputs {
        match field {
            syn::FnArg::Receiver(r) => {
                if r.mutability.is_some() {
                    bail!(
                        "error with fn {}: &self receiver on published fn's must be immutable",
                        fn_name
                    );
                }
            }
            syn::FnArg::Typed(pat_type) => {
                match pat_type.pat.as_ref() {
                    syn::Pat::Ident(ident) => {
                        let mut ty = pat_type.ty.clone();
                        match ty.as_mut() {
                            Type::Reference(tyref) => {
                                if let Type::Path(path) = tyref.elem.borrow() {
                                    if path.path.segments.first().unwrap().ident == "Context" {
                                        context_name = Some(ident.ident.clone());
                                        new_inputs.push(field.clone());
                                        if tyref.mutability.is_none() {
                                            is_query = true;
                                        }
                                        continue;
                                    }

                                    if let Some(s) = path.path.segments.first() {
                                        if s.ident == "EventBus" {
                                            fn_ctr_args.push(quote! { &mut Default::default(), });
                                            continue;
                                        }
                                    }
                                }

                                have_lifetimes = true;
                                assert!(
                                    tyref.lifetime.is_none()
                                        || tyref.lifetime.as_ref().unwrap().ident == "a",
                                    "lifetime must be either unnamed or called 'a"
                                );
                                tyref.lifetime = Some(parse_quote!('a));
                            }
                            Type::Path(path) => {
                                if let Some(s) = path.path.segments.first() {
                                    if s.ident == "EventBus" {
                                        fn_ctr_args.push(quote! { Default::default(), });
                                        continue;
                                    }
                                }
                            }
                            _ => {}
                        }
                        msg_fields.push(quote! {
                            pub #ident: #ty,
                        });
                        msg_deconstruct.push(quote! {
                            #ident,
                        });
                        fn_ctr_args.push(quote! {
                            #ident,
                        });
                        msg_fields_init.push(quote! {
                            #ident,
                        });
                    }
                    _ => bail!("expected identifier"),
                };
            }
            _ => {}
        }
        new_inputs.push(field.clone());
    }
    signature.inputs = new_inputs;
    let opt_lifetime = if have_lifetimes {
        quote! { <'a> }
    } else {
        quote! {}
    };
    let opt_underscore_lifetime = if have_lifetimes {
        quote! { <'_> }
    } else {
        quote! {}
    };

    push_item(
        &mut builder.items,
        quote! {
            #[derive(::ixc::SchemaValue, Default)]
            #[sealed]
            pub struct #msg_struct_name #opt_lifetime {
                #(#msg_fields)*
            }
        },
    )?;
    let selector = message_selector_from_str(msg_struct_name.to_string().as_str());
    let return_type = match &signature.output {
        ReturnType::Type(_, ty) => ty,
        ReturnType::Default => {
            bail!("expected return type")
        }
    };
    if publish_target.on_create.is_none() {
        push_item(
            &mut builder.items,
            quote! {
                impl <'a> ::ixc::core::message::Message<'a> for #msg_struct_name #opt_lifetime {
                    const SELECTOR: ::ixc::message_api::header::MessageSelector = #selector;
                    type Response<'b> = <#return_type as ::ixc::core::message::ExtractResponseTypes>::Response;
                    type Error = <#return_type as ::ixc::core::message::ExtractResponseTypes>::Error;
                    type Codec = ::ixc::schema::binary::NativeBinaryCodec;
                }
            },
        )?;
        ensure!(context_name.is_some(), "no context parameter found");
        let context_name = context_name.unwrap();
        let maybe_mut = if is_query {
            quote! { mut }
        } else {
            quote! {}
        };
        let route = quote! {
                    (< #msg_struct_name #opt_underscore_lifetime as ::ixc::core::message::Message >::SELECTOR, |h: &Self, packet, cb, a| {
                        unsafe {
                            let cdc = < #msg_struct_name as ::ixc::core::message::Message<'_> >::Codec::default();
                            let header = packet.header();
                            let in1 = header.in_pointer1.get(packet);
                            let mem = ::ixc::schema::mem::MemoryManager::new();
                            let #msg_struct_name { #(#msg_deconstruct)* } = ::ixc::schema::codec::decode_value::< #msg_struct_name >(&cdc, in1, &mem)?;
                            let #maybe_mut ctx = ::ixc::core::Context::new_with_mem(header.account, header.caller, header.gas_left, cb, &mem);
                            let res = h.#fn_name(& #maybe_mut ctx, #(#fn_ctr_args)*);
                            ::ixc::core::low_level::encode_response::< #msg_struct_name >(&cdc, res, a, packet)
                        }
                    }),
        };
        builder.routes.push(route);
        signature.output = parse_quote! {
            -> <#return_type as ::ixc::core::message::ExtractResponseTypes>::ClientResult
        };
        builder.client_signatures.push(signature.clone());
        let dynamic_invoke = if is_query {
            quote! { #context_name.dynamic_invoke_query(_acct_id, _msg) }
        } else {
            quote! { #context_name.dynamic_invoke_msg(_acct_id, _msg) }
        };
        builder.client_methods.push(quote! {
                #signature {
                    let _msg = #msg_struct_name {
                        #(#msg_fields_init)*
                    };
                    let _acct_id = ::ixc::core::handler::Client::account_id(self);
                    unsafe {
                        #dynamic_invoke
                    }
                }
        });
    } else {
        builder.system_routes.push(quote! {
            (::ixc::core::account_api::ON_CREATE_SELECTOR, | h: &Self, packet, cb, a| {
                unsafe {
                    let cdc = < #msg_struct_name #opt_underscore_lifetime as::ixc::core::handler::InitMessage<'_> >::Codec::default();
                    let header = packet.header();
                    let in1 = header.in_pointer1.get(packet);
                    let mem = ::ixc::schema::mem::MemoryManager::new();
                    let #msg_struct_name { #(#msg_deconstruct)* } = ::ixc::schema::codec::decode_value::< #msg_struct_name > ( & cdc, in1, &mem)?;
                    let mut ctx =::ixc::core::Context::new_with_mem(header.account, header.caller, header.gas_left, cb, &mem);
                    let res = h.#fn_name(&mut ctx, #(#fn_ctr_args)*);
                    ::ixc::core::low_level::encode_default_response(res, a, packet)
                }
            }),}
        );
        builder.create_msg_name = Some(msg_struct_name);
        builder.create_msg_lifetime = opt_lifetime;
    }
    Ok(())
}
