use std::ops::Range;
use ariadne::Source;
use line_index::LineIndex;
use rowan::TextRange;
use tower_lsp::lsp_types;

#[derive(Eq, PartialEq, Clone)]
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


impl Diagnostic {
    pub fn new(message: String, range: TextRange, severity: Severity) -> Self {
        Self {
            message,
            range,
            severity,
        }
    }

    pub fn print_report(&self, src: &str) {
        let range = to_std_range(self.range);
        ariadne::Report::build(self.severity.into(), range.clone())
            .with_message(self.message.clone())
            .with_label(ariadne::Label::new(range)
                .with_color(self.severity.into())
            )
            .finish()
            .eprint(Source::from(src))
            .unwrap()
    }
}

impl From<Severity> for ariadne::ReportKind<'static> {
    fn from(value: Severity) -> Self {
        match value {
            Severity::Error => ariadne::ReportKind::Error,
            Severity::Warning => ariadne::ReportKind::Warning,
            Severity::Info => ariadne::ReportKind::Advice,
            Severity::Hint => ariadne::ReportKind::Advice,
        }
    }
}

impl From<Severity> for ariadne::Color {
    fn from(value: Severity) -> Self {
        match value {
            Severity::Error => ariadne::Color::Red,
            Severity::Warning => ariadne::Color::Yellow,
            Severity::Info => ariadne::Color::Blue,
            Severity::Hint => ariadne::Color::Green,
        }
    }
}

fn to_std_range(range: TextRange) -> Range<usize> {
    range.start().into()..range.end().into()
}