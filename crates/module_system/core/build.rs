//! Build script.
use quote::quote;
use std::collections::HashMap;
use std::io::Write;
use serde::Deserialize;

#[derive(Deserialize, Default)]
struct Config {
    accounts: HashMap<String, String>,
}

fn process_config() -> anyhow::Result<()> {
    // read the IXC_CONFIG environment variable
    let config = std::env::var("IXC_CONFIG");
    println!("cargo:rerun-if-env-changed=IXC_CONFIG");

    // process the context of the environment variable as a TOML string
    // if there is no environment variable, use the default config
    let config: Config = match config {
        Ok(config) => toml::from_str(&config)?,
        Err(_) => Default::default(),
    };

    // get the "accounts" key
    let known_accounts = &config.accounts;

    let mut account_names = Vec::new();
    let mut ids = Vec::new();
    for (k, v) in known_accounts {
        account_names.push(k);
        let id = u128::from_str_radix(v, 16)?;
        ids.push(id);
    }

    let output = quote! {
        /// Well-known account mappings.
        pub const KNOWN_ACCOUNTS: &[(&str, u128)] = &[
            #((#account_names, #ids)),*
        ];
    };

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut file = std::fs::File::create(format!("{}/known_accounts.rs", out_dir))?;
    write!(file, "{}", output)?;
    Ok(())
}

fn main() {
    process_config().unwrap()
}
