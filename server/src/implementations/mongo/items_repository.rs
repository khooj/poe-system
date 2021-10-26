use crate::ports::outbound::public_stash_retriever::{Item, PublicStashData};
use anyhow::Result;
use mongodb::{
    bson::{bson, doc, from_document, to_document},
    options::{
        DeleteOptions, DistinctOptions, FindOneOptions, CreateIndexOptions, InsertManyOptions,
        InsertOneOptions, UpdateOptions,
    },
    Client, IndexModel,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::runtime::Builder;
use tracing::{debug, error};

#[derive(Deserialize, Serialize)]
pub struct LatestStashId {
    pub latest_stash_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DbItem {
    #[serde(flatten)]
    pub item: Item,
    pub account_name: Option<String>,
    pub stash: Option<String>,
}

#[derive(Clone)]
pub struct ItemsRepository {
    client: Client,
    database: String,
}

impl ItemsRepository {
    pub async fn new(client: Client, database: String) -> Result<ItemsRepository> {
        let db = client.database(&database);
        let items_col = db.collection::<DbItem>("items");
        let _ = items_col
            .create_index(
                IndexModel::builder()
                    .keys(doc! {
                        "account_name": 1,
                        "stash": 1,
                    })
                    .build(),
                CreateIndexOptions::builder().build(),
            )
            .await?;

        Ok(ItemsRepository { client, database })
    }

    pub async fn set_stash_id(&self, id_: &str) -> Result<()> {
        let db = self.client.database(&self.database);
        let col = db.collection("stash_id");
        let opts = FindOneOptions::builder()
            .max_time(Duration::from_secs(1))
            .build();

        let result = col.find_one(doc! {}, opts).await?;

        if result.is_none() {
            let opts = InsertOneOptions::builder().build();
            let doc = to_document(&LatestStashId {
                latest_stash_id: id_.to_owned(),
            })?;
            let _ = col.insert_one(doc, opts).await?;
            return Ok(());
        }

        let opts = UpdateOptions::builder().build();
        let _ = col
            .update_one(doc! {}, doc! { "$set": {"latest_stash_id": id_} }, opts)
            .await?;
        Ok(())
    }

    pub async fn get_stash_id(&self) -> Result<LatestStashId> {
        let db = self.client.database(&self.database);
        let col = db.collection("stash_id");
        let opts = FindOneOptions::builder()
            .max_time(Duration::from_secs(1))
            .build();
        let result = col.find_one(doc! {}, opts).await?;
        result
            .ok_or(anyhow::anyhow!("cant find stash id"))
            .and_then(|r| from_document(r).map_err(|e| e.into()))
    }

    pub fn get_stash_id_blocking(&self) -> Result<LatestStashId> {
        let rt = Builder::new_current_thread().build()?;
        rt.block_on(self.get_stash_id())
    }

    pub async fn insert_raw_item(&self, public_data: PublicStashData) -> Result<()> {
        self.insert_raw_item_impl2(public_data).await
    }

    async fn insert_raw_item_impl2(&self, public_data: PublicStashData) -> Result<()> {
        let db = self.client.database(&self.database);
        let col = db.collection("items");

        let mut new_items = vec![];
        let mut delete_stashes = vec![];
        let mut len = 0;

        debug!("processing items");
        for d in &public_data.stashes {
            delete_stashes.push((d.account_name.as_ref(), d.stash.as_ref()));

            let mut items = d
                .items
                .iter()
                .map(|i| DbItem {
                    item: i.clone(),
                    account_name: d.account_name.clone(),
                    stash: d.stash.clone(),
                })
                .filter_map(|i| match to_document(&i) {
                    Ok(k) => Some(k),
                    Err(e) => {
                        let js = serde_json::to_string(&i.item).unwrap();
                        debug!(item = ?i, js = %js);
                        error!("error: {}", e);
                        None
                    }
                })
                .collect::<Vec<_>>();

            len += d.items.len();

            if items.is_empty() {
                continue;
            }

            new_items.append(&mut items);
        }

        if len != new_items.len() {
            return Err(anyhow::anyhow!("not all new items correctly processed"));
        }

        debug!(
            "making requests to mongo: delete_stashes {} new_items {}",
            delete_stashes.len(),
            new_items.len()
        );
        let opts = DeleteOptions::builder().build();
        let filter = delete_stashes.into_iter().fold(bson!([]), |mut acc, x| {
            let m = acc.as_array_mut().unwrap();
            m.push(bson!({
                "account_name": { "$eq": x.0.unwrap() },
                "stash": { "$eq": x.1.unwrap() },
            }));
            acc
        });

        let _ = col
            .delete_many(
                doc! {
                    "$or": filter.as_array().unwrap(),
                },
                opts,
            )
            .await?;

        if new_items.is_empty() {
            return Ok(());
        }

        let opts = InsertManyOptions::builder().ordered(false).build();
        let _ = col.insert_many(new_items, opts).await?;

        self.set_stash_id(&public_data.next_change_id).await?;
        Ok(())
    }

    pub fn insert_raw_item_blocking(&self, public_data: PublicStashData) -> Result<()> {
        let rt = Builder::new_current_thread().build()?;
        rt.block_on(self.insert_raw_item(public_data))
    }

    pub async fn get_available_maps(&self) -> Result<Vec<String>> {
        let db = self.client.database(&self.database);
        let col = db.collection::<DbItem>("items");

        let opts = DistinctOptions::builder().build();
        let filter = doc! {
            "base_type": {
                "$text": {
                    "$search": "Map",
                    "$caseSensetive": true,
                },
            },
        };
        let result = col.distinct("base_type", filter, opts).await?;
        Ok(result
            .into_iter()
            .map(|el| el.as_str().unwrap().to_owned())
            .collect())
    }
}

#[cfg(test)]
mod test {
    use crate::ports::outbound::public_stash_retriever::PublicStashChange;
    use anyhow::Result;
    use mongodb::bson::to_document;

    const EXAMPLE_STASH_CHANGE: &str = include_str!("example-stash.json");

    #[test]
    fn bson_doc_test() -> Result<()> {
        let k: PublicStashChange = serde_json::from_str(&EXAMPLE_STASH_CHANGE)?;
        let l = k.items.len();
        let i = k
            .items
            .into_iter()
            .filter_map(|e| match to_document(&e) {
                Ok(k) => Some(k),
                Err(e) => {
                    println!("bson err: {:?}", e);
                    None
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(i.len(), l);
        Ok(())
    }
}
