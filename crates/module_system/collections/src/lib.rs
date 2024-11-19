//! **WARNING: This is an API preview! Most code won't work or even type check properly!**
//! State objects projects a state management framework that works well with interchain_core.

extern crate alloc;
extern crate core;

mod errors;
mod item;
mod map;
mod set;

pub use map::Map;
// pub use set::{Set};
pub use item::Item;
