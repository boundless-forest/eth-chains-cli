mod cli;
mod types;

use anyhow::Result;
use clap::{Arg, Command, Parser};
use cli::{Action, Cli};
use std::{collections::HashMap, fs::File};
use types::ChainInfo;
use walkdir::WalkDir;
#[macro_use]
extern crate prettytable;
use prettytable::{Cell, Row, Table};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.action {
        Action::List => {
            let mut table = Table::new();
            table.add_row(row!["CHAIN_ID", "CHAIN_NAME"]);

            for entry in WalkDir::new("chains/_data/chains")
                .into_iter()
                .filter_map(|i| i.ok())
                .filter(|i| i.file_type().is_file())
            {
                let file = File::open(entry.path()).unwrap();
                let chain_info: ChainInfo = serde_json::from_reader(file).unwrap();
                
                table.add_row(Row::new(vec![
                    Cell::new(&chain_info.chain_id.to_string()),
                    Cell::new(&chain_info.name),
                ]));
            }
            table.printstd();
        }
        Action::Add => {
            println!("This is the add branch");
            // TODO: Implement add command
        }
        Action::FindChainId => {
            println!("This is the find_chain_id branch");
            // TODO: Implement find-chain id command
        }
    }
    Ok(())
}
