use tower_lsp::lsp_types::{Diagnostic, DidOpenTextDocumentParams, Position, Range};
use std::borrow::Borrow;
use crate::db::{Db, FileSource, DatabaseExt};
use crate::lsp_server::server::LSPServer;
use crate::frontend::diagnostic;
use crate::frontend::parser;
use crate::lsp_server::diagnostic::{run_diagnostics, to_lsp_diagnostic};
use crate::lsp_server::line_col;

impl LSPServer {
    pub async fn on_did_open(&self, params: DidOpenTextDocumentParams) {

        let lsp_diags = {
            let db = self.db.lock().unwrap();
            if db.file_source(params.text_document.uri.as_str()).is_some() {
                tracing::debug!("file already opened");
                return;
            }

            let src = FileSource::new(&*db, params.text_document.text);
            db.add_file_source(params.text_document.uri.clone().into(), src);
            run_diagnostics(&*db, src)
        };

        self.client.publish_diagnostics(params.text_document.uri, lsp_diags, None).await;
    }
}