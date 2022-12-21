mod cli;
mod types;

use anyhow::Result;
use clap::{Arg, Command, Parser};
use cli::{Action, Cli};
use std::{collections::HashMap, fs::File};
use types::ChainInfo;
use walkdir::WalkDir;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.action {
        Action::List => {
            for entry in WalkDir::new("chains/_data/chains")
                .into_iter()
                .filter_map(|i| i.ok())
                .filter(|i| i.file_type().is_file())
            {
                let file = File::open(entry.path()).unwrap();
                let chain_info: ChainInfo = serde_json::from_reader(file).unwrap();
                println!("{:?}, {:?}", chain_info.chain_id, chain_info.name);
            }
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
