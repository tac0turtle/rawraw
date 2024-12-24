use manyhow::bail;
use proc_macro2::Ident;
use quote::quote;
use syn::Attribute;
use proc_macro2::TokenStream as TokenStream2;

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