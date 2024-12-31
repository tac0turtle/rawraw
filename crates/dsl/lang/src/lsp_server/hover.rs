use crate::frontend::parser;
use crate::frontend::resolver::node_id::{NodeId, NodePath};
use crate::frontend::resolver::resolve::resolve_name_ref;
use crate::lsp_server::goto_definition::find_name_or_ref;
use crate::lsp_server::line_col::{build_line_index, from_lsp_position};
use crate::lsp_server::server::LSPServer;
use comemo::Track;
use rowan::ast::AstNode;
use rowan::TokenAtOffset::Single;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::HoverContents::Scalar;
use tower_lsp::lsp_types::{Hover, HoverParams, MarkedString};

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
                    let node_id = NodeId::new(params.text_document.uri.as_str(), node_path);
                    let resolved = resolve_name_ref(self.files.track(), node_id.clone());
                    return Ok(Some(Hover {
                        contents: Scalar(MarkedString::String(format!(
                            "{:?} {:?}",
                            node_id, resolved
                        ))),
                        range: None,
                    }));
                }
            }
        }
        Ok(None)
    }
}
