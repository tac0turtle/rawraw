use proc_macro2::TokenStream as TokenStream2;
use syn::{parse2, Item};

/// Parse an item from a token stream and add it to the items vector.
pub(crate) fn push_item(items: &mut Vec<Item>, expanded: TokenStream2) -> manyhow::Result<()> {
    items.push(parse2::<Item>(expanded)?);
    Ok(())
}

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
