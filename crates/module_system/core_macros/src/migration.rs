use crate::api_builder::APIBuilder;
use crate::handler::{FromAttr, OnMigrateAttr, PublishedFnInfo, PublishedFnType};
use crate::util::maybe_extract_attribute;
use core::borrow::Borrow;
use manyhow::bail;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, ImplItemFn, Type};

/// Collects the information from the #[on_migrate] attribute.
pub(crate) fn collect_on_migrate_info(
    item_fn: &mut ImplItemFn,
    attr: OnMigrateAttr,
) -> manyhow::Result<OnMigrateInfo> {
    let fn_name = item_fn.sig.ident.clone();
    if item_fn.sig.inputs.len() != 3 {
        bail!("expected exactly 3 arguments in on_migrate function");
    }

    // match &self
    match &item_fn.sig.inputs[0] {
        FnArg::Receiver(r) => {
            if r.mutability.is_some() {
                bail!(
                    "error with fn {}: &self receiver on on_migrate function must be immutable",
                    fn_name
                );
            }
        }
        _ => bail!(
            "error with fn {}: the first argument of on_migrate function must be &self",
            fn_name
        ),
    }

    // match &mut Context
    match &item_fn.sig.inputs[1] {
        FnArg::Typed(pat_type) => match pat_type.ty.as_ref() {
            Type::Reference(tyref) => {
                if let Type::Path(path) = tyref.elem.borrow() {
                    if path.path.segments.first().unwrap().ident != "Context" {
                        bail!("error with fn {}: the second argument of on_migrate function must be &mut Context", fn_name);
                    }
                } else {
                    bail!("error with fn {}: the second argument of on_migrate function must be &mut Context", fn_name);
                }
                if tyref.mutability.is_none() {
                    bail!("error with fn {}: the second argument of on_migrate function must be &mut Context", fn_name);
                }
            }
            _ => bail!(
                "error with fn {}: the second argument of on_migrate function must be &mut Context",
                fn_name
            ),
        },
        _ => bail!(
            "error with fn {}: the second argument of on_migrate function must be &mut Context",
            fn_name
        ),
    }

    // extract #[from] attribute and handler type
    let from = match &mut item_fn.sig.inputs[2] {
        FnArg::Typed(pat_type) => {
            let from_attr: Option<FromAttr> = maybe_extract_attribute(pat_type)?;
            if from_attr.is_some() {
                match pat_type.ty.as_ref() {
                    Type::Reference(tyref) => {
                        tyref.elem.clone()
                    }
                    _ => bail!("error with fn {}: the #[from] attribute must be attached to a reference to the handler from which the account is migrating", fn_name),
                }
            } else {
                bail!("error with fn {}: the #[from] attribute must be attached to parameter which takes a reference to the handler from which the account is migrating", fn_name);
            }
        },
        _ => bail!("error with fn {}: the third argument of on_migrate function must be the old handler reference with the #[from] attribute", fn_name),
    };

    Ok(OnMigrateInfo { from, attr })
}

pub(crate) fn build_on_migrate_handler(
    builder: &mut APIBuilder,
    published_fn_info: &[PublishedFnInfo],
) -> manyhow::Result<()> {
    let mut cases = vec![];
    for fn_info in published_fn_info {
        if let PublishedFnType::OnMigrate(info) = &fn_info.ty {
            let fn_name = &fn_info.signature.ident;
            let OnMigrateInfo { from, .. } = info;
            cases.push(quote! {
                <#from as ::ixc::core::handler::HandlerResources>::NAME => {
                    let old_handler = <#from as ::ixc::core::resource::Resources>::new(&scope)
                        .map_err(|_| ::ixc::message_api::error::HandlerError::new(::ixc::message_api::code::ErrorCode::SystemCode(::ixc::message_api::code::SystemCode::InvalidHandler)))?;
                    h.#fn_name(&mut ctx, &old_handler)
                },
            });
        }
    }
    if !cases.is_empty() {
        builder.system_routes.push(quote! {
                (::ixc::core::account_api::ON_MIGRATE_SELECTOR, | h: & Self, caller, packet, cb, a | {
                    unsafe {
                       let old_handler_id = packet.request().in1().expect_string()?;
                        let mem =::ixc::schema::mem::MemoryManager::new();
                        let mut ctx =::ixc::core::Context::new_mut(&packet.target_account(), caller, cb, &mem);
                        let scope: ::ixc::core::resource::ResourceScope<'_> = ::core::default::Default::default();
                        let res = match old_handler_id {
                            #(#cases)*
                            _ => return Err(::ixc::message_api::code::ErrorCode::SystemCode(::ixc::message_api::code::SystemCode::MessageNotHandled).into()),
                        };
                        ::ixc::core::low_level::encode_default_response(res)
                    }
                })
            });
    }
    Ok(())
}

#[derive(Debug)]
pub(crate) struct OnMigrateInfo {
    pub(crate) from: Box<Type>,
    pub(crate) attr: OnMigrateAttr,
}
