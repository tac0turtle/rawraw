#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

// prop-test has problems with no_std, so we disable it during tests
#![cfg_attr(not(test), no_std)]

#[cfg(feature = "std")]
extern crate alloc;

// this is to allow this crate to use its own macros
#[cfg(feature = "use_ixc_macro_path")]
extern crate self as ixc;
#[cfg(not(feature = "use_ixc_macro_path"))]
extern crate self as ixc_schema;

pub mod binary;
pub mod buffer;
mod bump;
pub mod codec;
pub mod decoder;
pub mod encoder;
mod enums;
pub mod field;
mod fields;
mod json;
pub mod kind;
pub mod list;
pub mod mem;
mod message;
pub mod schema;
pub mod state_object;
pub mod structs;
pub mod types;
pub mod value;

pub use state_object::{Bytes, Str};
pub use value::SchemaValue;
