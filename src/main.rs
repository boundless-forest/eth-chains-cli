mod cli;

use clap::{Arg, Command, Parser};
use cli::{Action, Cli};

fn main() {
    let cli = Cli::parse();

    match &cli.action {
        Action::List => {
            println!("This is the action branch");
        }
        Action::Add => {
            println!("This is the add branch");
        }
        Action::FindChainId => {
            println!("This is the find_chain_id branch");
        }
    }

    // TODO: Implement list command

    // TODO: Implement add command

    // TODO: Implement find-chain id command
}