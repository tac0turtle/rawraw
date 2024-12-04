use proc_macro::TokenStream;
use blake2::{Blake2b512, Digest};
use quote::quote;
use syn::{parse_macro_input, LitStr};
use syn::__private::TokenStream2;

pub fn message_selector(item: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(item as LitStr);
    message_selector_from_str(&input_str.value()).into()
}

pub(crate) fn message_selector_from_str(msg: &str) -> TokenStream2 {
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
