pub mod cli;
pub mod types;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Action, Cli};
use colored::*;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Attribute, Cell, Color, ContentArrangement, Table};
use std::{env, fs::File, path::Path, process::Command};
use types::ChainInfo;
use walkdir::WalkDir;

const REMOTE_URL: &str = "https://github.com/ethereum-lists/chains.git";

fn main() -> Result<()> {
	let cli = Cli::parse();

	let home_dir = std::env::var("HOME").expect("HOME not set");
	let local_path = Path::new(&home_dir).join(".chains");

	if local_path.exists() {
		println!("Fetching the latest chain info from {REMOTE_URL} and store in {:?}", local_path);
		env::set_current_dir(&local_path)?;
		Command::new("git").args(["pull", "--depth", "1"]).status()?;
	} else {
		println!("Downloading the latest chain info from {REMOTE_URL} and store in {:?}", local_path);
		Command::new("git")
			.args(["clone", REMOTE_URL, &local_path.to_string_lossy(), "--depth", "1"])
			.status()?;
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
			table
				.set_header(vec![
					Cell::new("CHAIN_NAME").fg(Color::Green),
					Cell::new("CHAIN_ID").fg(Color::Green),
					Cell::new("NATIVE_CURRENCY").fg(Color::Green),
					Cell::new("SYMBOL").fg(Color::Green),
					Cell::new("DECIMALS").fg(Color::Green),
				])
				.load_preset(UTF8_FULL)
				.apply_modifier(UTF8_ROUND_CORNERS)
				.set_content_arrangement(ContentArrangement::Dynamic);
			chains_info.iter().for_each(|(id, name, currency)| {
				table.add_row(vec![
					Cell::new(name),
					Cell::new(id.to_string()),
					Cell::new(currency.name.to_owned()),
					Cell::new(currency.symbol.to_owned()),
					Cell::new(currency.decimals.to_string()),
				]);
			});
			println!("{table}");
		}
		Action::ById { id } => {
			let file = File::open(local_path.join(format!("_data/chains/eip155-{}.json", id)))
				.with_context(|| format!("NO chain associated with this id now"))?;
			let chain_info: ChainInfo = serde_json::from_reader(file).expect("Unable to parse chain info");

			print_chain_info(chain_info);
		}
		Action::ByName { name } => {
			let mut find = false;
			let mut candidate = Vec::new();
			for entry in WalkDir::new(local_path.join("_data/chains"))
				.into_iter()
				.filter_map(|i| i.ok())
				.filter(|i| i.file_type().is_file())
			{
				let file = File::open(entry.path()).with_context(|| format!("NO chain associated with this id now"))?;
				let chain_info: ChainInfo = serde_json::from_reader(file).expect("Unable to parse chain info");

				if chain_info.name == *name {
					find = true;
					print_chain_info(chain_info);
				} else if chain_info.name.to_lowercase().contains(&name.to_lowercase()) {
					candidate.push((chain_info.name, chain_info.chain_id));
				}
			}

			if !candidate.is_empty() {
				let mut table = Table::new();
				table
					.set_header(vec![
						Cell::new("Candidate Chain's Name").add_attribute(Attribute::Bold).fg(Color::Green),
						Cell::new("Candidate Chain's ID").add_attribute(Attribute::Bold).fg(Color::Green),
					])
					.apply_modifier(UTF8_ROUND_CORNERS)
					.set_content_arrangement(ContentArrangement::Dynamic);
				candidate.iter().for_each(|(name, id)| {
					table.add_row(vec![Cell::new(name), Cell::new(id)]);
				});

				println!("{table}");
				return Ok(());
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
	table
		.set_header(vec![
			Cell::new("ITEMS").add_attribute(Attribute::Bold).fg(Color::Green),
			Cell::new("VALUE").add_attribute(Attribute::Bold).fg(Color::Green),
		])
		.apply_modifier(UTF8_ROUND_CORNERS)
		.set_content_arrangement(ContentArrangement::Dynamic);

	table.add_row(vec![Cell::new("CHAIN_NAME").fg(Color::Green), Cell::new(&info.name)]);
	table.add_row(vec![Cell::new("CHAIN_ID").fg(Color::Green), Cell::new(info.chain_id.to_string())]);
	table.add_row(vec![
		Cell::new("NATIVE_CURRENCY").fg(Color::Green),
		Cell::new(&info.native_currency.name),
	]);
	table.add_row(vec![Cell::new("SYMBOL").fg(Color::Green), Cell::new(&info.native_currency.symbol)]);
	table.add_row(vec![
		Cell::new("DECIMALS").fg(Color::Green),
		Cell::new(info.native_currency.decimals.to_string()),
	]);
	table.add_row(vec![Cell::new("NETWORK").fg(Color::Green), Cell::new(info.network_id.to_string())]);
	table.add_row(vec![Cell::new("INFO").fg(Color::Green), Cell::new(&info.info_url)]);
	table.add_row(vec![
		Cell::new("RPC").fg(Color::Green),
		if info.rpc.is_empty() {
			Cell::new("None")
		} else {
			Cell::new(info.rpc.join("\n"))
		},
	]);
	table.add_row(vec![
		Cell::new("FAUCETS").fg(Color::Green),
		if info.faucets.is_empty() {
			Cell::new("None")
		} else {
			Cell::new(info.faucets.join("\n"))
		},
	]);
	table.add_row(vec![
		Cell::new("EXPLORERS").fg(Color::Green),
		if let Some(e) = info.explorers {
			Cell::new(
				e.into_iter()
					.map(|i| vec![i.name, i.url].join(" "))
					.collect::<Vec<String>>()
					.join("\n"),
			)
		} else {
			Cell::new("None")
		},
	]);
	table.add_row(vec![
		Cell::new("FEATURES").fg(Color::Green),
		if let Some(f) = info.features {
			Cell::new(f.into_iter().map(|i| i.name).collect::<Vec<String>>().join("\n"))
		} else {
			Cell::new("None")
		},
	]);
	println!("{table}");
}
