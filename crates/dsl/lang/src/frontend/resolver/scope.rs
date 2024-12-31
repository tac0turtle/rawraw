use crate::frontend::ast::{ConcreteNode, File, Interface, InterfaceItem, Item, ParsedAST};
use crate::frontend::diagnostic::Diagnostic;
use crate::frontend::resolver::symbol::SymbolDefiner;
use crate::frontend::resolver::ids::{AstPtr, NodePath};
use crate::frontend::resolver::item_ref::ItemPtr;
use crate::frontend::syntax::{IXCLanguage, SyntaxKind, SyntaxNode};
use dashmap::DashMap;
use rowan::ast::AstNode;
use std::collections::BTreeMap;
use crate::frontend::resolver::members::HasMembers;

pub trait ScopeProvider: AstNode<Language = IXCLanguage> {
    fn provide_scope(&self, scope: &mut ScopeBuilder);
}

pub fn as_scope_provider(syntax_node: SyntaxNode) -> Option<Box<dyn ScopeProvider>> {
    match syntax_node.kind() {
        SyntaxKind::FILE => Some(Box::new(File::cast(syntax_node)?)),
        SyntaxKind::INTERFACE => Some(Box::new(Interface::cast(syntax_node)?)),
        _ => None,
    }
}

pub fn resolve_scope(ast: &ParsedAST, path: &NodePath) -> Option<Scope> {
    let registry = init_registry();
    let node = path.resolve(&ast.syntax())?;
    let mut builder = ScopeBuilder {
        path: path.clone(),
        scope: Scope::new(path.clone()),
        root: ast.syntax(),
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
            if !scope.inherit_parent_scope {
                let parent = node_path.parent_path()?;
                return resolve_name_ref(ast, &parent, name_ref);
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scope {
    node_path: NodePath,
    names: BTreeMap<String, ItemPtr>,
    inherit_parent_scope: bool,
}

impl Scope {
    pub fn new(node_path: NodePath) -> Self {
        Self {
            node_path,
            names: Default::default(),
            inherit_parent_scope: false,
        }
    }
}

pub struct ScopeBuilder {
    path: NodePath,
    scope: Scope,
    root: SyntaxNode,
}

impl ScopeBuilder {
    pub fn put_into_scope<N: SymbolDefiner>(&mut self, node: N) {
        if let Some(name) = node.get_name().map(|it| it.name()).flatten() {
            self.scope.names.insert(name.text().to_string(), N::wrap_ptr(AstPtr::new(&node)));
        } else {
            // TODO: report diagnostic
        }
    }

    pub fn inherit_parent_scope(&mut self) {
        self.scope.inherit_parent_scope = true
    }

    pub fn put_members_into_scope<N: HasMembers>(&mut self, node: &N) {
        // TODO
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
        for item in node.items() {
            match item {
                Item::Interface(it) => builder.put_into_scope(it),
                Item::Object(it) => builder.put_into_scope(it),
                _ => {}
            }
        }
    });

    registry.register_provider::<Interface>(|node, builder| {
        for item in node.items() {
            match item {
                InterfaceItem::InterfaceFn(it) => builder.put_into_scope(it),
                InterfaceItem::Struct(it) => builder.put_into_scope(it),
                InterfaceItem::Event(it) => builder.put_into_scope(it),
                InterfaceItem::MapCollection(it) => builder.put_into_scope(it),
                InterfaceItem::VarCollection(it) => builder.put_into_scope(it),
            }
        }
    });
    registry
}