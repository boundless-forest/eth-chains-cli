#[macro_use]
extern crate prettytable;

mod cli;
mod types;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Action, Cli};
use git2::BranchType;
use prettytable::{Cell, Row, Table};
use std::{fs::File, path::Path};
use types::ChainInfo;
use walkdir::WalkDir;

fn main() -> Result<()> {
	let cli = Cli::parse();

	let remote_url = "https://github.com/ethereum-lists/chains.git";
	let local_path = Path::new("chains");

	let mut proxy_opts = git2::ProxyOptions::new();
	//  TODO: Use env variable
	proxy_opts.url("http://127.0.0.1:7890"); // remove this line if you don't use proxy

	if let Ok(repo) = git2::Repository::open(local_path) {
		let mut fo = git2::FetchOptions::new();
		fo.proxy_options(proxy_opts);

		let mut upstream = repo.find_remote("origin")?;
		let upstream_branch = repo.find_branch("origin/master", BranchType::Remote)?;
		let upstream_branch_ref = upstream_branch.get();
		upstream.fetch(
			&[upstream_branch_ref.name().expect("Unable to find remote branch info")],
			Some(&mut fo),
			None,
		)?;
		let upstream_object = repo.find_object(upstream_branch_ref.target().expect("Unable to get Oid"), None)?;
		repo.reset(&upstream_object, git2::ResetType::Hard, None)?;
	} else {
		let mut fo = git2::FetchOptions::new();
		fo.proxy_options(proxy_opts);

		let mut builder = git2::build::RepoBuilder::new();
		builder.fetch_options(fo);

		builder.clone(remote_url, local_path)?;
	}

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
			let file = File::open(format!("{}{}{}", "chains/_data/chains/eip155-", id, ".json"))
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
				if chain_info.name.to_lowercase().contains(&name.to_lowercase()) {
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
