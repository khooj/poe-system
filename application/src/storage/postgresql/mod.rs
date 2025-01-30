use crate::typed_item::TypedItem;

use super::{DynItemRepository, ItemRepositoryError, ItemRepositoryTrait, LatestStashId};
use domain::Mod;
use serde::Serialize;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions, Row};
use thiserror::Error;

pub static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Error, Debug)]
pub enum PostgresRepositoryError {
    #[error("connect error")]
    Connect(#[from] sqlx::Error),
    #[error("serde json error")]
    Serde(#[from] serde_json::Error),
}

pub struct ItemRepository {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl ItemRepository {
    pub async fn new(dsn: &str) -> Result<DynItemRepository, PostgresRepositoryError> {
        let pool = PgPoolOptions::new().max_connections(5).connect(dsn).await?;
        Ok(Box::new(ItemRepository { pool }))
    }
}

#[derive(Debug, Serialize)]
struct SearchMods {
    mods: Vec<ModObj>,
}

#[derive(Debug, Serialize)]
struct ModObj {
    stat_id: String,
}

impl From<Vec<Mod>> for SearchMods {
    fn from(value: Vec<Mod>) -> Self {
        let mods = value
            .into_iter()
            .map(|m| ModObj { stat_id: m.stat_id })
            .collect();
        SearchMods { mods }
    }
}

struct WrapperTypedItem(TypedItem);

impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for WrapperTypedItem {
    fn from_row(row: &'_ sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self(TypedItem {
            id: row.try_get("id")?,
            info: row
                .try_get::<'_, sqlx::types::Json<_>, &str>("data")
                .map(|x| x.0)?,
        }))
    }
}

#[async_trait::async_trait]
impl ItemRepositoryTrait for ItemRepository {
    async fn get_stash_id(&mut self) -> Result<LatestStashId, ItemRepositoryError> {
        let id: Option<(String,)> = sqlx::query_as("select id from latest_stash")
            .fetch_optional(&self.pool)
            .await?;
        if id.is_none() {
            Ok(LatestStashId { id: None })
        } else {
            Ok(LatestStashId {
                id: Some(id.unwrap().0),
            })
        }
    }

    async fn insert_items(
        &mut self,
        items: Vec<crate::typed_item::TypedItem>,
        stash_id: &str,
    ) -> Result<(), ItemRepositoryError> {
        let mut tx = self.pool.begin().await?;
        let mut copyin = tx
            .copy_in_raw(r#"COPY items FROM STDIN WITH (FORMAT CSV, DELIMITER ';', QUOTE E'@')"#)
            .await?;
        let mut ids = Vec::with_capacity(items.len());
        let items_len = items.len();
        for item in items {
            let json = serde_json::to_string(&item.info).unwrap();
            let line = format!("{};@{}@\n", item.id, json);
            copyin.send(line.as_bytes()).await?;
            ids.push(item.id);
        }

        let len = copyin.finish().await?;
        if (len as usize) != items_len {
            eprintln!("inserted less items: {}", len);
        }

        for id in ids {
            sqlx::query("insert into stashes(id, item_id) values($1, $2)")
                .bind(&id)
                .bind(stash_id)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    async fn clear_stash(&mut self, stash_id: &str) -> Result<Vec<String>, ItemRepositoryError> {
        let mut tx = self.pool.begin().await?;
        let ids: Vec<(String,)> = sqlx::query_as("select item_id from stashes where id = $1")
            .bind(stash_id)
            .fetch_all(&mut *tx)
            .await?;
        sqlx::query("delete from items where id in (select item_id from stashes where id = $1)")
            .bind(stash_id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("delete from stashes where id = $1")
            .bind(stash_id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(ids.into_iter().map(|s| s.0).collect())
    }

    async fn set_stash_id(&mut self, next: LatestStashId) -> Result<(), ItemRepositoryError> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("truncate table latest_stash")
            .execute(&mut *tx)
            .await?;
        sqlx::query("insert into latest_stash(id) values ($1)")
            .bind(&next.id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn search_items_by_mods(
        &mut self,
        mods: Vec<Mod>,
    ) -> Result<Vec<TypedItem>, ItemRepositoryError> {
        let mut tx = self.pool.begin().await?;
        let search_mods: SearchMods = mods.into();
        let search_mods = serde_json::to_string(&search_mods)?;

        let result: Vec<WrapperTypedItem> =
            sqlx::query_as("select id, data from items where data @> $1")
                .bind(&search_mods)
                .fetch_all(&mut *tx)
                .await?;
        tx.commit().await?;
        Ok(result.into_iter().map(|s| s.0).collect())
    }
}
