use clap::Parser;
use std::io::prelude::*;
use std::env;
use poeninja::{Client, ClientError, models::Response};
use tradeapi::{Client as TradeClient, ClientError as TradeClientError, 
    models::{ClientFetchItem, ClientFetchListing},
    query::{Builder, BuilderError, TypeFilters, StatusOption, TradeFilters},
};

#[derive(Parser, Debug)]
struct Args {
    //#[arg(short)]
    //stash_name: String,
    account_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Args::parse();
    let poesessid = env::var("POESESSID").unwrap_or_default();
    if poesessid.is_empty() {
        eprintln!("POESESSID env variable is empty!");
        Err(anyhow::anyhow!("error"))?;
        unreachable!();
    }

    let mut client = Client::new("Mozilla/5.0 (X11; Linux x86_64; rv:127.0) Gecko/20100101 Firefox/127.0", &poesessid, "Settlers");
    let items = client.get_items("UniqueWeapon").await.unwrap();
    println!("items fetched: {}", items.lines.len());

    let mut trade_client = TradeClient::new("Mozilla/5.0 (X11; Linux x86_64; rv:127.0) Gecko/20100101 Firefox/127.0", &poesessid, "Settlers");
    let mut builder = Builder::new();
    builder.set_type("Inscribed Ultimatum");
    builder.set_status(StatusOption::Any);
    builder.set_trade_filters(TradeFilters::default().set_account(&cli.account_name).set_sale_type("any"));

    let search_res = trade_client.get_search_id(&builder).await?;
    let fetch_res = trade_client.fetch_results(search_res.result.iter().take(5).cloned().collect(), &search_res.id).await?;

    println!("trade search for inscribed ultimatums: {}", fetch_res.result.len());
    Ok(())
}
