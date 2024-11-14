#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[doc(inline)]
pub use ixc_core::resource::Resources;
#[doc(inline)]
pub use ixc_core::{bail, create_account, ensure, error, Context, EventBus, Result, Service};

#[doc(inline)]
pub use ixc_message_api::AccountID;
#[doc(inline)]
pub use ixc_schema::{Bytes, Str};
#[doc(inline)]
pub use simple_time::{Duration, Time};
#[doc(inline)]
pub use state_objects::{Accumulator, AccumulatorMap, Item, Map};

pub use ixc_core as core;
pub use ixc_message_api as message_api;
pub use ixc_schema as schema;

#[allow(unused_imports)]
#[macro_use]
extern crate ixc_core_macros;
#[doc(inline)]
pub use ixc_core_macros::{handler, handler_api, on_create, publish, Resources};

#[allow(unused_imports)]
#[macro_use]
extern crate ixc_schema_macros;
#[doc(inline)]
pub use ixc_schema_macros::*;

#[allow(unused_imports)]
#[macro_use]
extern crate state_objects_macros;
#[doc(inline)]
pub use state_objects_macros::*;
