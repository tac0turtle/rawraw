//! **WARNING: This is an API preview! Expect major bugs, glaring omissions, and breaking changes!**
//! This is a macro utility crate for ixc_core.
#![allow(unused)]

mod message_selector;
mod handler;
mod handler_api;
mod api_builder;
mod util;
mod resources;

//TODO remove
use blake2::{Blake2b512, Digest};
use heck::ToUpperCamelCase;
use manyhow::{bail, ensure, manyhow};
use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use std::borrow::Borrow;
use std::default::Default;
use syn::punctuated::Punctuated;
use syn::{
    parse2, parse_macro_input, parse_quote, Attribute, Data, DeriveInput, Item, ItemMod, ItemTrait,
    LitStr, ReturnType, Signature, TraitItem, Type,
};

/// This derives an account handler.
#[manyhow]
#[proc_macro_attribute]
pub fn handler(attr: TokenStream2, mut item: ItemMod) -> manyhow::Result<TokenStream2> {
    handler::handler(attr, item)
}

/// This attribute macro should be attached to a trait that implements a handler API.
#[manyhow]
#[proc_macro_attribute]
pub fn handler_api(attr: TokenStream2, item_trait: ItemTrait) -> manyhow::Result<TokenStream2> {
    handler_api::handler_api(attr, item_trait)
}

/// This publishes a trait or struct impl block or a single fn within an impl block.
#[manyhow]
#[proc_macro_attribute]
pub fn publish(_attr: TokenStream2, _item: TokenStream2) -> manyhow::Result<TokenStream2> {
    bail!("the #[publish] attribute is being used in the wrong context, possibly #[module_handler] or #[account_handler] has not been applied to the enclosing module")
}

/// This attribute macro should be attached to the fn which is called when an account is created.
#[manyhow]
#[proc_macro_attribute]
pub fn on_create(_attr: TokenStream2, _item: TokenStream2) -> manyhow::Result<TokenStream2> {
    bail!("the #[on_create] attribute is being used in the wrong context, possibly #[module_handler] or #[account_handler] has not been applied to the enclosing module")
}

/// Derive the `Resources` trait for a struct.
#[manyhow]
#[proc_macro_derive(Resources, attributes(state, client))]
pub fn derive_resources(input: DeriveInput) -> manyhow::Result<TokenStream2> {
    resources::derive_resources(input)
}

/// Creates the message selector for the given message name.
#[proc_macro]
pub fn message_selector(item: TokenStream) -> TokenStream {
    message_selector::message_selector(item)
}

