use blake2::{Blake2b512, Digest};
use proc_macro2::TokenStream;
use quote::quote;

/// Generate a type selector from a string based on the first 8 bytes of the blake2b512 hash of the string.
/// This is a duplicate of the logic in ixc_schema_macros.
pub(crate) fn type_selector_from_str(msg: &str) -> TokenStream {
    let mut hasher = Blake2b512::new(); // TODO should we use 256 or 512?
    hasher.update(msg.as_bytes());
    let res = hasher.finalize();
    // take first 8 bytes and convert to u64
    let hash = u64::from_le_bytes(res[..8].try_into().unwrap());
    let expanded = quote! {
        #hash
    };
    expanded
}
