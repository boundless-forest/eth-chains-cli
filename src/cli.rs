use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(name = "bear")]
pub struct Cli {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(clap::Subcommand, Debug)]
pub enum Action {
    /// List all chains
    List,
    /// Add new chain's info
    Add,
    /// Find available chain id
    FindChainId,
}
