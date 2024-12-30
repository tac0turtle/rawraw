use crate::frontend::ast::{ConcreteNode, File, Interface, InterfaceItem, Item, ParsedAST};
use crate::frontend::diagnostic::Diagnostic;
use crate::frontend::resolver::definer::ItemDefiner;
use crate::frontend::resolver::ids::{AstPtr, NodeId};
use crate::frontend::resolver::item_ref::ItemPtr;
use crate::frontend::syntax::{IXCLanguage, SyntaxKind, SyntaxNode};
use dashmap::DashMap;
use rowan::ast::AstNode;
use salsa::{Accumulator, Database};
use std::collections::BTreeMap;

#[salsa::tracked]
pub fn resolve_scope<'db>(db: &'db dyn Database, ast: ParsedAST<'db>, node_path: NodeId<'db>) -> Option<Scope<'db>> {
    let registry = init_registry(db);
    let node = node_path.path(db).resolve(&ast.syntax(db))?;
    let mut builder = ScopeBuilder {
        path: node_path,
        scope: Scope::default(),
        db,
    };
    let f = registry.providers.get(&node.kind())?;
    f(node, &mut builder);
    Some(builder.scope)
}

pub fn resolve_name_ref<'db>(db: &'db dyn Database, ast: ParsedAST<'db>, node_id: NodeId<'db>, name_ref: &str) -> Option<ItemPtr<'db>> {
    let mut maybe_node_path = Some(node_id.clone());
    while let Some(ref node_path) = maybe_node_path {
        if let Some(scope) = resolve_scope(db, ast, node_id.clone()) {
            if let Some(item) = scope.names.get(name_ref) {
                return Some(item.clone());
            }
            if let Some(parent) = scope.parent {
                return resolve_name_ref(db, ast, parent, name_ref);
            }
        } else {
            maybe_node_path = node_path.parent_path(db);
        }
    }
    None
}

#[derive(Default)]
pub struct ScopeProviderRegistry {
    providers: DashMap<SyntaxKind, Box<dyn Fn(SyntaxNode, &mut ScopeBuilder)>>,
}

unsafe impl Sync for ScopeProviderRegistry {}
unsafe impl Send for ScopeProviderRegistry {}

impl ScopeProviderRegistry {
    fn register_provider<N: ConcreteNode + AstNode<Language=IXCLanguage> + 'static>(&mut self, provider: fn(N, &mut ScopeBuilder)) {
        self.providers.insert(N::KIND, Box::new(move |node, builder| {
            N::cast(node).map(|it| provider(it, builder));
        }));
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scope<'db> {
    parent: Option<NodeId<'db>>,
    names: BTreeMap<String, ItemPtr<'db>>,
}

pub struct ScopeBuilder<'db> {
    path: NodeId<'db>,
    scope: Scope<'db>,
    db: &'db dyn Database
}

impl<'db> ScopeBuilder<'db> {
    pub fn provide_symbol_for_children<N: ItemDefiner>(&mut self, node: N) {
        if let Some(name) = node.get_name().map(|it| it.name()).flatten() {
            self.scope.names.insert(name.text().to_string(), N::wrap_ptr(AstPtr::new(self.db, &node)));
        } else {
            // TODO: report diagnostic
        }
    }

    pub fn inherit_parent_node_scope(&mut self) {
        if let Some(parent) = self.path.parent_path(self.db) {
            self.scope.parent = Some(parent);
        }
    }

    pub fn report_diagnostic(&self, diagnostic: Diagnostic) {
        diagnostic.accumulate(self.db)
    }
}

// TODO find a way to const initialize this
// #[salsa::tracked]
fn init_registry(db: &dyn Database) -> ScopeProviderRegistry {
    let mut registry = ScopeProviderRegistry::default();
    registry.register_provider::<File>(|node, builder| {
        builder.inherit_parent_node_scope();
        for item in node.items() {
            match item {
                Item::Interface(it) => builder.provide_symbol_for_children(it),
                Item::Object(it) => builder.provide_symbol_for_children(it),
                _ => {}
            }
        }
    });

    registry.register_provider::<Interface>(|node, builder| {
        builder.inherit_parent_node_scope();
        for item in node.items() {
            match item {
                InterfaceItem::InterfaceFn(it) => builder.provide_symbol_for_children(it),
                InterfaceItem::Struct(it) => builder.provide_symbol_for_children(it),
                InterfaceItem::Event(it) => builder.provide_symbol_for_children(it),
                InterfaceItem::MapCollection(it) => builder.provide_symbol_for_children(it),
                InterfaceItem::VarCollection(it) => builder.provide_symbol_for_children(it),
            }
        }
    });
    registry
}