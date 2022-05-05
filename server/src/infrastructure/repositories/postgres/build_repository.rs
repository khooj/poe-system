use super::PgTransaction;
use crate::domain::item::Item;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{
    types::{
        chrono::{DateTime, Utc},
        uuid::Uuid,
        Json,
    },
    PgPool,
};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Default)]
pub struct BuildItems {
    pub weapon1: Item,
    pub weapon2: Item,
    pub helmet: Item,
    pub body: Item,
    pub belt: Item,
    pub gloves: Item,
    pub boots: Item,
    pub ring1: Item,
    pub ring2: Item,
    pub amulet: Item,
    pub flasks: Vec<Item>,
    pub jewels: Vec<Item>,
    pub gems: Vec<Item>,
}

pub struct Build {
    id: Uuid,
    itemset: String,
    league: String,
    required_items: Json<BuildItems>,
    found_items: Json<BuildItems>,
}

impl Build {
    pub fn new(
        id: &str,
        itemset: &str,
        league: &str,
        required_items: BuildItems,
        found_items: BuildItems,
    ) -> Result<Self> {
        Ok(Build {
            id: Uuid::from_str(id)?,
            itemset: itemset.to_string(),
            league: league.to_string(),
            required_items: Json(required_items),
            found_items: Json(found_items),
        })
    }
}

#[derive(Clone)]
pub struct BuildRepository {
    pool: PgPool,
}

impl BuildRepository {
    pub fn new(pool: PgPool) -> BuildRepository {
        BuildRepository { pool }
    }

    pub async fn begin(&self) -> Result<PgTransaction<'_>> {
        Ok(PgTransaction {
            transaction: self.pool.begin().await?,
        })
    }

    pub async fn new_build(&self, tr: &mut PgTransaction<'_>, build: Build) -> Result<()> {
        let _ = sqlx::query!(
            r#"
INSERT INTO builds (id, itemset, league, required, found)
VALUES ($1, $2, $3, $4, $5)
            "#,
            build.id,
            build.itemset,
            build.league,
            build.required_items as _,
            build.found_items as _,
        )
        .execute(&mut *tr.transaction)
        .await?;
        Ok(())
    }

    pub async fn get_build(&self, id: &str) -> Result<Build> {
        let ret = sqlx::query_as!(
            Build,
            r#"
SELECT id, itemset, league, 
    required as "required_items: Json<BuildItems>",
    found as "found_items: Json<BuildItems>"
FROM builds
WHERE id = $1
            "#,
            id as &str,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(ret)
    }
}
