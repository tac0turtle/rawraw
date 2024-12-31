use crate::frontend::ast::{File, Interface};
use crate::frontend::resolver::members::{HasMembers, MemberSet};
use crate::frontend::resolver::node_id::{NodeId, NodePath};
use crate::frontend::resolver::symbol::{SymbolDefiner, SymbolId};
use crate::frontend::syntax::{IXCLanguage, SyntaxKind, SyntaxNode};
use rowan::ast::AstNode;
use std::collections::BTreeMap;
use comemo::Tracked;
use crate::files::FileSources;

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

#[comemo::memoize]
pub fn resolve_scope(files: Tracked<FileSources>, node_id: &NodeId) -> Option<Scope> {
    let node = node_id.resolve(files)?;
    let mut builder = ScopeBuilder::new(node_id.filename.as_str(), node.clone());
    let provider = as_scope_provider(node)?;
    provider.provide_scope(&mut builder);
    Some(builder.scope)
}

// pub fn resolve_scope(ast: &ParsedAST, path: &NodePath) -> Option<Scope> {
//     let registry = init_registry();
//     let node = path.resolve(&ast.syntax())?;
//     let mut builder = ScopeBuilder {
//         path: path.clone(),
//         scope: Scope::new(path.clone()),
//         root: ast.syntax(),
//     };
//     let f = registry.providers.get(&node.kind())?;
//     f(node, &mut builder);
//     Some(builder.scope)
// }
//
// pub fn resolve_name_ref(ast: &ParsedAST, node_path: &NodePath, name_ref: &str) -> Option<ItemPtr> {
//     let mut maybe_node_path = Some(node_path.clone());
//     while let Some(ref node_path) = maybe_node_path {
//         if let Some(scope) = resolve_scope(ast, node_path) {
//             if let Some(item) = scope.names.get(name_ref) {
//                 return Some(item.clone());
//             }
//             if !scope.inherit_parent_scope {
//                 let parent = node_path.parent_path()?;
//                 return resolve_name_ref(ast, &parent, name_ref);
//             }
//             return None
//         } else {
//             maybe_node_path = node_path.parent_path();
//         }
//     }
//     None
// }
//
// #[derive(Default)]
// pub struct ScopeProviderRegistry {
//     providers: DashMap<SyntaxKind, Box<dyn Fn(SyntaxNode, &mut ScopeBuilder)>>,
// }
//
// unsafe impl Sync for ScopeProviderRegistry {}
// unsafe impl Send for ScopeProviderRegistry {}
//
// impl ScopeProviderRegistry {
//     fn register_provider<N: ConcreteNode + AstNode<Language=IXCLanguage> + 'static>(&mut self, provider: fn(N, &mut ScopeBuilder)) {
//         self.providers.insert(N::KIND, Box::new(move |node, builder| {
//             N::cast(node).map(|it| provider(it, builder));
//         }));
//     }
// }
//
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scope {
    node_id: NodeId,
    names: BTreeMap<String, SymbolId>,
    pub(crate) inherit_parent_scope: bool,
}

impl Scope {
    pub fn new(node_id: NodeId) -> Self {
        Self {
            node_id,
            names: Default::default(),
            inherit_parent_scope: false,
        }
    }

    pub fn resolve_name_ref(&self, name_ref: &str) -> Option<SymbolId> {
        self.names.get(name_ref).cloned()
    }
}

pub struct ScopeBuilder {
    node: SyntaxNode,
    scope: Scope,
}

impl ScopeBuilder {
    pub(crate) fn new(filename: &str, node: SyntaxNode) -> Self {
        Self {
            node: node.clone(),
            scope: Scope::new(NodeId::new(filename, NodePath::new(&node))),
        }
    }

    pub fn put_into_scope<N: SymbolDefiner>(&mut self, node: N) {
        if let Some(name) = node.get_name().map(|it| it.name()).flatten() {
            self.scope.names.insert(
                name.text().to_string(),
                SymbolId::Node(NodeId::new(
                    self.scope.node_id.filename.as_str(),
                    NodePath::new(node.syntax()),
                )),
            );
        } else {
            // TODO: report diagnostic
        }
    }

    pub fn inherit_parent_scope(&mut self) {
        self.scope.inherit_parent_scope = true
    }

    pub fn put_members_into_scope<N: HasMembers>(&mut self, node: &N) {
        assert_eq!(&self.node, node.syntax());
        let mut members = MemberSet::new(self.scope.node_id.clone());
        node.provide_members(&mut members);
        for (name, symbol) in members.members {
            self.scope.names.insert(name, symbol);
        }
    }
}

// // TODO find a way to const initialize this
// // #[salsa::tracked]
// fn init_registry() -> ScopeProviderRegistry {
//     let mut registry = ScopeProviderRegistry::default();
//     registry.register_provider::<File>(|node, builder| {
//         for item in node.items() {
//             match item {
//                 Item::Interface(it) => builder.put_into_scope(it),
//                 Item::Object(it) => builder.put_into_scope(it),
//                 _ => {}
//             }
//         }
//     });
//
//     registry.register_provider::<Interface>(|node, builder| {
//         for item in node.items() {
//             match item {
//                 InterfaceItem::InterfaceFn(it) => builder.put_into_scope(it),
//                 InterfaceItem::Struct(it) => builder.put_into_scope(it),
//                 InterfaceItem::Event(it) => builder.put_into_scope(it),
//                 InterfaceItem::MapCollection(it) => builder.put_into_scope(it),
//                 InterfaceItem::VarCollection(it) => builder.put_into_scope(it),
//             }
//         }
//     });
//     registry
// }
