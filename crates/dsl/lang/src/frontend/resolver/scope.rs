use crate::frontend::ast::{ConcreteNode, File, Interface, InterfaceItem, Item, ParsedAST};
use crate::frontend::diagnostic::Diagnostic;
use crate::frontend::resolver::definer::ItemDefiner;
use crate::frontend::resolver::ids::{AstPtr, NodePath};
use crate::frontend::resolver::item_ref::ItemPtr;
use crate::frontend::syntax::{IXCLanguage, SyntaxKind, SyntaxNode};
use dashmap::DashMap;
use rowan::ast::AstNode;
use std::collections::BTreeMap;

pub fn resolve_scope(ast: &ParsedAST, path: &NodePath) -> Option<Scope> {
    let registry = init_registry();
    let node = path.resolve(&ast.syntax())?;
    let mut builder = ScopeBuilder {
        path: path.clone(),
        scope: Scope::default(),
    };
    let f = registry.providers.get(&node.kind())?;
    f(node, &mut builder);
    Some(builder.scope)
}

pub fn resolve_name_ref(ast: &ParsedAST, node_path: &NodePath, name_ref: &str) -> Option<ItemPtr> {
    let mut maybe_node_path = Some(node_path.clone());
    while let Some(ref node_path) = maybe_node_path {
        if let Some(scope) = resolve_scope(ast, node_path) {
            if let Some(item) = scope.names.get(name_ref) {
                return Some(item.clone());
            }
            if let Some(ref parent) = scope.parent {
                return resolve_name_ref(ast, parent, name_ref);
            }
            return None
        } else {
            maybe_node_path = node_path.parent_path();
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
pub struct Scope {
    parent: Option<NodePath>,
    names: BTreeMap<String, ItemPtr>,
}

pub struct ScopeBuilder {
    path: NodePath,
    scope: Scope,
}

impl ScopeBuilder {
    pub fn provide_symbol_for_children<N: ItemDefiner>(&mut self, node: N) {
        if let Some(name) = node.get_name().map(|it| it.name()).flatten() {
            self.scope.names.insert(name.text().to_string(), N::wrap_ptr(AstPtr::new(&node)));
        } else {
            // TODO: report diagnostic
        }
    }

    pub fn inherit_parent_node_scope(&mut self) {
        if let Some(parent) = self.path.parent_path() {
            self.scope.parent = Some(parent);
        }
    }

    pub fn report_diagnostic(&self, diagnostic: Diagnostic) {
        todo!()
    }
}

// TODO find a way to const initialize this
// #[salsa::tracked]
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