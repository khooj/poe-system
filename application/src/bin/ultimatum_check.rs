use clap::Parser;
use std::io::prelude::*;
use std::env;
use std::collections::HashMap;
use poeninja::{Client, ClientError, models::{Response, Object}};
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

struct Ultimatum {
    sacrifice: String,
    reward: String,
    div: bool,
}

#[derive(serde::Deserialize)]
struct Property {
    name: String,
    values: Vec<Vec<serde_json::Value>>,
}

fn map_ultimatum(item: ClientFetchItem) -> Option<Ultimatum> {
    let vv: Vec<Property> = serde_json::from_value(item.rest["properties"].clone()).unwrap();
    let v: Vec<String> = vv.into_iter().map(|e| (e.name, e.values)).filter(|e| e.0.contains("Requires Sacrifice") || e.0.contains("Reward")).map(|e| e.1[0][0].as_str().unwrap().to_string()).collect();

    if v[1].contains("Currency") {
        return None;
    }

    let div = v[1].contains("Divination");

    Some(Ultimatum {
        sacrifice: v[0].to_string(),
        reward: v[1].to_string(),
        div,
    })
}

enum Price {
    Range { min: f32, max: f32 },
    Static(f32),
}

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Price::Range { min, max } => write!(f, "{} - {}", min, max),
            Price::Static(val) => write!(f, "{}", val),
        }
    }
}

struct Data {
    chaos_price: Price,
    div_price: Price,
}

fn get_price<T>(v: &Vec<Object>, f: T) -> Price 
where T: Fn(&Object) -> f32 {
    let min = v.iter().map(|e| f(e)).reduce(f32::min).unwrap();
    let max = v.iter().map(f).reduce(f32::max).unwrap();
    if min != max {
        Price::Range { min, max }
    } else {
        Price::Static(min)
    }
}

fn process_poeninja_resp(items: Vec<Object>) -> HashMap<String, Data> {
    let grouped = items.into_iter().fold(HashMap::new(), |mut acc, x| {
        let v = acc.entry(x.name.clone()).or_insert(vec![]);
        v.push(x);
        acc
    });

    let mut res = HashMap::new();
    for (k, v) in grouped {
        let chaos_price = get_price(&v, |e| e.chaos_value);
        let div_price = get_price(&v, |e| e.divine_value);
        res.insert(k, Data { chaos_price, div_price });
    }
    res
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

    let mut trade_client = TradeClient::new("Mozilla/5.0 (X11; Linux x86_64; rv:127.0) Gecko/20100101 Firefox/127.0", &poesessid, "Settlers");
    let mut builder = Builder::new();
    builder.set_type("Inscribed Ultimatum");
    builder.set_status(StatusOption::Any);
    builder.set_trade_filters(TradeFilters::default().set_account(&cli.account_name).set_sale_type("any"));

    println!("fetching ultimatums from trade search");
    let search_res = trade_client.get_search_id(&builder).await?;
    let fetch_res = trade_client.fetch_results(search_res.result.iter().take(5).cloned().collect(), &search_res.id).await?;
    let ultimatums: Vec<_> = fetch_res.result.into_iter().filter_map(|e| map_ultimatum(e.item)).collect();

    println!("ultimatums: {}", ultimatums.len());

    let mut client = Client::new("Mozilla/5.0 (X11; Linux x86_64; rv:127.0) Gecko/20100101 Firefox/127.0", &poesessid, "Settlers");
    let divcards = client.get_items("DivinationCard").await.unwrap().lines;

    let uniques = [
        "UniqueWeapon",
        "UniqueArmour",
        //"UniqueAccessory",
        //"UniqueFlask",
        //"UniqueJewel",
    ];

    println!("fetching data from poe.ninja");
    let mut uniques_res = vec![];
    for i in uniques {
        let resp = client.get_items(i).await.unwrap();
        uniques_res.extend(resp.lines);
    }

    let divcards = process_poeninja_resp(divcards);
    let uniques = process_poeninja_resp(uniques_res);
    println!("poe.ninja items fetched: {}", uniques.len() + divcards.len());

    for ulti in ultimatums {
        if ulti.div {
            let div = &divcards[&ulti.sacrifice];
            println!("(Div): {}: cost {} chaos or {} div", ulti.sacrifice, div.chaos_price, div.div_price);
        } else {
            if !uniques.contains_key(&ulti.sacrifice) || !uniques.contains_key(&ulti.reward) {
                eprintln!("Sacrifice or reward data unknown for {} -> {}", ulti.sacrifice, ulti.reward);
                continue;
            }
            let sacr = &uniques[&ulti.sacrifice];
            let rew = &uniques[&ulti.reward];
            println!("(Item): {} ({} chaos or {} div) into {} ({} chaos or {} div)", ulti.sacrifice, sacr.chaos_price, sacr.div_price, ulti.reward, rew.chaos_price, rew.div_price);
        }
    }
    Ok(())
}
