use crate::db::DatabaseExt;
use crate::frontend::ast::*;
use crate::frontend::parser;
use crate::frontend::syntax::SyntaxNode;
use crate::lsp_server::line_col::{build_line_index, to_lsp_range};
use crate::lsp_server::server::LSPServer;
use line_index::LineIndex;
use rowan::ast::AstNode;
use tower_lsp::lsp_types::{
    DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse, SymbolKind,
};

impl LSPServer {
    pub async fn on_document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> tower_lsp::jsonrpc::Result<Option<DocumentSymbolResponse>> {
        let db = self.db.lock().unwrap();
        if let Some(src) = db.file_source(params.text_document.uri.as_str()) {
            let ast = parser::parse(&*db, src).syntax(&*db);
            let line_index = build_line_index(&*db, src);
            let builder = SymbolBuilder {
                line_index: &line_index,
            };
            if let Some(node) = File::cast(ast) {
                let syms = builder.file_symbols(&node);
                return Ok(Some(DocumentSymbolResponse::Nested(syms)));
            }
        }
        Ok(None)
    }
}

struct SymbolBuilder<'a> {
    line_index: &'a LineIndex,
}

impl<'a> SymbolBuilder<'a> {
    fn file_symbols(&self, file: &File) -> Vec<DocumentSymbol> {
        let mut children = vec![];
        for item in file.items() {
            match item {
                Item::Interface(it) => children.push(self.interface(&it)),
                Item::Object(it) => children.push(self.object(&it)),
                Item::Impl(it) => children.push(self.impl_(&it)),
                // Item::Test(it) => children.push(self.test(&it)),
                _ => {}
            }
        }
        children
    }

    fn interface(&self, node: &Interface) -> DocumentSymbol {
        let mut children = Vec::new();
        for item in node.items() {
            match item {
                InterfaceItem::Struct(it) => children.push(self.struct_(&it)),
                InterfaceItem::InterfaceFn(it) => {
                    self.interface_fn(&it).map(|it| children.push(it));
                }
                InterfaceItem::Event(it) => children.push(self.event(&it)),
                InterfaceItem::MapCollection(it) => children.push(self.map_collection(&it)),
                InterfaceItem::VarCollection(it) => children.push(self.var_collection(&it)),
            }
        }
        self.symbol(
            SymbolKind::INTERFACE,
            node.syntax(),
            node.name(),
            Some(children),
        )
    }

    fn object(&self, node: &Object) -> DocumentSymbol {
        let mut children = Vec::new();
        for item in node.items() {
            match item {
                ObjectItem::MapCollection(it) => children.push(self.map_collection(&it)),
                ObjectItem::Client(it) => children.push(self.client(&it)),
                ObjectItem::VarCollection(it) => children.push(self.var_collection(&it)),
                ObjectItem::ImplFn(it) => {
                    self.impl_fn(&it).map(|it| children.push(it));
                }
            }
        }
        self.symbol(
            SymbolKind::CLASS,
            node.syntax(),
            node.name(),
            Some(children),
        )
    }

    fn impl_(&self, node: &Impl) -> DocumentSymbol {
        let mut children = Vec::new();
        for item in node.items() {
            match item {
                ImplItem::ImplFn(it) => {
                    self.impl_fn(&it).map(|it| children.push(it));
                }
                ImplItem::MapCollection(it) => children.push(self.map_collection(&it)),
                ImplItem::VarCollection(it) => children.push(self.var_collection(&it)),
            }
        }
        self.symbol(
            SymbolKind::OBJECT,
            node.syntax(),
            node.name(),
            Some(children),
        )
    }

    fn struct_(&self, node: &Struct) -> DocumentSymbol {
        let mut children = Vec::new();
        for field in node.fields() {
            children.push(self.field(&field));
        }
        self.symbol(
            SymbolKind::STRUCT,
            node.syntax(),
            node.name(),
            Some(children),
        )
    }

    fn interface_fn(&self, node: &InterfaceFn) -> Option<DocumentSymbol> {
        node.sig().map(|it| self.fn_signature(&it))
    }

    fn impl_fn(&self, node: &ImplFn) -> Option<DocumentSymbol> {
        node.sig().map(|it| self.fn_signature(&it))
    }

    fn fn_signature(&self, node: &FnSignature) -> DocumentSymbol {
        self.symbol(SymbolKind::METHOD, node.syntax(), node.name(), None)
    }

    fn event(&self, node: &Event) -> DocumentSymbol {
        let mut children = vec![];
        for field in node.fields() {
            children.push(self.field(&field));
        }
        self.symbol(
            SymbolKind::STRUCT,
            node.syntax(),
            node.name(),
            Some(children),
        )
    }

    fn field(&self, node: &StructField) -> DocumentSymbol {
        self.symbol(SymbolKind::FIELD, node.syntax(), node.name(), None)
    }

    fn map_collection(&self, node: &MapCollection) -> DocumentSymbol {
        self.symbol(SymbolKind::FIELD, node.syntax(), node.name(), None)
    }

    fn var_collection(&self, node: &VarCollection) -> DocumentSymbol {
        self.symbol(SymbolKind::FIELD, node.syntax(), node.name(), None)
    }

    fn client(&self, node: &Client) -> DocumentSymbol {
        self.symbol(SymbolKind::FIELD, node.syntax(), node.name(), None)
    }

    // fn test(&self, node: &Test) -> DocumentSymbol {
    //     self.symbol(SymbolKind::FUNCTION, node.syntax(), node.name(), None)
    // }

    fn symbol<N: AstNode>(
        &self,
        kind: SymbolKind,
        node: &SyntaxNode,
        name: Option<N>,
        children: Option<Vec<DocumentSymbol>>,
    ) -> DocumentSymbol {
        let (name, sel_range) = match name {
            Some(name) => {
                let syntax = name.syntax();
                (syntax.text().to_string(), &syntax.text_range())
            }
            None => ("".to_string(), &node.text_range()),
        };
        DocumentSymbol {
            name,
            kind,
            range: to_lsp_range(self.line_index, &node.text_range()),
            selection_range: to_lsp_range(self.line_index, &sel_range),
            children,
            detail: None,
            tags: None,
            deprecated: None,
        }
    }
}
