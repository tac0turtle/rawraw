use manyhow::bail;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Attribute, ImplGenerics, Lifetime, TypeGenerics};

pub(crate) fn has_attribute<I>(attrs: &Vec<Attribute>, ident: &I) -> bool
where
    I: ?Sized,
    Ident: PartialEq<I>,
{
    for attr in attrs {
        if attr.path().is_ident(ident) {
            return true;
        }
    }
    false
}

// this is to make macros work with a single import of the ixc crate
pub(crate) fn mk_ixc_schema_path() -> TokenStream2 {
    #[cfg(feature = "use_ixc_macro_path")]
    quote! {::ixc::schema}
    #[cfg(not(feature = "use_ixc_macro_path"))]
    quote! {::ixc_schema}
}

pub(crate) fn is_sealed(input: &syn::DeriveInput) -> manyhow::Result<bool> {
    let sealed = has_attribute(&input.attrs, "sealed");
    let non_exhaustive = has_attribute(&input.attrs, "non_exhaustive");
    if !sealed && !non_exhaustive {
        bail!("struct or enum must have either a #[sealed] or #[non_exhaustive] attribute to indicate whether adding new fields is or is not a breaking change. Only sealed structs can be used as input types and cannot have new fields added.")
    };
    if sealed && non_exhaustive {
        bail!("struct or enum cannot be both sealed and non_exhaustive")
    };
    Ok(sealed)
}

pub(crate) fn extract_generics(input: &syn::DeriveInput) -> manyhow::Result<GenericInfo> {
    let generics = &input.generics;
    if generics.lifetimes().count() > 1 {
        bail!("only one lifetime parameter is allowed")
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let lifetime = if let Some(lifetime) = generics.lifetimes().next() {
        lifetime.lifetime.clone()
    } else {
        Lifetime::new("'a", Span::call_site())
    };
    let lifetime2 = if lifetime.ident == "b" {
        Lifetime::new("'c", Span::call_site())
    } else {
        Lifetime::new("'b", Span::call_site())
    };
    let ty_generics2 = if let Some(_lifetime) = generics.lifetimes().next() {
        quote! { < #lifetime2 > }
    } else {
        quote! {}
    };
    Ok(GenericInfo {
        lifetime,
        lifetime2,
        ty_generics2,
        impl_generics,
        where_clause,
        ty_generics,
    })
}

pub(crate) struct GenericInfo<'a> {
    pub(crate) lifetime: Lifetime,
    pub(crate) lifetime2: Lifetime,
    pub(crate) ty_generics2: TokenStream,
    pub(crate) impl_generics: ImplGenerics<'a>,
    pub(crate) where_clause: Option<&'a syn::WhereClause>,
    pub(crate) ty_generics: TypeGenerics<'a>,
}
