use std::{
    collections::HashMap,
    env::{self, VarError},
    sync::Arc,
};

use domain::{Item, SocketColor};
use pob::{Pob, PobError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::Mutex;
use tradeapi::{
    models::{ClientFetchItem, ClientFetchListing},
    query::{Builder, BuilderError, SocketFilters, StatQuery, StatQueryType, TypeFilters},
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
    #[error("builder error")]
    BuilderError(#[from] BuilderError),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Percent(i32);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CompareOption {
    SameBase,
    SameSockets,
    ModSimilarity { idx: i32, percent: Percent },
}

impl CompareOption {
    fn apply(&self, item: &Item, builder: &mut Builder) -> Result<(), CalculateBuildError> {
        match self {
            CompareOption::SameBase => builder.set_type(&item.base_type),
            CompareOption::SameSockets => {
                let sock = item.sockets();
                let max_links = if sock.max_links() == 1 {
                    None
                } else {
                    Some(sock.max_links())
                };
                let colors = sock.colors();
                let sf = SocketFilters::default().set_sockets(
                    None,
                    None,
                    colors.get(&SocketColor::R),
                    colors.get(&SocketColor::G),
                    colors.get(&SocketColor::B),
                    colors.get(&SocketColor::W),
                );
                let sf = sf.set_links(max_links, max_links, None, None, None, None);
                builder.set_socket_filters(sf);
            }
            CompareOption::ModSimilarity { idx, percent } => {
                let modtext = &item.mods[*idx as usize];
                let replaced = modtext.text.replace(|c: char| c.is_alphanumeric(), "#");
                let replaced = replaced.chars().fold(vec![], |mut acc, c| {
                    if acc.is_empty() || *acc.last().unwrap() != '#' {
                        acc.push(c);
                    }
                    acc
                });
                let replaced = String::from_iter(replaced);
                let num = modtext.numeric_value.unwrap_or_default();
                let min = Some(num - num * percent.0 / 100);
                let max = Some(num + num * percent.0 / 100);
                let group = StatQuery::new().try_add_mod(&replaced, min, max, None)?;
                builder.add_stat_group(group);
            }
            _ => {}
        };
        Ok(())
    }
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

#[derive(Clone, Serialize)]
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
    pub fn parse_pastebin(data: String) -> Result<Self, PobError> {
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
        options: Vec<CompareOption>,
    ) -> Result<FoundItem, CalculateBuildError> {
        let mut builder = Builder::new();
        options
            .iter()
            .map(|co| co.apply(&compare_item, &mut builder))
            .find(|e| e.is_err())
            .unwrap_or(Ok(()))?;

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
        options: HashMap<String, Vec<CompareOption>>,
    ) -> Result<Vec<FoundItem>, CalculateBuildError> {
        let items = if let Ok(doc) = self.pob.as_document() {
            doc.get_itemset(itemset).unwrap().items().clone()
        } else {
            vec![]
        };

        let mut result = Vec::with_capacity(items.len());
        for i in items {
            let opts = options.get(&i.name).unwrap_or(&vec![]).clone();
            if let Ok(item) = self.calculate_item_cost(i, opts).await {
                result.push(item);
            }
        }
        Ok(result)
    }
}
