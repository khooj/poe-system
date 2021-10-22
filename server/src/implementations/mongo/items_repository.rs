use crate::ports::outbound::public_stash_retriever::{Item, ItemProperty, PublicStashData};
use anyhow::Result;
use mongodb::{
    bson::{doc, from_document, to_document},
    options::{DeleteOptions, FindOneOptions, InsertManyOptions, InsertOneOptions, UpdateOptions},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;
use tokio::runtime::Runtime;
use tracing::{debug, error};

#[derive(Deserialize, Serialize)]
pub struct LatestStashId {
    #[serde(rename = "_id")]
    pub id: Option<String>,
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
    pub client: Client,
    pub database: String,
}

impl ItemsRepository {
    pub async fn set_stash_id(&self, id_: &str) -> Result<()> {
        let db = self.client.database(&self.database);
        let col = db.collection("stash_id");
        let opts = FindOneOptions::builder()
            .max_time(Duration::from_secs(1))
            .build();

        let result = col
            .find_one(doc! { "latest_stash_id": { "$eq": id_ } }, opts)
            .await?;

        if result.is_none() {
            let opts = InsertOneOptions::builder().build();
            let doc = to_document(&LatestStashId {
                id: Some("uniqueid".into()),
                latest_stash_id: id_.to_owned(),
            })?;
            let _ = col.insert_one(doc, opts).await?;
            return Ok(());
        }

        let opts = UpdateOptions::builder().build();
        let doc = result.unwrap();
        let id = from_document::<LatestStashId>(doc)?;
        let d = to_document(&LatestStashId {
            id: id.id.clone(),
            latest_stash_id: id_.to_owned(),
        })?;
        let _ = col
            .update_one(doc! { "_id": { "$eq": id.id.unwrap() }}, d, opts)
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
        let rt = Runtime::new()?;
        rt.block_on(self.get_stash_id())
    }

    fn fix_unsigned(mut item: Item) -> Item {
        let func = |props: Option<Vec<ItemProperty>>| {
            if props.is_none() {
                return props;
            }

            let props = props.unwrap();

            let props = props
                .into_iter()
                .map(|mut p| {
                    p.values = p
                        .values
                        .into_iter()
                        .map(|v| {
                            v.into_iter()
                                .map(|k| {
                                    if let Value::Number(n) = k {
                                        if n.is_u64() {
                                            return json!(n.as_i64().unwrap());
                                        } else {
                                            Value::Number(n)
                                        }
                                    } else {
                                        k
                                    }
                                })
                                .collect::<Vec<Value>>()
                        })
                        .collect::<Vec<_>>();
                    p
                })
                .collect::<Vec<_>>();

            Some(props)
        };

        item.properties = func(item.properties);
        item.notable_properties = func(item.notable_properties);
        item.requirements = func(item.requirements);
        item.additional_properties = func(item.additional_properties);
        item.next_item_requirements = func(item.next_item_requirements);

        item
    }

    pub async fn insert_raw_item(&self, public_data: &PublicStashData) -> Result<()> {
        let db = self.client.database(&self.database);
        let col = db.collection("items");

        for d in &public_data.stashes {
            let opts = DeleteOptions::builder().build();
            let r = col
                .delete_many(
                    doc! {
                        "account_name": { "$eq": d.account_name.as_ref().unwrap() },
                        "stash": { "$eq": d.stash.as_ref().unwrap() },
                    },
                    opts,
                )
                .await?;

            if d.items.is_empty() {
                continue;
            }

            let mut items = vec![];
            for i in d.items.iter().map(|i| DbItem {
                item: ItemsRepository::fix_unsigned(i.clone()),
                account_name: d.account_name.clone(),
                stash: d.stash.clone(),
            }) {
                let d = match to_document(&i) {
                    Ok(k) => k,
                    Err(e) => {
                        let js = serde_json::to_string(&i.item).unwrap();
                        debug!(item = ?i, js = %js);
                        return Err(e.into());
                    }
                };
                items.push(d);
            }
            debug!(items = ?items);

            let opts = InsertManyOptions::builder().build();
            let _ = col.insert_many(items, opts).await?;
        }

        self.set_stash_id(&public_data.next_change_id).await?;
        Ok(())
    }

    pub fn insert_raw_item_blocking(&self, public_data: &PublicStashData) -> Result<()> {
        let rt = Runtime::new()?;
        rt.block_on(self.insert_raw_item(public_data))
    }
}

#[cfg(test)]
mod test {
    use super::ItemsRepository;
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
            .map(|i| ItemsRepository::fix_unsigned(i))
            .filter_map(|e| to_document(&e).ok())
            .collect::<Vec<_>>();
        assert!(!i.is_empty());
        assert_eq!(i.len(), l);
        Ok(())
    }
}
