use crate::lsp_server::server::LSPServer;
use tower_lsp::lsp_types::{Diagnostic, DidChangeTextDocumentParams, DidOpenTextDocumentParams, MessageType, Position, Range, Url};

impl LSPServer {
    pub async fn on_did_update(&self, uri: Url, text: String) {
        if self.files.get(uri.as_str()).is_none() {
            self.client.log_message(MessageType::ERROR, format!("file not opened: {uri}")).await;
            return;
        };
        self.files.update(uri.clone().as_str(), text.as_str());

        let lsp_diags = self.run_diagnostics(uri.as_str());

        self.client
            .publish_diagnostics(uri, lsp_diags, None)
            .await;
    }
}
