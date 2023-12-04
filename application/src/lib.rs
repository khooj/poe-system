use std::{
    env::{self, VarError},
    sync::Arc,
};

use domain::Item;
use futures::future;
use pob::{Pob, PobError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::Mutex;
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
    #[error("var error: {0}")]
    VarError(#[from] VarError),
    #[error("item not found")]
    NotFound,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CompareOption {
    SameBase,
    SameSockets,
    ModSimilarity { idx: i32, percent: i32 },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ItemCost {
    Chaos(f32),
    Divine(f32),
    Another(String, f32),
}

fn parse_price(listing: ClientFetchListing) -> ItemCost {
    let price = listing.price.amount;
    match listing.price.currency.as_str() {
        "divine" => ItemCost::Divine(price),
        "chaos" => ItemCost::Chaos(price),
        _ => ItemCost::Another(listing.price.currency, price),
    }
}

#[derive(Clone)]
pub struct FoundItem {
    pub item: ClientFetchItem,
    pub original_item: Item,
    pub price: ItemCost,
}

#[derive(Clone)]
pub struct CalculatingState {
    pob: Pob,
    client: Arc<Mutex<Client>>,
}

impl CalculatingState {
    pub fn parse_pob(data: String) -> Result<Self, PobError> {
        let pob = Pob::from_pastebin_data(data)?;
        let poesessid = env::var("POESESSID").unwrap();
        let client = Client::new(
            "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/110.0".to_string(),
            &poesessid,
            "Standard",
        );
        Ok(CalculatingState {
            pob,
            client: Arc::new(Mutex::new(client)),
        })
    }

    pub fn itemsets(&self) -> Vec<String> {
        if let Ok(doc) = self.pob.as_document() {
            doc.get_item_sets()
                .into_iter()
                .map(|f| f.title().to_string())
                .collect()
        } else {
            vec![]
        }
    }

    pub fn items(&self, itemset: &str) -> Vec<Item> {
        if let Ok(doc) = self.pob.as_document() {
            doc.get_itemset(itemset).unwrap().items().clone()
        } else {
            vec![]
        }
    }

    pub fn max_progress(&self, itemset_title: &str) -> usize {
        if let Ok(doc) = self.pob.as_document() {
            if let Ok(itemset) = doc.get_itemset(itemset_title) {
                return itemset.items().len();
            }
        }
        0
    }

    pub async fn calculate_item_cost(
        &self,
        compare_item: Item,
    ) -> Result<FoundItem, CalculateBuildError> {
        let mut builder = Builder::new();
        builder.set_type(&compare_item.base_type);

        let search_id;
        loop {
            let mut client = self.client.lock().await;
            match client.get_search_id(&builder).await {
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
            let mut client = self.client.lock().await;
            let r = client
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
            return Err(CalculateBuildError::NotFound);
        }

        let item = results.result[0].clone();
        let price = parse_price(item.listing);
        Ok(FoundItem {
            original_item: compare_item,
            item: item.item,
            price,
        })
    }

    pub async fn calculate_build_cost(
        &self,
        itemset: &str,
    ) -> Result<Vec<FoundItem>, CalculateBuildError> {
        let items = if let Ok(doc) = self.pob.as_document() {
            doc.get_itemset(itemset).unwrap().items().clone()
        } else {
            vec![]
        };

        let fs = items.into_iter().map(|i| self.calculate_item_cost(i));
        future::try_join_all(fs).await
    }
}
