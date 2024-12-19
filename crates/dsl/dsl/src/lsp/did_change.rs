use crate::db::FileSource;
use crate::lsp::diagnostic::run_diagnostics;
use crate::lsp::server::LSPServer;
use salsa::Setter;
use tower_lsp::lsp_types::{
    Diagnostic, DidChangeTextDocumentParams, DidOpenTextDocumentParams, Position, Range,
};

impl LSPServer {
    pub async fn on_did_change(&self, params: DidChangeTextDocumentParams) {
        let src = if let Some(src) = self.document_map.get(&params.text_document.uri) {
            src
        } else {
            tracing::debug!("file not opened");
            return;
        };

        let lsp_diags = {
            let mut db = self.db.lock().unwrap();
            src.set_text(&mut *db)
                .to(params.content_changes[0].text.clone());
            run_diagnostics(&*db, *src)
        };

        self.client
            .publish_diagnostics(params.text_document.uri, lsp_diags, None)
            .await;
    }
}
