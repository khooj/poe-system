use application::calc_set_shared::{CalculateBuildError, CalculatingState, CompareOption};
use clap::Parser;
use std::io::prelude::*;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short)]
    pob: Option<String>,
    #[arg(short)]
    pastebin: Option<String>,
    itemset: String,
    output: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Args::parse();

    let mut pastebin = std::fs::File::open(cli.pastebin.unwrap())?;
    let mut buf = String::new();
    pastebin.read_to_string(&mut buf)?;
    let state = CalculatingState::parse_pastebin(buf)?;
    let items = state.items(&cli.itemset);

    let mut results = vec![];
    for it in items {
        let item = match state
            .calculate_item_cost(it, vec![CompareOption::SameBase])
            .await
        {
            Err(CalculateBuildError::NotFound) => continue,
            e => e?,
        };
        results.push(item);
    }

    let f = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&cli.output)?;

    Ok(serde_json::to_writer(f, &results)?)
}
