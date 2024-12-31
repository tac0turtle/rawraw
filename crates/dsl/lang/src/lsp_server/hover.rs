use crate::frontend::parser;
use crate::frontend::resolver::ids::NodePath;
use crate::frontend::resolver::scope::resolve_name_ref;
use crate::frontend::syntax::{SyntaxKind, SyntaxNode, SyntaxToken};
use crate::lsp_server::line_col::{build_line_index, from_lsp_position};
use crate::lsp_server::server::LSPServer;
use rowan::ast::AstNode;
use rowan::TokenAtOffset::Single;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::HoverContents::Scalar;
use tower_lsp::lsp_types::{Hover, HoverParams, MarkedString};
use crate::lsp_server::goto_definition::find_name_or_ref;

impl LSPServer {
    pub async fn on_hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let params = params.text_document_position_params;
        if let Some(src) = self.files.get(params.text_document.uri.as_str()) {
            let ast = parser::parse(&src);
            let root = ast.syntax();
            let line_index = build_line_index(&src);
            let pos = from_lsp_position(&line_index, params.position);
            if let Single(token) = root.token_at_offset(pos.into()) {
                if let Some(node) = find_name_or_ref(&token) {
                    let node_path = NodePath::new(&node);
                    let node_resolved = node_path.resolve(&root);
                    let name_ref = node.text().to_string();
                    let resolved = resolve_name_ref(&ast, &node_path, &name_ref).map(|it| it.node_path());
                    let resolved_syntax = resolved.clone().map(|it| it.resolve(&ast.syntax())).flatten();
                    return Ok(Some(Hover {
                        contents: Scalar(MarkedString::String(format!(
                            "{:?} {:?} {} {:?} {:?} {:?}",
                            node, node_path, name_ref, node_resolved, resolved, resolved_syntax
                        ))),
                        range: None,
                    }));
                }
            }
        }
        Ok(None)
    }
}

