//! Handler traits for account and module handlers.

use crate::message::InitMessage;
use crate::resource::Resources;
use crate::routing::Router;
use ixc_message_api::handler::RawHandler;
use ixc_message_api::AccountID;
use ixc_schema::message::MessageDescriptor;
use ixc_schema::types::TypeVisitor;

/// Handler trait for account and module handlers.
pub trait Handler: RawHandler + Router + HandlerResources + Service {
    /// The parameter used for initializing the handler.
    type Init<'a>: InitMessage<'a>;

    /// Visit the schema of the handler.
    fn visit_schema<'a, V: APISchemaVisitor<'a>>(visitor: &mut V);
}

/// The resources associated with a handler. This specifies the name of the handler.
pub trait HandlerResources: Resources {
    /// The name of the handler.
    const NAME: &'static str;
}

/// A type that represents some sort of handler service and thus has an associated client type.
/// This is used to create clients for the service.
pub trait Service {
    /// The client type associated with the service.
    type Client: Client;

    /// Create a new client for the service with the given account ID.
    fn new_client(account_id: AccountID) -> Self::Client {
        Self::Client::new(account_id)
    }
}

/// The trait that clients of a service must implement.
pub trait Client {
    /// Create a new client with the given account ID.
    fn new(account_id: AccountID) -> Self;

    /// Get the address of the account that this client sends messages to.
    fn target_account(&self) -> AccountID;

    /// Visit the schema of the client.
    fn visit_schema<'a, V: APISchemaVisitor<'a>>(visitor: &mut V);
}

/// The client of a handler.
pub trait HandlerClient: Client {
    /// The handler type.
    type Handler: Handler;
}

/// A visitor for the schema of a client.
pub trait APISchemaVisitor<'a>: TypeVisitor {
    /// Visit the client's messages.
    fn visit_message(&mut self, messages: &MessageDescriptor<'a>);
}
