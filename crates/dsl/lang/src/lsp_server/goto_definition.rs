use crate::frontend::parser;
use crate::frontend::resolver::ids::NodePath;
use crate::frontend::resolver::scope::resolve_name_ref;
use crate::lsp_server::line_col::{
    build_line_index, from_lsp_position, to_lsp_position, to_lsp_range,
};
use crate::lsp_server::server::LSPServer;
use rowan::TokenAtOffset::Single;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    GotoDefinitionParams, GotoDefinitionResponse, Location, Position, Range,
};
use crate::frontend::syntax::{SyntaxKind, SyntaxNode, SyntaxToken};

impl LSPServer {
    pub async fn on_goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let params = params.text_document_position_params;
        if let Some(src) = self.files.get(params.text_document.uri.as_str()) {
            let ast = parser::parse(&src);
            let root = ast.syntax();
            let line_index = build_line_index(&src);
            let pos = from_lsp_position(&line_index, params.position);
            if let Single(token) = root.token_at_offset(pos.into()) {
                if let Some(node) = find_name_or_ref(&token) {
                    let node_path = NodePath::new(&node);
                    let name_ref = node.text().to_string();
                    // self.client.log_message(tower_lsp::lsp_types::MessageType::LOG, format!("resolving name ref: {name_ref} at {node_path:?}")).await;
                    if let Some(resolved) = resolve_name_ref(&ast, &node_path, &name_ref) {
                        // self.client.log_message(tower_lsp::lsp_types::MessageType::LOG, format!("resolved: {resolved:?}")).await;
                        let target_path = resolved.node_path();
                        if let Some(target_node) = target_path.resolve(&ast.syntax()) {
                            let range = to_lsp_range(&line_index, &target_node.text_range());
                            return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                                uri: params.text_document.uri.clone(),
                                range,
                            })));
                        }
                    }
                }
            }
        }
        Ok(None)
    }
}

pub fn find_name_or_ref(token: &SyntaxToken) -> Option<SyntaxNode> {
    if token.kind() != SyntaxKind::IDENT {
        return None;
    }
    if let Some(parent) = token.parent() {
        match parent.kind() {
            SyntaxKind::NAME => return Some(parent.clone()),
            SyntaxKind::NAME_REF => return Some(parent.clone()),
            _ => {}
        }
    }
    None
}
