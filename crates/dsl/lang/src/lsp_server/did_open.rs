use tower_lsp::lsp_types::{Diagnostic, DidOpenTextDocumentParams, MessageType, Position, Range};
use std::borrow::Borrow;
use crate::lsp_server::server::LSPServer;
use crate::frontend::diagnostic;
use crate::frontend::parser;
use crate::lsp_server::diagnostic::{run_diagnostics, to_lsp_diagnostic};
use crate::lsp_server::line_col;

impl LSPServer {
    pub async fn on_did_open(&self, params: DidOpenTextDocumentParams) {
        if self.files.get(params.text_document.uri.as_str()).is_some() {
            self.client.log_message(MessageType::ERROR, format!("file already opened: {}", params.text_document.uri)).await;
            return;
        }

        self.files.update(params.text_document.uri.as_str(), params.text_document.text.as_str());
        let lsp_diags = run_diagnostics(params.text_document.text.as_str());

        self.client.publish_diagnostics(params.text_document.uri, lsp_diags, None).await;
    }
}