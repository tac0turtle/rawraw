//! **WARNING: This is an API preview! Most code won't work or even type check properly!**
//! State objects projects a state management framework that works well with interchain_core.

#![feature(concat_bytes)]
extern crate alloc;

mod item;
mod map;
mod store_client;
pub mod accumulator;
mod prefix;

pub use accumulator::{Accumulator, AccumulatorMap};
pub use item::Item;
pub use map::Map;
