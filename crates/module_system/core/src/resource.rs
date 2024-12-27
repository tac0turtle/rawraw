//! Resource module.

use crate::handler::Client;
use crate::{ClientFactory, Service};
use allocator_api2::alloc::Allocator;
use allocator_api2::vec::Vec;
use ixc_message_api::AccountID;
use ixc_schema::list::List;
use ixc_schema::state_object::StateObjectDescriptor;
use ixc_schema::types::TypeVisitor;

/// An account or module handler's resources.
/// This is usually derived by the state management framework.
/// # Safety
/// The trait is marked as unsafe because only macros should implement it.
pub unsafe trait Resources: Sized {
    /// Initializes the resources.
    /// # Safety
    /// The function is marked as unsafe to detour users from calling it directly
    unsafe fn new(scope: &ResourceScope) -> Result<Self, InitializationError>;

    /// Visit the resources to discover their schema.
    fn visit_resources<'a, V: ResourcesVisitor<'a>>(visitor: &mut V);
}

/// The resource scope.
#[derive(Default)]
pub struct ResourceScope<'a> {
    /// The prefix of all state objects under this scope.
    pub state_scope: &'a [u8],

    /// The optional runtime account resolver.
    pub account_resolver: Option<&'a dyn AccountResolver>,
}

/// Resolves account names to account IDs.
pub trait AccountResolver {
    /// Resolves an account name to an account ID.
    fn resolve(&self, name: &str) -> Result<AccountID, InitializationError>;
}

#[cfg(feature = "std")]
impl AccountResolver for alloc::collections::BTreeMap<&str, AccountID> {
    fn resolve(&self, name: &str) -> Result<AccountID, InitializationError> {
        self.get(name)
            .copied()
            .ok_or(InitializationError::AccountNotFound)
    }
}

/// A resource is anything that an account or module can use to store its own
/// state or interact with other accounts and modules.
///  # Safety  
/// the trait is marked as unsafe to detour users from creating it
pub unsafe trait StateObjectResource: Sized {
    /// Creates a new resource.
    /// This should only be called in generated code.
    /// Do not call this function directly.
    /// # Safety
    /// the function is marked as unsafe to detour users from calling it directly
    unsafe fn new(scope: &[u8], prefix: u8) -> Result<Self, InitializationError>;

    #[cfg(feature = "std")]
    /// Gets the descriptor for the state object with the supplied names.
    fn descriptor<'a>(
        allocator: &'a dyn Allocator,
        collection_name: &'a str,
        key_names: &[&'a str],
        value_names: &[&'a str],
    ) -> StateObjectDescriptor<'a>;
}

/// An error that occurs during resource initialization.
#[derive(Debug)]
pub enum InitializationError {
    /// An non-specific error occurred.
    Other,
    /// The account with the specified name could not be resolved.
    AccountNotFound,
    /// The length of the scope is too long.
    ExceedsLength,
}

impl ResourceScope<'_> {
    /// Resolves an account name to an account ID or returns a default account ID if provided.
    pub fn resolve_account(
        &self,
        name: &str,
        default: Option<AccountID>,
    ) -> core::result::Result<AccountID, InitializationError> {
        self.account_resolver
            .map(|resolver| resolver.resolve(name))
            .unwrap_or_else(|| default.ok_or(InitializationError::AccountNotFound))
    }
}

/// A visitor for discovering resources.
pub trait ResourcesVisitor<'a>: TypeVisitor {
    /// An allocator that can be used to allocate dynamicresources.
    fn allocator(&self) -> &'a dyn Allocator;
    /// Visit a state object.
    fn visit_state_object(&mut self, state_object: &StateObjectDescriptor<'a>);
    /// Visit a client.
    fn visit_client<C: Client>(&mut self, name: &'a str, account_id: AccountID);
}

/// Extract the state object descriptor for a state object.
/// Used in macros to extract state object schemas.
pub fn extract_state_object_descriptor<'a, R: StateObjectResource, V: ResourcesVisitor<'a>>(
    visitor: &mut V,
    prefix: u8,
    collection_name: &'a str,
    key_names: &'a [&'a str],
    value_names: &'a [&'a str],
) {
    let mut state_object = R::descriptor(visitor.allocator(), collection_name, key_names, value_names);
    let mut prefix_vec = Vec::new_in(visitor.allocator());
    prefix_vec.push(prefix);
    state_object.prefix = List::Owned(prefix_vec);
    visitor.visit_state_object(&state_object);
}

/// Visit a client factory to extract the schema.
/// The signature of this function is a bit of a hack around a possible bug in the quote crate.
pub fn visit_client_factory<'a, S: Service, V: ResourcesVisitor<'a>>(
    _factory: ClientFactory<S>,
    visitor: &mut V,
    name: &'a str,
) {
    visitor.visit_client::<S::Client>(name, AccountID::EMPTY);
}