use std::ops::Range;
use line_index::LineIndex;
use rowan::TextRange;
use tower_lsp::lsp_types;

#[salsa::accumulator]
#[derive(Eq, PartialEq)]
pub struct Diagnostic {
    pub message: String,
    pub range: TextRange,
    pub severity: Severity,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint,
}

pub fn text_range_from_span(span: &Range<usize>) -> TextRange {
    let Range { start, end } = span;
    TextRange::new((*start).try_into().unwrap(), (*end).try_into().unwrap())
}