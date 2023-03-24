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
	let branch_name = "master";

	let mut proxy_opts = git2::ProxyOptions::new();
	//  TODO: Use env variable
	proxy_opts.url("http://127.0.0.1:7890"); // remove this line if you don't use proxy

	if let Ok(repo) = git2::Repository::open(local_path) {
		let mut fo = git2::FetchOptions::new();
		fo.proxy_options(proxy_opts);

		repo.find_remote("origin")?.fetch(&[branch_name], Some(&mut fo), None)?;

		let status = repo.statuses(None)?;
		if !status.is_empty() {
			panic!("Local repository is not clean. Please commit or discard changes before continuing.");
		}

		let fetch_head = repo.find_reference("FETCH_HEAD")?;
		let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
		if repo.merge_analysis(&[&fetch_commit])?.0.is_fast_forward() {
			let mut ref_head = repo.find_reference(&format!("refs/heads/{}", branch_name))?;
			ref_head.set_target(
				fetch_commit.id(),
				&format!("Fast-Forward: Setting {} to id: {}", branch_name, fetch_commit.id()),
			)?;
			repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
		}
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
			table.add_row(row!["CHAIN_NAME", "CHAIN_ID", "NATIVE_CURRENCY", "SYMBOL", "DECIMALS"]);
			chains_info.iter().for_each(|(id, name, currency)| {
				table.add_row(Row::new(vec![
					Cell::new(&name),
					Cell::new(&id.to_string()),
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
			let chain_info: ChainInfo = serde_json::from_reader(file).expect("Unable to parse chain info");

			print_chain_info(chain_info);
		}
		Action::ByName { name } => {
			let mut find = false;
			for entry in WalkDir::new("chains/_data/chains")
				.into_iter()
				.filter_map(|i| i.ok())
				.filter(|i| i.file_type().is_file())
			{
				let file = File::open(entry.path()).with_context(|| format!("NO chain associated with this id now"))?;
				let chain_info: ChainInfo = serde_json::from_reader(file).expect("Unable to parse chain info");
				if chain_info.name.to_lowercase().contains(&name.to_lowercase()) {
					find = true;
					print_chain_info(chain_info);
				}
			}

			if !find {
				println!("NO chain associated with this name now");
			}
		}
	}
	Ok(())
}

fn print_chain_info(info: ChainInfo) {
	let mut table = Table::new();
	table.add_row(row![Cell::new_align("CHAIN_NAME", CENTER), Cell::new(&info.name.to_string())]);
	table.add_row(row![Cell::new_align("CHAIN_ID", CENTER), Cell::new(&info.chain_id.to_string())]);
	table.add_row(row![
		Cell::new_align("NATIVE_CURRENCY", CENTER),
		Cell::new(&info.native_currency.name)
	]);
	table.add_row(row![Cell::new_align("SYMBOL", CENTER), Cell::new(&info.native_currency.symbol)]);
	table.add_row(row![
		Cell::new_align("DECIMALS", CENTER),
		Cell::new(&info.native_currency.decimals.to_string())
	]);
	table.add_row(row![Cell::new_align("NETWORK", CENTER), Cell::new(&info.network_id.to_string())]);
	table.add_row(row![Cell::new_align("INFO", CENTER), Cell::new(&info.info_url)]);
	table.add_row(row![
		Cell::new_align("RPC", CENTER),
		if info.rpc.is_empty() {
			Cell::new("None")
		} else {
			Cell::new(&info.rpc.join("\n"))
		}
	]);
	table.add_row(row![
		Cell::new_align("FAUCETS", CENTER),
		if info.faucets.is_empty() {
			Cell::new("None")
		} else {
			Cell::new(&info.faucets.join("\n"))
		}
	]);
	table.add_row(row![
		Cell::new_align("EXPLORERS", CENTER),
		if let Some(e) = info.explorers {
			Cell::new(
				&e.into_iter()
					.map(|i| vec![i.name, i.url].join(" "))
					.collect::<Vec<String>>()
					.join("\n"),
			)
		} else {
			Cell::new("None")
		}
	]);
	table.add_row(row![
		Cell::new_align("FEATURES", CENTER),
		if let Some(f) = info.features {
			Cell::new(&f.into_iter().map(|i| i.name).collect::<Vec<String>>().join("\n"))
		} else {
			Cell::new("None")
		}
	]);
	table.printstd();
}
