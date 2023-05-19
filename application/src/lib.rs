use std::{
    env,
    io::{BufWriter, Write},
};

use domain::Item;
use pob::{Pob, PobError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tradeapi::{
    models::{ClientFetchItem, ClientFetchListing},
    query::{Builder, BuilderError, StatQuery, StatQueryType, TypeFilters},
    Client, ClientError,
};

#[derive(Error, Debug)]
pub enum CalculateBuildError {
    #[error("pob parsing error")]
    PobError(#[from] PobError),
    #[error("api client error")]
    ClientError(#[from] ClientError),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum CompareOption {
    SameBase,
    SameSockets,
    ModSimilarity { idx: i32, percent: i32 },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum ItemCost {
    Chaos(f32),
    Divine(f32),
    Another(String, f32),
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
struct CompareItem {
    original_item: Item,
    compare_options: Vec<CompareOption>,
    found_item: Option<ClientFetchItem>,
    cost: Option<ItemCost>,
    tried: bool,
}

impl CompareItem {
    pub fn new(item: Item) -> Self {
        CompareItem {
            original_item: item,
            ..Default::default()
        }
    }
}

fn parse_price(listing: ClientFetchListing) -> ItemCost {
    let price = listing.price.amount;
    match listing.price.currency.as_str() {
        "divine" => ItemCost::Divine(price),
        "chaos" => ItemCost::Chaos(price),
        _ => ItemCost::Another(listing.price.currency, price),
    }
}

const ITEMSET_FILENAME: &str = "saved_itemset.json";

pub async fn calculate_build_cost(pob_base64: &str) -> Result<(), CalculateBuildError> {
    let pob = Pob::from_pastebin_data(pob_base64.to_string())?;
    let pob_doc = pob.as_document()?;
    let first_itemset = pob_doc.get_first_itemset()?;

    let poesessid = env::var("POESESSID").expect("poesessid not defined");
    let mut api_client = Client::new(
        "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/110.0".to_string(),
        &poesessid,
        "Crucible",
    );

    let mut itemset = if std::fs::metadata(ITEMSET_FILENAME).is_ok() {
        let f = std::fs::read(ITEMSET_FILENAME).unwrap();
        let i: Vec<CompareItem> = serde_json::from_slice(&f).unwrap();
        println!("loaded itemset with {} items", i.len());
        i
    } else {
        vec![]
    };
    if itemset.is_empty() {
        for item in first_itemset.items() {
            let compare_item = CompareItem::new(item.clone());
            itemset.push(compare_item);
        }
    }

    let mut result_itemset = vec![];
    for mut compare_item in itemset {
        if compare_item.tried {
            println!(
                "skipping item: {} {}",
                compare_item.original_item.name, compare_item.original_item.base_type
            );
            result_itemset.push(compare_item);
            continue;
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        println!(
            "searching for item: {} {}",
            compare_item.original_item.name, compare_item.original_item.base_type
        );
        let mut builder = Builder::new();
        builder.set_type(&compare_item.original_item.base_type);

        let search_id;
        loop {
            match api_client.get_search_id(&builder).await {
                Ok(k) => {
                    search_id = k;
                    break;
                }
                Err(ClientError::NextCycle) => continue,
                e @ Err(..) => e?,
            };
        }

        let results;
        loop {
            let r = api_client
                .fetch_results(
                    search_id.result.iter().take(5).cloned().collect(),
                    &search_id.id,
                )
                .await;

            match r {
                Ok(k) => {
                    results = k;
                    break;
                }
                Err(ClientError::NextCycle) => continue,
                e @ Err(..) => e?,
            };
        }

        if results.result.is_empty() {
            continue;
        }

        let item = results.result[0].clone();
        compare_item.found_item = Some(item.item);
        compare_item.cost = Some(parse_price(item.listing));
        compare_item.tried = true;
        println!("found");
        result_itemset.push(compare_item);
        let wr = serde_json::to_vec(&result_itemset).unwrap();
        std::fs::write(ITEMSET_FILENAME, wr).unwrap();
    }

    println!("found items: {:?}", result_itemset);
    Ok(())
}
