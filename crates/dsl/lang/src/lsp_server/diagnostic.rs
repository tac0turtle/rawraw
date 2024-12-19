use line_index::LineIndex;
use salsa::Database;
use tower_lsp::lsp_types;
use crate::db::FileSource;
use crate::frontend::diagnostic::{Diagnostic, Severity};
use crate::lsp_server::line_col::{line_col_index, to_lsp_range};
use crate::frontend::{parser};

pub fn to_lsp_diagnostic(line_index: &LineIndex, diag: Diagnostic) -> lsp_types::Diagnostic {
    let range = to_lsp_range(line_index, diag.range);
    let mut lspdiag = lsp_types::Diagnostic::new_simple(range, diag.message);
    lspdiag.severity = Some(diag.severity.into());
    lspdiag
}

impl From<Severity> for lsp_types::DiagnosticSeverity {
    fn from(from: Severity) -> Self {
        match from {
            Severity::Error => lsp_types::DiagnosticSeverity::ERROR,
            Severity::Warning => lsp_types::DiagnosticSeverity::WARNING,
            Severity::Info => lsp_types::DiagnosticSeverity::INFORMATION,
            Severity::Hint => lsp_types::DiagnosticSeverity::HINT,
        }
    }
}

pub fn run_diagnostics(db: &dyn Database, src: FileSource) -> Vec<lsp_types::Diagnostic> {
    let mut lsp_diags = vec![];
    let _ = parser::parse(&*db, src);
    let line_index = line_col_index(&*db, src);
    let diags = parser::parse::accumulated::<Diagnostic>(&*db, src);
    for diag in diags {
        lsp_diags.push(to_lsp_diagnostic(&line_index, diag));
    }
    lsp_diags
}
