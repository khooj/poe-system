use clap::Parser;
use std::io::prelude::*;
use std::env;
use std::collections::HashMap;
use std::cmp::{PartialOrd, Ordering};
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
    count: String,
}

#[derive(serde::Deserialize)]
struct Property {
    name: String,
    values: Vec<Vec<serde_json::Value>>,
}

fn map_ultimatum(item: ClientFetchItem) -> Option<Ultimatum> {
    let vv: Vec<Property> = serde_json::from_value(item.rest["properties"].clone()).unwrap();
    let v: Vec<String> = vv.into_iter().map(|e| (e.name, e.values)).filter(|e| e.0.contains("Requires Sacrifice") || e.0.contains("Reward")).flat_map(|e| {
        if e.1.len() == 2 {
            vec![
                e.1[0][0].as_str().unwrap().to_string(),
                e.1[1][0].as_str().unwrap().to_string(),
            ]
        } else {
            vec![e.1[0][0].as_str().unwrap().to_string()]
        }
    }).collect();

    if v[1].contains("Currency") || v.get(2).unwrap_or(&String::new()).contains("Currency") {
        return None;
    }

    let div = v[1].contains("Divination") || v.get(2).unwrap_or(&String::new()).contains("Divination");
    let (sacrifice, reward, count) = if div {
        let v0 = v[0].clone();
        let v1 = v[1].clone();
        let v2 = v[2].clone();
        (v0, v2, v1)
    } else {
        (v[0].clone(), v[1].clone(), String::new())
    };

    Some(Ultimatum {
        sacrifice,
        reward,
        div,
        count,
    })
}

#[derive(PartialEq)]
enum Price {
    Range { min: f32, max: f32 },
    Static(f32),
}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Price) -> Option<Ordering> {
        Some(match self {
            Price::Range { max, .. } => match other {
                Price::Range { max: max2, .. } if max > max2 => Ordering::Greater,
                Price::Static(val) if max > val => Ordering::Greater,
                _ => Ordering::Less,
            },
            Price::Static(val) => match other {
                Price::Range { max, .. } if val > max => Ordering::Greater,
                Price::Static(val2) if val > val2 => Ordering::Greater,
                _ => Ordering::Less,
            },
        })
    }
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

struct EveryNElements<T> {
    n: usize,
    iter: T,
}

impl<T, K> Iterator for EveryNElements<T>
where T: Iterator<Item = K> 
{
    type Item = Vec<K>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut v = Vec::with_capacity(self.n);
        for _ in 0..self.n {
            let i = self.iter.next();
            if i.is_some() {
                v.push(i.unwrap());
            } else {
                break;
            }
        }
        if v.is_empty() {
            None
        } else {
            Some(v)
        }
    }
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
    let it = EveryNElements { n: 10, iter: search_res.result.iter() };
    let mut ultimatums = Vec::with_capacity(200);
    for els in it {
        println!("fetching next elements from trade");
        loop {
            let fetch_res = trade_client.fetch_results(els.iter().map(|e| (*e).clone()).collect(), &search_res.id).await;
            if matches!(fetch_res, Err(TradeClientError::NextCycle)) {
                println!("cycle");
                continue;
            }
            let fetch_res = fetch_res?;
            let m = fetch_res.result.into_iter().filter_map(|e| map_ultimatum(e.item));
            ultimatums.extend(m);
            break;
        }
        println!("ultimatums: {}", ultimatums.len());
    }

    let mut client = Client::new("Mozilla/5.0 (X11; Linux x86_64; rv:127.0) Gecko/20100101 Firefox/127.0", &poesessid, "Settlers");
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
    println!("poe.ninja items fetched: {}", uniques.len() + divcards.len());

    for ulti in ultimatums {
        if ulti.div {
            let div = &divcards[&ulti.sacrifice];
            println!("(Div): {} {}: cost for one card is {} chaos or {} div", ulti.sacrifice, ulti.count, div.chaos_price, div.div_price);
        } else {
            if !uniques.contains_key(&ulti.sacrifice) || !uniques.contains_key(&ulti.reward) {
                eprintln!("\x1b[31mSacrifice or reward data unknown for {} -> {}\x1b[0m", ulti.sacrifice, ulti.reward);
                continue;
            }
            let sacr = &uniques[&ulti.sacrifice];
            let rew = &uniques[&ulti.reward];
            let color = if rew.chaos_price > sacr.chaos_price {
                "\x1b[32m"
            } else {
                "\x1b[31m"
            };
            println!("{}(Item): {} ({} chaos or {} div) into {} ({} chaos or {} div)\x1b[0m", color, ulti.sacrifice, sacr.chaos_price, sacr.div_price, ulti.reward, rew.chaos_price, rew.div_price);
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
        let price1 = Price::Range { min: 0.10, max: 10.5 };
        let price2 = Price::Range { min: 0.01, max: 8.5 };
        let price3 = Price::Static(9.5);
        assert!(price1 > price2);
        assert!(price3 > price2);
        assert!(price1 > price3);
    }
}
