use crate::frontend::ast::{ConcreteNode, File, Interface, InterfaceItem, Item};
use crate::frontend::diagnostic::Diagnostic;
use crate::frontend::resolve::definer::ItemDefiner;
use crate::frontend::resolve::ids::{AstPtr, NodePath};
use crate::frontend::resolve::item_ref::ItemPtr;
use crate::frontend::syntax::{IXCLanguage, SyntaxKind, SyntaxNode};
use rowan::ast::AstNode;
use salsa::{Accumulator, Database};
use std::collections::HashMap;
use std::sync::LazyLock;
use dashmap::DashMap;

#[derive(Default)]
pub struct ScopeProviderRegistry {
    providers: DashMap<SyntaxKind, Box<dyn Fn(SyntaxNode, &mut ScopeBuilder)>>,
}

impl ScopeProviderRegistry {
    fn register_provider<N: ConcreteNode + AstNode<Language=IXCLanguage> + 'static>(&mut self, provider: fn(N, &mut ScopeBuilder)) {
        self.providers.insert(N::KIND, Box::new(move |node, builder| {
            N::cast(node).map(|it| provider(it, builder));
        }));
    }
}

pub struct Scope<'db> {
    parent: Option<NodePath>,
    names: HashMap<String, ItemPtr<'db>>,
}

pub struct ScopeBuilder<'db> {
    path: NodePath,
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
        self.scope.parent = self.path.parent_path();
    }

    pub fn report_diagnostic(&self, diagnostic: Diagnostic) {
        diagnostic.accumulate(self.db)
    }
}

static REGISTRY: LazyLock<ScopeProviderRegistry> = LazyLock::new(|| init_registry());

fn init_registry() -> ScopeProviderRegistry {
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