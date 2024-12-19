use tower_lsp::lsp_types::{DidChangeTextDocumentParams, DidOpenTextDocumentParams};
use crate::db::FileSource;
use crate::lsp::server::LSPServer;

impl LSPServer {
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(src) = self.document_map.get(&params.text_document.uri) {
            let mut db  = self.db.lock().unwrap();
            src.set_text(&mut *db).to(params.content_changes[0].text.clone());
        } else {
            tracing::debug!("file not opened");
        }
    }
}
