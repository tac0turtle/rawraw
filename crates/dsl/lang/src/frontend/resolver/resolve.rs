use crate::files::FileSources;
use crate::frontend::ast::NameRef;
use crate::frontend::resolver::node_id::NodeId;
use crate::frontend::resolver::scope::resolve_scope;
use crate::frontend::resolver::symbol::SymbolId;
use comemo::Tracked;
use rowan::ast::AstNode;
use crate::frontend::diagnostic::Diagnostic;

#[comemo::memoize]
pub fn resolve_name_ref(sources: Tracked<FileSources>, node_id: NodeId) -> Option<SymbolId> {
    // get the name from the name ref node
    let node = node_id.resolve(sources)?;
    let name_ref = NameRef::cast(node)?;
    let name = name_ref.name_ref()?.text().to_string();

    let mut node_id = node_id;
    while let Some(parent) = node_id.parent() {
        if let Some(scope) = resolve_scope(sources, &parent) {
            if let Some(symbol) = scope.resolve_name_ref(&name) {
                return Some(symbol);
            } else if !scope.inherit_parent_scope {
                // if we're not inheriting the parent scope, we're done
                // otherwise we continue to search up the tree
                return None;
            }
        }
        node_id = parent;
    }
    None
}
