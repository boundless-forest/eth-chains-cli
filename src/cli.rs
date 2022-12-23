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
    /// Get chain info by chain id
    ById {
        #[arg(short, long)]
        id: u64,
    },
    /// Get chain info by chain name
    ByName {
        #[arg(short, long)]
        name: String,
    },
}
