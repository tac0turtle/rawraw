//! Build script.
use anyhow::bail;
use quote::quote;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Write;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Deserialize, Default, Debug)]
struct Config {
    accounts: HashMap<String, String>,
    root_account_id: Option<String>,
}

fn parse_account_id(s: &str) -> Result<u128, ParseIntError> {
    if s.starts_with("0x") {
        u128::from_str_radix(&s[2..], 16)
    } else {
        u128::from_str_radix(s, 10)
    }
}

fn process_config() -> anyhow::Result<()> {
    // read the IXC_CONFIG environment variable
    let config_file = std::env::var("IXC_CONFIG");
    println!("cargo:rerun-if-env-changed=IXC_CONFIG");

    // process the context of the environment variable as a filename pointing to a TOML config file
    // if there is no environment variable, use the default config
    let config: Config = match config_file {
        Ok(config_file) => {
            if !std::path::Path::new(&config_file).exists() {
                bail!("IXC_CONFIG file not found: {config_file}");
            }
            println!("cargo:rerun-if-changed={config_file}");
            let config_str = std::fs::read_to_string(config_file)?;
            toml::from_str(&config_str)?
        }
        Err(_) => Default::default(),
    };

    // get the "accounts" key
    let known_accounts = &config.accounts;

    let mut account_names = Vec::new();
    let mut ids = Vec::new();
    for (k, v) in known_accounts {
        account_names.push(k);
        let id = parse_account_id(v)?;
        ids.push(id);
    }

    let root_account = if let Some(root_account) = config.root_account_id {
        let id = parse_account_id(&root_account)?;
        id
    } else {
        0
    };

    let output = quote! {
        /// Well-known account mappings.
        pub const KNOWN_ACCOUNTS: &[(&str, u128)] = &[
            #((#account_names, #ids)),*
        ];

        /// The ID of the root account which creates and manages accounts.
        pub const ROOT_ACCOUNT: AccountID = ixc_message_api::AccountID::new(#root_account);
    };

    let out_dir = std::env::var("OUT_DIR")?;
    let mut file = std::fs::File::create(format!("{}/known_accounts.rs", out_dir))?;
    write!(file, "{}", output)?;
    Ok(())
}

fn main() {
    match process_config() {
        Err(e) => panic!("{e}"),
        _ => {}
    }
}
