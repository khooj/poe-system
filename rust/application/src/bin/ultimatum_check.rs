use application::ultimatum::*;
use clap::Parser;
use poeninja::Client;
use std::env;
use tradeapi::poe1::{
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
    let it = EveryNElements::new(10, search_res.result.iter());
    let mut ultimatums = Vec::with_capacity(200);
    for els in it {
        println!("fetching next elements from trade");
        loop {
            let fetch_res = trade_client
                .fetch_results(els.iter().map(|e| (*e).clone()).collect(), &search_res.id)
                .await;
            if matches!(fetch_res, Err(TradeClientError::TooManyRequestsOrSimilar)) {
                println!("cycle");
                continue;
            }
            let fetch_res = fetch_res?;
            let m = fetch_res
                .result
                .into_iter()
                .map(|e| map_ultimatum(e.item))
                .filter(|e| !e.currency);
            ultimatums.extend(m);
            break;
        }
        println!("ultimatums: {}", ultimatums.len());
    }

    let mut client = Client::new(
        "Mozilla/5.0 (X11; Linux x86_64; rv:127.0) Gecko/20100101 Firefox/127.0",
        &poesessid,
        "Settlers",
    );
    let divcards = client.get_items("DivinationCard").await.unwrap().lines;

    let uniques = [
        "UniqueWeapon",
        "UniqueArmour",
        "UniqueAccessory",
        "UniqueFlask",
        "UniqueJewel",
    ];

    println!("fetching data from poe.ninja");
    let mut uniques_res = vec![];
    for i in uniques {
        println!("fetching {}", i);
        let resp = client.get_items(i).await.unwrap();
        uniques_res.extend(resp.lines);
    }

    let divcards = process_poeninja_resp(divcards);
    let uniques = process_poeninja_resp(uniques_res);
    println!(
        "poe.ninja items fetched: {}",
        uniques.len() + divcards.len()
    );

    for ulti in ultimatums {
        if ulti.div {
            let div = &divcards[&ulti.sacrifice];
            println!(
                "(Div): {} {}: cost for one card is {} chaos or {} div",
                ulti.sacrifice, ulti.count, div.chaos_price, div.div_price
            );
        } else {
            if !uniques.contains_key(&ulti.sacrifice) || !uniques.contains_key(&ulti.reward) {
                eprintln!(
                    "\x1b[31mSacrifice or reward data unknown for {} -> {}\x1b[0m",
                    ulti.sacrifice, ulti.reward
                );
                continue;
            }
            let sacr = &uniques[&ulti.sacrifice];
            let rew = &uniques[&ulti.reward];
            let color = if rew.chaos_price > sacr.chaos_price {
                "\x1b[32m"
            } else {
                "\x1b[31m"
            };
            println!(
                "{}(Item): {} ({} chaos or {} div) into {} ({} chaos or {} div)\x1b[0m",
                color,
                ulti.sacrifice,
                sacr.chaos_price,
                sacr.div_price,
                ulti.reward,
                rew.chaos_price,
                rew.div_price
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_conrol() {
        println!("\x1b[31mTest\x1b[0m");
    }

    #[test]
    fn price_partial_ord() {
        let price1 = Price::Range {
            min: 0.10,
            max: 10.5,
        };
        let price2 = Price::Range {
            min: 0.01,
            max: 8.5,
        };
        let price3 = Price::Static(9.5);
        assert!(price1 > price2);
        assert!(price3 > price2);
        assert!(price1 > price3);
    }
}
