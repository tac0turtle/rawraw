use crate::frontend;
use crate::frontend::ast::ParsedAST;
use crate::frontend::diagnostic::{Diagnostic, Severity};
use crate::lsp_server::line_col::{build_line_index, to_lsp_range};
use line_index::LineIndex;
use tower_lsp::lsp_types;

pub fn run_diagnostics<'a>(
    src: &str,
) -> Vec<lsp_types::Diagnostic> {
    let ast = frontend::compile(src);
    let line_index = build_line_index(src);
    let mut lsp_diags = vec![];
    for diag in ast.diagnostics {
        lsp_diags.push(to_lsp_diagnostic(&line_index, diag));
    }
    lsp_diags
}

pub fn to_lsp_diagnostic(line_index: &LineIndex, diag: Diagnostic) -> lsp_types::Diagnostic {
    let range = to_lsp_range(line_index, &diag.range);
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
