use clap::{Parser, Subcommand};
use ixc_lang::frontend::compile_cli;

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
        Some(Command::LSPServer) => {
            // TODO: remove this in the future - it's just for debugging now
            let file_appender = tracing_appender::rolling::hourly("/tmp/ixc_lsp_debug", "ixc_lsp.log");
            let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
            tracing_subscriber::fmt().with_writer(non_blocking).init();
            ixc_lang::lsp_server::main()
        }
        _ => {
            // test example:
            compile_cli("crates/dsl/lang/examples/bank.ixc")
        }
    }
}
