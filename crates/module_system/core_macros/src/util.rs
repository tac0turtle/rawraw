use syn::{parse2, Item};
use proc_macro2::{TokenStream as TokenStream2};

pub(crate) fn push_item(items: &mut Vec<Item>, expanded: TokenStream2) -> manyhow::Result<()> {
    items.push(parse2::<Item>(expanded)?);
    Ok(())
}

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

