use tower_lsp::lsp_types::DidOpenTextDocumentParams;
use std::borrow::Borrow;
use crate::db::{Db, FileSource};
use crate::lsp::server::LSPServer;

impl LSPServer {
    pub async fn on_did_open(&self, params: DidOpenTextDocumentParams) {
        if self.document_map.contains_key(&params.text_document.uri) {
            tracing::debug!("file already opened");
            return;
        }

        let db  = self.db.lock().unwrap();
        let src = FileSource::new(&*db, params.text_document.text);
        self.document_map.insert(params.text_document.uri, src);
    }
}