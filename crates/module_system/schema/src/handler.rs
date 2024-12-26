//! Account handler schemas.

use alloc::vec::Vec;
use ixc_schema_macros::SchemaValue;
use crate::client::ClientDescriptor;
use crate::message::MessageDescriptor;
use crate::schema::SchemaType;
use crate::state_object::StateObjectDescriptor;

/// An account handler schema.
#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq, Default, SchemaValue)]
pub struct HandlerSchema<'a> {
    /// The types used by the account in its public API or to interact with other accounts.
    pub types: Vec<SchemaType<'a>>,
    /// The messages that the account handles.
    pub messages: Vec<MessageDescriptor<'a>>,
    /// The state objects defining the account's state.
    pub state_objects: Vec<StateObjectDescriptor<'a>>,
    /// The statically defined clients that the client uses to communicate with other accounts.
    pub clients: Vec<ClientDescriptor<'a>>,
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::json;
    use crate::types::collect_types;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn test_schema_in_schema() {
        // let types_map = collect_types::<SchemaType>().unwrap();
        // let types_vec = types_map.values().cloned().collect::<Vec<_>>();
        // let schema_schema = HandlerSchema {
        //     types: types_vec.as_slice(),
        //     messages: &[],
        //     state_objects: &[],
        //     clients: &[],
        // };
        // let as_json = json::encode_value(&schema_schema).unwrap();
        // println!("{}", as_json);
    }
}
