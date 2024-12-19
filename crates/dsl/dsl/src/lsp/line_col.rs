use line_index::LineIndex;
use crate::db::FileSource;
use rowan::TextSize;
use salsa::Database;
use tower_lsp::lsp_types;

// #[salsa::tracked]
// pub struct LineColIndex<'db> {
//     #[return_ref]
//     pub index: line_index::LineIndex,
// }
//

#[salsa::tracked(return_ref)]
pub fn new_line_col_index(db: &dyn Database, source: FileSource) -> LineIndex {
    LineIndex::new(source.text(db))
}

pub fn to_lsp_range(index: &LineIndex, text_range: rowan::TextRange) -> lsp_types::Range {
    let start = to_lsp_position(index, text_range.start());
    let end = to_lsp_position(index, text_range.end());
    lsp_types::Range::new(start, end)
}

pub fn from_lsp_range(index: &LineIndex, range: lsp_types::Range) -> rowan::TextRange {
    let start = from_lsp_position(index, range.start);
    let end = from_lsp_position(index, range.end);
    rowan::TextRange::new(start, end)
}

pub fn to_lsp_position(index: &LineIndex, offset: TextSize) -> lsp_types::Position {
    let line_col = index.line_col(offset.into());
    lsp_types::Position::new(line_col.line, line_col.col)
}

pub fn from_lsp_position(index: &LineIndex, position: lsp_types::Position) -> TextSize {
    let line_col = line_index::LineCol {
        line: position.line,
        col: position.character,
    };
    index.offset(line_col).unwrap_or_default().into()
}
