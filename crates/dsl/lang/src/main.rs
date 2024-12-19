use std::io::Read;

fn read_example() -> anyhow::Result<String> {
    let mut input = String::new();
    // read examples/ex1.ixc
    std::fs::File::open("crates/dsl/dsl/examples/ex1.ixc")?.read_to_string(&mut input)?;
    Ok(input)
}

fn main() {
    ixc_lang::lsp_server::main()
}