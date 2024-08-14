use application::ultimatum::*;
use clap::Parser;
use std::env;
use std::{collections::HashMap, time::Duration};
use tradeapi::{
    query::{Builder, StatusOption, TradeFilters},
    Client as TradeClient, ClientError as TradeClientError,
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

    let mut trade_client = TradeClient::new(
        "Mozilla/5.0 (X11; Linux x86_64; rv:127.0) Gecko/20100101 Firefox/127.0",
        &poesessid,
        "Settlers",
    );
    let mut builder = Builder::new();
    builder.set_type("Inscribed Ultimatum");
    builder.set_status(StatusOption::Any);
    builder.set_trade_filters(
        TradeFilters::default()
            .set_account(&cli.account_name)
            .set_sale_type("any"),
    );

    println!("fetching ultimatums from trade search");
    let search_res = trade_client.get_search_id(&builder).await?;
    println!(
        "search res size: {} total: {}",
        search_res.result.len(),
        search_res.total
    );
    let it = EveryNElements::new(10, search_res.result.iter());
    let mut ultimatums = Vec::with_capacity(200);
    for els in it {
        println!("fetching next elements from trade");
        loop {
            let fetch_res = trade_client
                .fetch_results(els.iter().map(|e| (*e).clone()).collect(), &search_res.id)
                .await;
            if matches!(fetch_res, Err(TradeClientError::TooManyRequestsOrSimilar)) {
                println!("too many requests");
                tokio::time::sleep(Duration::from_millis(500)).await;
                continue;
            }
            let fetch_res = fetch_res?;
            let m = fetch_res
                .result
                .into_iter()
                .map(|e| map_ultimatum(e.item))
                .filter(|e| e.currency);
            ultimatums.extend(m);
            break;
        }
        println!("ultimatums with currency: {}", ultimatums.len());
    }

    let collected_currency_sacrifice = ultimatums.into_iter().fold(HashMap::new(), |mut acc, x| {
        let v = acc.entry(x.sacrifice.clone()).or_insert(0);
        let c = x.count[1..].parse::<u32>().unwrap();
        *v += c;
        acc
    });

    for (k, v) in collected_currency_sacrifice {
        println!("{}: {}", k, v);
    }

    Ok(())
}
