use std::io::Read;
use clap::{Parser, Subcommand};
use ixc_lang::frontend;
use ixc_lang::frontend::compile_cli;
use ixc_lang::frontend::diagnostic::Diagnostic;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    #[command(name = "lsp-server")]
    LSPServer,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::LSPServer) => ixc_lang::lsp_server::main(),
        _ =>  {
            // test example:
            compile_cli("crates/dsl/lang/examples/bank.ixc")
        }
    }
}