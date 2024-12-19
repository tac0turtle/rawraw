// mod completion;
// mod nrs_lang;
// mod semantic_analyze;
// mod semantic_token;
// mod span;
// mod symbol_table;
mod server;
mod did_open;
mod document_symbol;
mod line_col;
mod did_change;

pub use server::main;
