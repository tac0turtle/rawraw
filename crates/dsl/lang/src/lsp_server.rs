// mod completion;
// mod nrs_lang;
// mod semantic_analyze;
// mod semantic_token;
// mod span;
// mod symbol_table;
mod server;
mod did_open;
mod document_symbol;
mod did_change;
mod diagnostic;
mod line_col;

pub use server::main;
