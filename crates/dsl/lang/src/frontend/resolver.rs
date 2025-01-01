use comemo::Tracked;
use syn::ReturnType::Default;
use crate::files::FileSources;
use crate::frontend::diagnostic::{Diagnostic, Severity};
use crate::frontend::resolver::node_id::{NodeId, NodePath};
use crate::frontend::resolver::resolve::resolve_name_ref;

pub mod symbol;
pub mod scope;
pub mod node_id;
pub mod resolve;
mod file;
mod interface;
mod members;
mod struct_;
mod impl_;

#[comemo::memoize]
pub fn resolve(sources: Tracked<FileSources>, filename: &str) -> Result<(), Vec<Diagnostic>> {
    let src = sources.get(filename)
        .ok_or_else(|| vec![])?;
    let ast = crate::frontend::parser::parse(&src);
    let root = ast.syntax();
    let mut diagnostics = vec![];
    for item in root.descendants() {
        if item.kind() == crate::frontend::syntax::SyntaxKind::NAME_REF {
            let node_id = NodeId::new(filename, NodePath::new(&item));
            if resolve_name_ref(sources, node_id).is_none() {
                diagnostics.push(Diagnostic::new(
                    "unresolved symbol".into(),
                    item.text_range(),
                    Severity::Error,
                ));
            }
        }
    }
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    Ok(())
}

