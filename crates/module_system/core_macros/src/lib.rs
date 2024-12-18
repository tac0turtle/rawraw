//! **WARNING: This is an API preview! Expect major bugs, glaring omissions, and breaking changes!**
//! This is a macro utility crate for ixc_core.
#![allow(unused)]

mod api_builder;
mod handler;
mod handler_api;
mod message_selector;
mod migration;
mod resources;
mod util;

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

/// Derives a handler implementation for a module.
///
/// This macro must be applied to a module and generates the necessary
/// implementation details for handling messages and queries.
///
/// # Example
/// ```rust
/// #[handler(MyHandler)]
/// mod my_handler {
///     // Handler implementation
/// }
/// ```
#[manyhow]
#[proc_macro_attribute]
pub fn handler(attr: TokenStream2, mut item: ItemMod) -> manyhow::Result<TokenStream2> {
    handler::handler(attr, item)
}

/// Defines a handler API trait implementation.
///
/// This attribute macro should be attached to a trait that implements a handler API.
/// It generates the necessary routing and implementation code for handling messages
/// and queries defined in the trait.
///
/// # Features
/// - Automatically implements message routing for trait methods
/// - Generates necessary boilerplate for handler communication
/// - Supports both synchronous and asynchronous methods
///
/// # Example
/// ```rust
/// #[handler_api]
/// pub trait UserAPI {
///     /// Creates a new user
///     fn create_user(&self, username: String, email: String) -> Result<User, Error>;
///
///     /// Retrieves user information
///     fn get_user(&self, user_id: u64) -> Result<Option<User>, Error>;
/// }
/// ```
///
/// # Notes
/// - The trait must be public (`pub`)
/// - Methods should return `Result` types for error handling
/// - Method parameters must implement necessary serialization traits
#[manyhow]
#[proc_macro_attribute]
pub fn handler_api(attr: TokenStream2, item_trait: ItemTrait) -> manyhow::Result<TokenStream2> {
    handler_api::handler_api(attr, item_trait)
}

/// This publishes a trait or struct impl block or a single fn within an impl block.
#[manyhow]
#[proc_macro_attribute]
pub fn publish(_attr: TokenStream2, _item: TokenStream2) -> manyhow::Result<TokenStream2> {
    bail!("the #[publish] attribute is being used in the wrong context, possibly #[handler] has not been applied to the enclosing module")
}

/// This attribute macro should be attached to the fn which is called when an account is created.
#[manyhow]
#[proc_macro_attribute]
pub fn on_create(_attr: TokenStream2, _item: TokenStream2) -> manyhow::Result<TokenStream2> {
    bail!("the #[on_create] attribute is being used in the wrong context, possibly #[handler] has not been applied to the enclosing module")
}

/// This attribute macro should be attached functions are called when an
/// account has been migrated to new handler.
///
/// It requires a #[from] parameter to specify the handler from
/// which the account is being migrated.
/// Parameters annotated with #[from] must be borrowed references
/// to handler structs or any struct that implements [`ixc::core::handler::HandlerResources`].
/// This makes it possible to migrate an account to a new handler
/// while reading the state of the old handler,
/// and only retaining the handler struct itself rather than all the old implementation code.
///
/// A unique migration function should be defined for each handler
/// from which the account can be migrated.
#[manyhow]
#[proc_macro_attribute]
pub fn on_migrate(_attr: TokenStream2, _item: TokenStream2) -> manyhow::Result<TokenStream2> {
    bail!("the #[on_migrate] attribute is being used in the wrong context, possibly #[handler] has not been applied to the enclosing module")
}

/// This attribute macro should be used on the parameter of functions
/// annotated with #[on_migrate] to the handler from
/// which the account is being migrated.
/// The type of this parameter must be a reference to a handler struct
/// which implements [ixc::core::handler::HandlerResources].
/// This struct is used to both extract the name of the handler from
/// and can be used to read state from the old handler.
/// This attribute must be attached to exactly one parameter in on #[on_migrate] function.
#[manyhow]
#[proc_macro_attribute]
pub fn from(_attr: TokenStream2, _item: TokenStream2) -> manyhow::Result<TokenStream2> {
    bail!("the #[from] attribute is being used in the wrong context, possibly #[handler] has not been applied to the enclosing module")
}

/// Derives the `Resources` trait for a struct.
///
/// This macro implements the Resources trait for the annotated struct,
/// allowing it to be used as a handler resource. It processes the following attributes:
///
/// - `#[state]`: Marks fields that represent handler state
/// - `#[client]`: Marks fields that represent client connections
///
/// # Example
/// ```rust
/// #[derive(Resources)]
/// struct MyHandler {
///     #[state]
///     counter: u32,
///     #[client]
///     connection: Client,
/// }
/// ```
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
