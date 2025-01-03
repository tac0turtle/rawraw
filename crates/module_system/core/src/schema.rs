//! Schema extraction and printing utilities.
extern crate std;
use crate::handler::{APISchemaVisitor, Client, Handler};
use crate::resource::ResourcesVisitor;
use alloc::string::{String, ToString};
use allocator_api2::alloc::Allocator;
use allocator_api2::vec::Vec;
use ixc_message_api::AccountID;
use ixc_schema::client::ClientDescriptor;
use ixc_schema::handler::HandlerSchema;
use ixc_schema::json;
use ixc_schema::list::List;
use ixc_schema::mem::MemoryManager;
use ixc_schema::message::MessageDescriptor;
use ixc_schema::state_object::StateObjectDescriptor;
use ixc_schema::types::{Type, TypeCollector, TypeVisitor};

/// Extract the schema of the handler.
pub fn extract_handler_schema<H: Handler>(
    allocator: &dyn Allocator,
) -> Result<HandlerSchema, String> {
    struct Visitor<'b> {
        allocator: &'b dyn Allocator,
        type_collector: TypeCollector<'b>,
        messages: Vec<MessageDescriptor<'b>, &'b dyn Allocator>,
        state_objects: Vec<StateObjectDescriptor<'b>, &'b dyn Allocator>,
        clients: Vec<ClientDescriptor<'b>, &'b dyn Allocator>,
    }
    impl TypeVisitor for Visitor<'_> {
        fn visit<T: Type>(&mut self) {
            self.type_collector.visit::<T>();
        }
    }
    impl<'b> APISchemaVisitor<'b> for Visitor<'b> {
        fn allocator(&self) -> &'b dyn Allocator {
            self.allocator
        }

        fn visit_message(&mut self, messages: &MessageDescriptor<'b>) {
            self.messages.push(messages.clone());
        }
    }
    impl<'b> ResourcesVisitor<'b> for Visitor<'b> {
        fn allocator(&self) -> &'b dyn Allocator {
            self.allocator
        }

        fn visit_state_object(&mut self, state_object: &StateObjectDescriptor<'b>) {
            self.state_objects.push(state_object.clone());
        }

        fn visit_client<C: Client>(&mut self, name: &'b str, account_id: &AccountID) {
            struct ClientVisitor<'c, 'd> {
                allocator: &'c dyn Allocator,
                types: &'d mut TypeCollector<'c>,
                messages: Vec<MessageDescriptor<'c>, &'c dyn Allocator>,
            }
            impl TypeVisitor for ClientVisitor<'_, '_> {
                fn visit<T: Type>(&mut self) {
                    self.types.visit::<T>();
                }
            }
            impl<'c> APISchemaVisitor<'c> for ClientVisitor<'c, '_> {
                fn allocator(&self) -> &'c dyn Allocator {
                    self.allocator
                }

                fn visit_message(&mut self, messages: &MessageDescriptor<'c>) {
                    self.messages.push(messages.clone());
                }
            }
            let mut client_visitor = ClientVisitor {
                allocator: self.allocator,
                types: &mut self.type_collector,
                messages: Vec::new_in(self.allocator),
            };
            C::visit_schema(&mut client_visitor);
            let mut desc = ClientDescriptor::new(name, *account_id);
            desc.messages = List::Owned(client_visitor.messages);
            self.clients.push(desc);
        }
    }
    let mut visitor = Visitor {
        allocator,
        type_collector: TypeCollector::new(allocator),
        messages: Vec::new_in(allocator),
        state_objects: Vec::new_in(allocator),
        clients: Vec::new_in(allocator),
    };
    H::visit_schema(&mut visitor);
    H::visit_resources(&mut visitor);
    if !visitor.type_collector.errors.is_empty() {
        return Err(visitor
            .type_collector
            .errors
            .iter()
            .as_slice()
            .join("\n")
            .to_string());
    }
    let mut types = Vec::new_in(allocator);
    // for (_, ty) in visitor.type_collector.types.drain() {
    //     types.push(ty);
    // }
    todo!();
    let mut res = HandlerSchema::default();
    res.types = List::Owned(types);
    res.messages = List::Owned(visitor.messages);
    res.state_objects = List::Owned(visitor.state_objects);
    res.clients = List::Owned(visitor.clients);
    Ok(res)
}

/// Dump the schema of the handler to stdout as JSON.
pub fn print_handler_schema<H: Handler>() -> Result<(), String> {
    let mem = MemoryManager::new();
    let schema = extract_handler_schema::<H>(&mem)?;
    let mut out = Vec::new();
    json::encode_value(&schema, &mut out).map_err(|e| e.to_string())?;
    std::println!(
        "{}",
        std::str::from_utf8(&out).map_err(|_| "invalid utf-8")?
    );
    Ok(())
}
