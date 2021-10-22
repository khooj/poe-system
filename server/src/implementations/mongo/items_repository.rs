use crate::ports::outbound::public_stash_retriever::{Item, PublicStashData};
use anyhow::Result;
use mongodb::{
    bson::{doc, to_document},
    options::{DeleteOptions, FindOneOptions, InsertManyOptions, InsertOneOptions, UpdateOptions},
    Client,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::runtime::Runtime;

#[derive(Deserialize, Serialize)]
pub struct LatestStashId {
    pub id: Option<String>,
    pub latest_stash_id: String,
}

#[derive(Deserialize, Serialize)]
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
    pub async fn set_stash_id(&self, id_: &str) -> Result<()> {
        let db = self.client.database(&self.database);
        let col = db.collection::<LatestStashId>("stash_id");
        let opts = FindOneOptions::builder()
            .max_time(Duration::from_secs(1))
            .build();

        let result = col
            .find_one(doc! { "latest_stash_id": { "$eq": id_ } }, opts)
            .await?;

        if result.is_none() {
            let opts = InsertOneOptions::builder().build();
            let _ = col
                .insert_one(
                    LatestStashId {
                        id: None,
                        latest_stash_id: id_.to_owned(),
                    },
                    opts,
                )
                .await?;
            return Ok(());
        }

        let opts = UpdateOptions::builder().build();
        let id = result.unwrap().id;
        let d = to_document(&LatestStashId {
            id: id.clone(),
            latest_stash_id: id_.to_owned(),
        })?;
        let _ = col
            .update_one(doc! { "_id": { "$eq": id.unwrap() }}, d, opts)
            .await?;
        Ok(())
    }

    pub async fn get_stash_id(&self) -> Result<LatestStashId> {
        let db = self.client.database(&self.database);
        let col = db.collection::<LatestStashId>("stash_id");
        let opts = FindOneOptions::builder()
            .max_time(Duration::from_secs(1))
            .build();
        let result = col.find_one(doc! {}, opts).await?;
        result.ok_or(anyhow::anyhow!("cant find stash id"))
    }

    pub fn get_stash_id_blocking(&self) -> Result<LatestStashId> {
        let rt = Runtime::new()?;
        rt.block_on(self.get_stash_id())
    }

    pub async fn insert_raw_item(&self, public_data: &PublicStashData) -> Result<()> {
        let db = self.client.database(&self.database);
        let col = db.collection::<DbItem>("items");

        for d in &public_data.stashes {
            let opts = DeleteOptions::builder().build();
            let r = col
                .delete_many(
                    doc! {
                        "account_name": { "$eq": &d.account_name },
                        "stash": { "$eq": &d.stash },
                    },
                    opts,
                )
                .await?;

            let items = d
                .items
                .iter()
                .map(|i| DbItem {
                    item: i.clone(),
                    account_name: d.account_name.clone(),
                    stash: d.stash.clone(),
                })
                .collect::<Vec<_>>();

            let opts = InsertManyOptions::builder().build();
            let _ = col.insert_many(&items, opts).await?;
        }

        self.set_stash_id(&public_data.next_change_id).await?;
        Ok(())
    }

    pub fn insert_raw_item_blocking(&self, public_data: &PublicStashData) -> Result<()> {
        let rt = Runtime::new()?;
        rt.block_on(self.insert_raw_item(public_data))
    }
}
