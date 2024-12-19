use tower_lsp::lsp_types::{Diagnostic, DidOpenTextDocumentParams, Position, Range};
use std::borrow::Borrow;
use crate::db::{Db, FileSource};
use crate::lsp::server::LSPServer;
use crate::{diagnostic, parser};
use crate::lsp::diagnostic::{run_diagnostics, to_lsp_diagnostic};
use crate::lsp::line_col;

impl LSPServer {
    pub async fn on_did_open(&self, params: DidOpenTextDocumentParams) {
        if self.document_map.contains_key(&params.text_document.uri) {
            tracing::debug!("file already opened");
            return;
        }


        let lsp_diags = {
            let db = self.db.lock().unwrap();
            let src = FileSource::new(&*db, params.text_document.text);
            self.document_map.insert(params.text_document.uri.clone(), src);
            run_diagnostics(&*db, src)
        };

        self.client.publish_diagnostics(params.text_document.uri, lsp_diags, None).await;
    }
}