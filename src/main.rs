#[macro_use]
extern crate prettytable;

mod cli;
mod types;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Action, Cli};
use git2::BranchType;
use prettytable::{format::Alignment::CENTER, Cell, Row, Table};
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
				chains_info.push((chain_info.chain_id, chain_info.name, chain_info.native_currency));
			}
			chains_info.sort_by_key(|a| a.0);

			let mut table = Table::new();
			table.add_row(row!["CHAIN_ID", "CHAIN_NAME", "NATIVE_CURRENCY", "SYMBOL", "DECIMALS"]);
			chains_info.iter().for_each(|(id, name, currency)| {
				table.add_row(Row::new(vec![
					Cell::new(&id.to_string()),
					Cell::new(&name),
					Cell::new(&currency.name.to_owned()),
					Cell::new(&currency.symbol.to_owned()),
					Cell::new(&currency.decimals.to_string()),
				]));
			});

			table.printstd();
		}
		Action::ById { id } => {
			let file = File::open(format!("{}{}{}", "chains/_data/chains/eip155-", id, ".json"))
				.with_context(|| format!("NO chain associated with this id now"))?;
			let chain_info: ChainInfo = serde_json::from_reader(file).unwrap();

			let mut table = Table::new();
			table.add_row(row![
				Cell::new_align("CHAIN_ID", CENTER),
				Cell::new(&chain_info.chain_id.to_string())
			]);
			table.add_row(row![
				Cell::new_align("CHAIN_NAME", CENTER),
				Cell::new(&chain_info.name.to_string())
			]);
			table.add_row(row![
				Cell::new_align("NATIVE_CURRENCY", CENTER),
				Cell::new(&chain_info.native_currency.name.to_owned())
			]);
			table.add_row(row![
				Cell::new_align("SYMBOL", CENTER),
				Cell::new(&chain_info.native_currency.symbol.to_owned())
			]);
			table.add_row(row![
				Cell::new_align("DECIMALS", CENTER),
				Cell::new(&chain_info.native_currency.decimals.to_string())
			]);
			table.add_row(row![
				Cell::new_align("NETWORK", CENTER),
				Cell::new(&chain_info.network_id.to_string())
			]);
			table.add_row(row![Cell::new_align("INFO", CENTER), Cell::new(&chain_info.info_url)]);
			table.add_row(row![
				Cell::new_align("RPC", CENTER),
				if chain_info.rpc.is_empty() {
					Cell::new("None")
				} else {
					Cell::new(&chain_info.rpc.join("\n"))
				}
			]);
			table.add_row(row![
				Cell::new_align("FAUCETS", CENTER),
				if chain_info.faucets.is_empty() {
					Cell::new("None")
				} else {
					Cell::new(&chain_info.faucets.join("\n"))
				}
			]);
			table.add_row(row![
				Cell::new_align("EXPLORERS", CENTER),
				if let Some(e) = chain_info.explorers {
					Cell::new(
						&e.into_iter()
							.map(|i| vec!(i.name, i.url).join(" "))
							.collect::<Vec<String>>()
							.join("\n"),
					)
				} else {
					Cell::new("None")
				}
			]);
			table.add_row(row![
				Cell::new_align("FEATURES", CENTER),
				if let Some(f) = chain_info.features {
					Cell::new(&f.into_iter().map(|i| i.name).collect::<Vec<String>>().join("\n"))
				} else {
					Cell::new("None")
				}
			]);
			table.printstd();
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
