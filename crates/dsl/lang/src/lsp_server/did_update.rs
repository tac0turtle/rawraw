use crate::db::{DatabaseExt, FileSource};
use crate::lsp_server::server::LSPServer;
use salsa::Setter;
use tower_lsp::lsp_types::{Diagnostic, DidChangeTextDocumentParams, DidOpenTextDocumentParams, MessageType, Position, Range, Url};
use crate::lsp_server::diagnostic::run_diagnostics;

impl LSPServer {
    pub async fn on_did_update(&self, uri: Url, text: String) {
        let (ast_debug, lsp_diags) = {
            let mut db = self.db.lock().unwrap();
            let src = if let Some(src) = db.file_source(uri.as_str()) {
                src
            } else {
                // self.client.log_message(MessageType::ERROR, format!("file not opened: {uri}")).await;
                return;
            };
            src.set_text(&mut *db)
                .to(text.to_string());
            let (ast, lsp_diags) = run_diagnostics(&*db, src);
            let ast_debug = format!("{:#?}", ast.syntax(&*db));
            (ast_debug, lsp_diags)
        };

        self.client
            .publish_diagnostics(uri, lsp_diags, None)
            .await;
        self.client.log_message(MessageType::INFO, format!("ast: {ast_debug}")).await;
    }
}
