//! **WARNING: This is an API preview! Most code won't work or even type check properly!**
//! State objects projects a state management framework that works well with interchain_core.

extern crate alloc;

pub mod accumulator;
mod item;
mod map;
mod prefix;
mod store_client;

pub use accumulator::{Accumulator, AccumulatorMap};
pub use item::Item;
pub use map::Map;
