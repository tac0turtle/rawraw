//! Schema extraction and printing utilities.
use crate::handler::{APISchemaVisitor, Client, Handler};
use crate::resource::ResourcesVisitor;
use alloc::string::{String, ToString};
use ixc_schema::client::ClientDescriptor;
use ixc_schema::handler::HandlerSchema;
use ixc_schema::json;
use ixc_schema::message::MessageDescriptor;
use ixc_schema::state_object::StateObjectDescriptor;
use ixc_schema::types::{Type, TypeCollector, TypeVisitor};

extern crate std;

/// Extract the schema of the handler.
pub fn extract_handler_schema<'a, H: Handler>() -> Result<HandlerSchema<'a>, String> {
    #[derive(Default)]
    struct Visitor<'b> {
        type_collector: TypeCollector,
        messages: alloc::vec::Vec<MessageDescriptor<'b>>,
        state_objects: alloc::vec::Vec<StateObjectDescriptor<'b>>,
        clients: alloc::vec::Vec<ClientDescriptor<'b>>,
    }
    impl TypeVisitor for Visitor<'_> {
        fn visit<T: Type>(&mut self) {
            self.type_collector.visit::<T>();
        }
    }
    impl<'b> APISchemaVisitor<'b> for Visitor<'b> {
        fn visit_message(&mut self, messages: &MessageDescriptor<'b>) {
            self.messages.push(messages.clone());
        }
    }
    impl<'b> ResourcesVisitor<'b> for Visitor<'b> {
        fn visit_state_object(&mut self, state_object: &StateObjectDescriptor<'b>) {
            self.state_objects.push(state_object.clone());
        }

        fn visit_client<C: Client>(&mut self, desc: &ClientDescriptor<'b>) {
            struct ClientVisitor<'c, 'd> {
                types: &'d mut TypeCollector,
                desc: ClientDescriptor<'c>,
            }
            impl<'c, 'd> TypeVisitor for ClientVisitor<'c, 'd> {
                fn visit<T: Type>(&mut self) {
                    self.types.visit::<T>();
                }
            }
            impl<'c, 'd> APISchemaVisitor<'c> for ClientVisitor<'c, 'd> {
                fn visit_message(&mut self, messages: &MessageDescriptor<'c>) {
                    self.desc.messages.push(messages.clone());
                }
            }
            let mut client_visitor = ClientVisitor {
                types: &mut self.type_collector,
                desc: desc.clone(),
            };
            C::visit_schema(&mut client_visitor);
            self.clients.push(client_visitor.desc);
        }
    }
    let mut visitor = Visitor::default();
    H::visit_schema(&mut visitor);
    H::visit_resources(&mut visitor);
    let type_map = visitor
        .type_collector
        .finish()
        .map_err(|errors| errors.iter().as_slice().join("\n").to_string())?;
    let mut res = HandlerSchema::default();
    res.types = type_map.values().cloned().collect();
    res.messages = visitor.messages;
    res.state_objects = visitor.state_objects;
    res.clients = visitor.clients;
    Ok(res)
}

/// Dump the schema of the handler to stdout as JSON.
pub fn print_handler_schema<'a, H: Handler>() -> Result<(), String> {
    let schema = extract_handler_schema::<H>()?;
    let res = json::encode_value(&schema).map_err(|e| e.to_string())?;
    std::println!("{}", res);
    Ok(())
}
