use manyhow::bail;
use proc_macro2::Ident;
use quote::quote;
use syn::Attribute;
use proc_macro2::TokenStream as TokenStream2;

/// Extracts and parses an attribute from the item if it is present.
/// This is a helper function for working around this bug in deluxe: https://github.com/jf2048/deluxe/issues/24
pub(crate) fn maybe_extract_attribute<T, R>(t: &mut T) -> manyhow::Result<Option<R>>
where
    T: deluxe::HasAttributes,
    R: deluxe::ExtractAttributes<T>,
{
    let mut have_attr = false;
    for attr in t.attrs() {
        if R::path_matches(attr.meta.path()) {
            have_attr = true;
        }
    }
    if !have_attr {
        return Ok(None);
    }
    Ok(Some(R::extract_attributes(t)?))
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