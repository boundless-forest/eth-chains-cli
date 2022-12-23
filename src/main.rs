mod cli;
mod types;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Action, Cli};
use std::fs::File;
use types::ChainInfo;
use walkdir::WalkDir;
#[macro_use]
extern crate prettytable;
use prettytable::{Cell, Row, Table};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.action {
        Action::List => {
            let mut chains_info = Vec::new();
            for entry in WalkDir::new("chains/_data/chains")
                .into_iter()
                .filter_map(|i| i.ok())
                .filter(|i| i.file_type().is_file())
            {
                let file = File::open(entry.path()).unwrap();
                let chain_info: ChainInfo = serde_json::from_reader(file).unwrap();
                chains_info.push((chain_info.chain_id, chain_info.name));
            }
            chains_info.sort_by_key(|a| a.0);

            let mut table = Table::new();
            table.add_row(row!["CHAIN_ID", "CHAIN_NAME"]);
            chains_info.iter().for_each(|(id, name)| {
                table.add_row(Row::new(vec![Cell::new(&id.to_string()), Cell::new(&name)]));
            });

            table.printstd();
        }
        Action::ById { id } => {
            let file = File::open(format!(
                "{}{}{}",
                "chains/_data/chains/eip155-", id, ".json"
            ))
            .with_context(|| format!("NO chain associated with this id now"))?;
            let chain_info: ChainInfo = serde_json::from_reader(file).unwrap();
            
            println!("{}", serde_json::to_string(&chain_info).unwrap());
        }
        Action::ByName { name } => {
            let mut find = false;
            for entry in WalkDir::new("chains/_data/chains")
                .into_iter()
                .filter_map(|i| i.ok())
                .filter(|i| i.file_type().is_file())
            {
                let file = File::open(entry.path()).unwrap();
                let chain_info: ChainInfo = serde_json::from_reader(file).unwrap();
                if chain_info
                    .name
                    .to_lowercase()
                    .contains(&name.to_lowercase())
                {
                    find = true;
                    println!("{}", serde_json::to_string(&chain_info).unwrap());
                }
            }

            if !find {
                println!("NO chain associated with this name now");
            }
        }
    }
    Ok(())
}
