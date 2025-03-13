use crate::storage::{ItemRepositoryError, LatestStashId};
use domain::{
    build_calculation::typed_item::TypedItem,
    types::{Category, Mod, Subcategory},
};
use serde::Serialize;
use sqlx::Row;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PostgresRepositoryError {
    #[error("connect error")]
    Connect(#[from] sqlx::Error),
    #[error("serde json error")]
    Serde(#[from] serde_json::Error),
}

#[derive(Clone)]
pub struct ItemRepository {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl ItemRepository {
    pub async fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Result<Self, PostgresRepositoryError> {
        Ok(ItemRepository { pool })
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
            basetype: row.try_get("basetype")?,
            category: row
                .try_get::<'_, &str, &str>("category")?
                .parse()
                .map_err(|e| sqlx::Error::ColumnDecode {
                    index: "category".to_string(),
                    source: Box::new(e),
                })?,
            subcategory: row
                .try_get::<'_, &str, &str>("subcategory")?
                .parse()
                .map_err(|e| sqlx::Error::ColumnDecode {
                    index: "subcategory".to_string(),
                    source: Box::new(e),
                })?,
            info: row
                .try_get::<'_, sqlx::types::Json<_>, &str>("data")
                .map(|x| x.0)?,
            name: row.try_get("name")?,
        }))
    }
}

impl ItemRepository {
    pub async fn get_stash_id(&mut self) -> Result<LatestStashId, ItemRepositoryError> {
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

    pub async fn insert_items(
        &mut self,
        items: Vec<TypedItem>,
        stash_id: &str,
    ) -> Result<(), ItemRepositoryError> {
        let mut tx = self.pool.begin().await?;
        let mut copyin = tx
            .copy_in_raw(r#"COPY items(id, data, basetype, category, subcategory, name) FROM STDIN WITH (FORMAT CSV, DELIMITER ';', QUOTE E'@')"#)
            .await?;
        let mut ids = Vec::with_capacity(items.len());
        let items_len = items.len();
        for item in items {
            let json = serde_json::to_string(&item.info).unwrap();
            let line = format!(
                "{};@{}@;{};{};{};{}\n",
                item.id,
                json,
                item.basetype,
                item.category.as_ref(),
                item.subcategory.as_ref(),
                item.name,
            );
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

    pub async fn clear_stash(
        &mut self,
        stash_id: &str,
    ) -> Result<Vec<String>, ItemRepositoryError> {
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

    pub async fn set_stash_id(&mut self, next: LatestStashId) -> Result<(), ItemRepositoryError> {
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

    pub async fn search_items_by_attrs(
        &mut self,
        basetype: Option<&str>,
        category: Option<Category>,
        subcategory: Option<Subcategory>,
        mods: Option<Vec<Mod>>,
    ) -> Result<Vec<TypedItem>, ItemRepositoryError> {
        let mut tx = self.pool.begin().await?;

        let query = "select id, data, basetype, category, subcategory, name from items";
        let mut filters = vec![];
        let mut count = 0;
        if basetype.is_some() {
            count += 1;
            filters.push(format!("basetype = ${}", count));
        }
        if category.is_some() {
            count += 1;
            filters.push(format!("category = ${}", count));
        }
        if subcategory.is_some() {
            count += 1;
            filters.push(format!("subcategory = ${}", count));
        }
        if mods.is_some() {
            count += 1;
            filters.push(format!("data @> ${}::jsonb", count));
        }

        let query = if filters.is_empty() {
            query.to_string()
        } else {
            query.to_string() + " where " + &filters.join(" and ")
        };

        let mut sqx_query = sqlx::query_as(&query);

        if let Some(b) = basetype {
            sqx_query = sqx_query.bind(b);
        }
        if let Some(b) = category {
            sqx_query = sqx_query.bind(b.as_ref().to_string());
        }
        if let Some(b) = subcategory {
            sqx_query = sqx_query.bind(b.as_ref().to_string());
        }
        if let Some(b) = mods {
            let search_mods: SearchMods = b.into();
            let search_mods = serde_json::to_string(&search_mods)?;
            sqx_query = sqx_query.bind(search_mods);
        }

        let result: Vec<WrapperTypedItem> = sqx_query.fetch_all(&mut *tx).await?;
        tx.commit().await?;
        Ok(result.into_iter().map(|s| s.0).collect())
    }
}
