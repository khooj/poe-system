use crate::ports::outbound::public_stash_retriever::{Item, PropertyValueType, PublicStashData};
use anyhow::Result;
use mongodb::{
    bson::{bson, doc, from_document, to_document},
    options::{
        CreateIndexOptions, DeleteOptions, DistinctOptions, FindOneOptions, FindOptions,
        InsertManyOptions, InsertOneOptions, UpdateOptions,
    },
    Client, IndexModel,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};
use tokio::runtime::Builder;
use tracing::{debug, error};

#[derive(Deserialize, Serialize, Clone)]
pub struct OwnerInfo {
    pub tier: i32,
    pub count: i64,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
pub struct MapName(String);

#[derive(Deserialize, Serialize, Clone)]
pub struct MapInfo {
    pub account_name: String,
    pub owned: HashMap<MapName, OwnerInfo>,
}

#[derive(Clone)]
pub struct MapsRepository {
    client: Client,
    database: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ProcessedMap {
    #[serde(rename = "_id")]
    pub id: String,
}

impl MapsRepository {
    pub async fn new(client: Client, database: String) -> Result<Self> {
        let db = client.database(&database);

        let tiers_col = db.collection::<MapInfo>("map_info");
        let _ = tiers_col
            .create_indexes(
                vec![IndexModel::builder()
                    .keys(doc! {
                        "name": 1,
                    })
                    .build()],
                CreateIndexOptions::builder().build(),
            )
            .await?;
        Ok(MapsRepository { client, database })
    }

    pub async fn is_processed(&self, id: &str) -> Result<bool> {
        let db = self.client.database(&self.database);
        let proc_col = db.collection::<ProcessedMap>("processed");

        let result = proc_col
            .find_one(
                doc! {
                    "_id": { "$eq": id },
                },
                FindOneOptions::builder().build(),
            )
            .await?;

        Ok(result.is_some())
    }

    pub async fn add_processed(&self, ids: &[&str]) -> Result<()> {
        let db = self.client.database(&self.database);
        let proc_col = db.collection::<ProcessedMap>("processed");

        let _ = proc_col
            .insert_many(
                ids.iter()
                    .map(|el| ProcessedMap { id: el.to_string() })
                    .collect::<Vec<_>>(),
                InsertManyOptions::builder().build(),
            )
            .await?;

        Ok(())
    }

    pub async fn remove_processed(&self, ids: &[&str]) -> Result<()> {
        let db = self.client.database(&self.database);
        let proc_col = db.collection::<ProcessedMap>("processed");

        let mut f = bson!([]);
        for i in ids {
            f.as_array_mut().unwrap().push(bson!(
                {
                    "_id": i,
                }
            ));
        }

        let f = doc! {
            "$or": f.as_array().unwrap(),
        };
        let _ = proc_col
            .delete_many(f, DeleteOptions::builder().build())
            .await?;
        Ok(())
    }
}
