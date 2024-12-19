use line_index::LineIndex;
use rowan::ast::AstNode;
use tower_lsp::lsp_types::{DocumentSymbol, SymbolKind};
use crate::ast::*;
use crate::lsp::line_col::to_lsp_range;

fn file(file: &File, line_index: &LineIndex) -> DocumentSymbol {
    let node = file.syntax();
    DocumentSymbol {
        name: "".to_string(),
        kind: SymbolKind::FILE,
        range: to_lsp_range(line_index, node.text_range()),
        selection_range: to_lsp_range(line_index, node.text_range()),
        children: None,
        detail: None,
        tags: None,
        deprecated: None,
    }
}
