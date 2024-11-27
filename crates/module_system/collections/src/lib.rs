//! **WARNING: This is an API preview! Most code won't work or even type check properly!**
//! State objects projects a state management framework that works well with interchain_core.

extern crate alloc;

mod errors;
mod item;
mod map;
mod store_client;
// mod uint_map;
pub mod accumulator;
mod ordered_map;

pub use map::Map;
// pub use set::{Set};
pub use item::Item;
// pub use index::{Index};
// pub use unique::{UniqueIndex};
pub use accumulator::{Accumulator, AccumulatorMap};
// pub use ordered_map::{OrderedMap};
// pub use ordered_set::{OrderedSet};
