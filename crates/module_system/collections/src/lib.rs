//! **WARNING: This is an API preview! Most code won't work or even type check properly!**
//! State objects projects a state management framework that works well with interchain_core.

#![feature(concat_bytes)]
extern crate alloc;

mod errors;
mod item;
mod map;
mod store_client;
// mod uint_map;
pub mod accumulator;

pub use accumulator::{Accumulator, AccumulatorMap};
pub use item::Item;
pub use map::Map;
