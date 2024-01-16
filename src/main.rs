#[macro_use]
extern crate prettytable;

pub mod cli;
pub mod types;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Action, Cli};
use colored::*;
use git2::{
	build::{CheckoutBuilder, RepoBuilder},
	FetchOptions, Repository,
};
use prettytable::{format::Alignment::CENTER, Cell, Row, Table};
use std::{fs::File, path::Path};
use types::ChainInfo;
use walkdir::WalkDir;

const BRANCH_NAME: &str = "master";
const REMOTE_URL: &str = "https://github.com/ethereum-lists/chains.git";

fn main() -> Result<()> {
	let cli = Cli::parse();

	let home_dir = std::env::var("HOME").expect("HOME not set");
	let local_path = Path::new(&home_dir).join(".chains");

	if let Ok(repo) = Repository::open(&local_path) {
		let mut fo = FetchOptions::new();
		repo.find_remote("origin")?.fetch(&[BRANCH_NAME], Some(&mut fo), None)?;

		let status = repo.statuses(None)?;
		if !status.is_empty() {
			panic!(
				"Local {:?} repository is not clean. Please discard changes and try again.",
				local_path
			);
		}

		let fetch_head = repo.find_reference("FETCH_HEAD")?;
		let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
		if repo.merge_analysis(&[&fetch_commit])?.0.is_fast_forward() {
			let mut ref_head = repo.find_reference(&format!("refs/heads/{}", BRANCH_NAME))?;
			ref_head.set_target(
				fetch_commit.id(),
				&format!("Fast-Forward: Setting {} to id: {}", BRANCH_NAME, fetch_commit.id()),
			)?;
			repo.checkout_head(Some(CheckoutBuilder::default().force()))?;
		}
	} else {
		let mut builder = RepoBuilder::new();
		let mut fetch_option = FetchOptions::new();
		fetch_option.depth(1);

		builder.fetch_options(fetch_option);
		builder.clone(REMOTE_URL, &local_path)?;
	}

	match &cli.action {
		Action::List => {
			let mut chains_info = Vec::new();
			for entry in WalkDir::new(local_path.join("_data/chains"))
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
					Cell::new(name),
					Cell::new(&id.to_string()),
					Cell::new(&currency.name.to_owned()),
					Cell::new(&currency.symbol.to_owned()),
					Cell::new(&currency.decimals.to_string()),
				]));
			});

			table.printstd();
		}
		Action::ById { id } => {
			let file = File::open(local_path.join(format!("_data/chains/eip155-{}.json", id)))
				.with_context(|| format!("NO chain associated with this id now"))?;
			let chain_info: ChainInfo = serde_json::from_reader(file).expect("Unable to parse chain info");

			print_chain_info(chain_info);
		}
		Action::ByName { name } => {
			let mut find = false;
			for entry in WalkDir::new(local_path.join("_data/chains"))
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
				println!("{}", "NO chain associated with this name now".red());
			}
		}
	}
	Ok(())
}

fn print_chain_info(info: ChainInfo) {
	let mut table = Table::new();
	table.add_row(row![Cell::new_align("CHAIN_NAME", CENTER), Cell::new(&info.name)]);
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
