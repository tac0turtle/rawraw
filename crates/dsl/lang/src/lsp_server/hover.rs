use crate::frontend::ast::File;
use crate::frontend::parser;
use crate::frontend::syntax::{SyntaxKind, SyntaxNode, SyntaxToken};
use crate::lsp_server::line_col::{build_line_index, from_lsp_position};
use crate::lsp_server::server::LSPServer;
use rowan::ast::AstNode;
use rowan::TokenAtOffset::Single;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::HoverContents::Scalar;
use tower_lsp::lsp_types::{
    DidOpenTextDocumentParams, DocumentSymbolResponse, Hover, HoverParams, MarkedString,
};

impl LSPServer {
    pub async fn on_hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let params = params.text_document_position_params;
        if let Some(src) = self.document_map.get(&params.text_document.uri) {
            let db = self.db.lock().unwrap();
            let root = parser::parse(&*db, *src).syntax(&*db);
            let line_index = build_line_index(&*db, *src);
            let pos = from_lsp_position(&line_index, params.position);
            if let Single(token) = root.token_at_offset(pos.into()) {
                if let Some(node) = find_name_or_ref(&token) {
                    return Ok(Some(Hover {
                        contents: Scalar(MarkedString::String(format!("{:?}", node))),
                        range: None,
                    }));
                }
            }
        }
        Ok(None)
    }
}

fn find_name_or_ref(token: &SyntaxToken) -> Option<SyntaxNode> {
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
