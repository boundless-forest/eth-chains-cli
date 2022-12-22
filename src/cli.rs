use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(clap::Subcommand, Debug)]
pub enum Action {
    /// List all chains
    List,
    /// Get chain info by chain_id
    GetChainInfoById {
        #[arg(short, long)]
        id: u64,
    },
    /// Add new chain's info
    Add,
}
