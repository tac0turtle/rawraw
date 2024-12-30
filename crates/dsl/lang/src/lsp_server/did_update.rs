use crate::db::{DatabaseExt, FileSource};
use crate::lsp_server::diagnostic::run_diagnostics;
use crate::lsp_server::server::LSPServer;
use salsa::Setter;
use tower_lsp::lsp_types::{Diagnostic, DidChangeTextDocumentParams, DidOpenTextDocumentParams, Position, Range, Url};

impl LSPServer {
    pub async fn on_did_update(&self, uri: Url, text: String) {

        let lsp_diags = {
            let mut db = self.db.lock().unwrap();
            let src = if let Some(src) = db.file_source(uri.as_str()) {
                src
            } else {
                tracing::debug!("file not opened");
                return;
            };
            src.set_text(&mut *db)
                .to(text.to_string());
            run_diagnostics(&*db, src)
        };

        self.client
            .publish_diagnostics(uri, lsp_diags, None)
            .await;
    }
}
