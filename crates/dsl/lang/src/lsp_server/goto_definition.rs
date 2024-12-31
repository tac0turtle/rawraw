use std::str::FromStr;
use comemo::Track;
use crate::frontend::parser;
use crate::frontend::resolver::node_id::{NodeId, NodePath};
use crate::lsp_server::line_col::{
    build_line_index, from_lsp_position, to_lsp_position, to_lsp_range,
};
use crate::lsp_server::server::LSPServer;
use rowan::TokenAtOffset::Single;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location, Position, Range, Url};
use crate::frontend::resolver::resolve::resolve_name_ref;
use crate::frontend::resolver::symbol::SymbolId;
use crate::frontend::syntax::{SyntaxKind, SyntaxNode, SyntaxToken};

impl LSPServer {
    pub async fn on_goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Option<GotoDefinitionResponse> {
        let params = params.text_document_position_params;
        let src = self.files.get(params.text_document.uri.as_str())?;
        let ast = parser::parse(&src);
        let root = ast.syntax();
        let line_index = build_line_index(&src);
        let pos = from_lsp_position(&line_index, params.position);
        if let Single(token) = root.token_at_offset(pos.into()) {
            let node = find_name_or_ref(&token)?;
            let node_path = NodePath::new(&node);
            let node_id = NodeId::new(params.text_document.uri.as_str(), node_path);
            let resolved = resolve_name_ref(self.files.track(), node_id.clone())?;
            match resolved {
                SymbolId::Node(node_id) => {
                    let target_node = node_id.resolve(self.files.track())?;
                    let file_src = self.files.get(node_id.filename.as_str())?;
                    let line_index = build_line_index(&file_src);
                    let range = to_lsp_range(&line_index, &target_node.text_range());
                    Some(GotoDefinitionResponse::Scalar(Location {
                        uri: Url::from_str(node_id.filename.as_str()).unwrap(),
                        range,
                    }))
                }
                _ => None
            }
        } else {
            None
        }
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
