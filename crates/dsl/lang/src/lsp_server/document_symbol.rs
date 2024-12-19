use crate::frontend::ast::*;
use crate::frontend::parser;
use crate::lsp_server::line_col::{build_line_index, to_lsp_range};
use crate::lsp_server::server::LSPServer;
use line_index::LineIndex;
use rowan::ast::AstNode;
use tower_lsp::lsp_types::{
    DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse, SymbolKind,
};
use crate::frontend::syntax::{SyntaxNode, SyntaxToken};

impl LSPServer {
    pub async fn on_document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> tower_lsp::jsonrpc::Result<Option<DocumentSymbolResponse>> {
        if let Some(src) = self.document_map.get(&params.text_document.uri) {
            let db = self.db.lock().unwrap();
            let ast = parser::parse(&*db, *src).syntax(&*db);
            let line_index = build_line_index(&*db, *src);
            let builder = SymbolBuilder { line_index: &line_index };
            if let Some(node) = File::cast(ast) {
                let file = builder.file(&node);
                return Ok(Some(DocumentSymbolResponse::Nested(vec![file])));
            }
        }
        Ok(None)
    }
}

struct SymbolBuilder<'a> {
    line_index: &'a LineIndex,
}

impl<'a> SymbolBuilder<'a> {
    fn file(&self, file: &File) -> DocumentSymbol {
        let node = file.syntax();
        let mut sym = self.symbol(SymbolKind::FILE, node, None);
        let mut children = vec![];
        for item in file.items() {
            match item {
                Item::Interface(it) => children.push(self.interface(&it)),
                // Item::Object(it) => children.push(object_symbols(&it, line_index)),
                // Item::Impl(it) => children.push(impl_symbols(&it, line_index)),
                _ => {},
            }
        }
        sym.children = Some(children);
        sym
    }

    fn interface(&self, node: &Interface) -> DocumentSymbol {
        self.symbol(SymbolKind::INTERFACE, node.syntax(), node.name())
    }

    fn symbol(&self, kind: SymbolKind, node: &SyntaxNode, name: Option<SyntaxToken>) -> DocumentSymbol {
        let (name, sel_range) = match name {
            Some(name) => (name.text().to_string(), &name.text_range()),
            None => ("".to_string(), &node.text_range()),
        };
        DocumentSymbol {
            name,
            kind,
            range: to_lsp_range(self.line_index, &node.text_range()),
            selection_range: to_lsp_range(self.line_index, &sel_range),
            children: None,
            detail: None,
            tags: None,
            deprecated: None,
        }
    }
}