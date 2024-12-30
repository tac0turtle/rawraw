use crate::db::FileSource;
use crate::frontend;
use crate::frontend::ast::ParsedAST;
use crate::frontend::diagnostic::{Diagnostic, Severity};
use crate::lsp_server::line_col::{build_line_index, to_lsp_range};
use crate::lsp_server::server::LSPServer;
use line_index::LineIndex;
use salsa::Database;
use tower_lsp::lsp_types;
use tower_lsp::lsp_types::MessageType;

pub fn run_diagnostics<'a>(
    db: &'a dyn Database,
    src: FileSource,
) -> (ParsedAST<'a>, Vec<lsp_types::Diagnostic>) {
    let ast = frontend::compile(&*db, src);
    let line_index = build_line_index(&*db, src);
    let diags = frontend::compile::accumulated::<Diagnostic>(&*db, src);
    let mut lsp_diags = vec![];
    for diag in diags {
        lsp_diags.push(to_lsp_diagnostic(&line_index, diag));
    }
    (ast, lsp_diags)
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
