use super::PgTransaction;
use crate::{
    infrastructure::repositories::LatestStashId, interfaces::public_stash_retriever::Item,
};
use anyhow::Result;
use sqlx::{types::Json, PgPool};

pub struct RawItem {
    id: String,
    account_name: String,
    stash: String,
    item: Json<Item>,
}

#[derive(Clone)]
pub struct RawItemRepository {
    pool: PgPool,
}

impl RawItemRepository {
    pub async fn new(pool: PgPool) -> RawItemRepository {
        RawItemRepository { pool }
    }

    pub async fn begin(&self) -> Result<PgTransaction<'_>> {
        let transaction = self.pool.begin().await?;
        Ok(PgTransaction { transaction })
    }

    pub async fn get_stash_id(&self, transaction: &mut PgTransaction<'_>) -> Result<LatestStashId> {
        let id = sqlx::query_as!(
            LatestStashId,
            "SELECT stash_id as latest_stash_id FROM latest_stash LIMIT 1",
        )
        .fetch_one(&mut *transaction.transaction)
        .await?;
        Ok(id)
    }

    pub async fn insert_raw_item(
        &self,
        transaction: &mut PgTransaction<'_>,
        id: &str,
        acc: &str,
        stash: &str,
        item: Item,
    ) -> Result<()> {
        let _ = sqlx::query!(
            r#"
INSERT INTO raw_items (id, account_name, stash, item) 
VALUES ($1, $2, $3, $4)
            "#,
            id,
            acc,
            stash,
            Json(item) as _,
        )
        .execute(&mut *transaction.transaction)
        .await?;
        Ok(())
    }

    pub async fn delete_raw_item(
        &self,
        transaction: &mut PgTransaction<'_>,
        acc: &str,
        stash: &str,
    ) -> Result<()> {
        let _ = sqlx::query!(
            r#"
DELETE FROM raw_items WHERE account_name = $1 AND stash = $2 
            "#,
            acc,
            stash,
        )
        .execute(&mut *transaction.transaction)
        .await?;
        Ok(())
    }

    pub async fn set_stash_id(&self, transaction: &mut PgTransaction<'_>, id: &str) -> Result<()> {
        let r = sqlx::query!("SELECT stash_id FROM latest_stash LIMIT 1")
            .fetch_optional(&mut *transaction.transaction)
            .await?;

        if r.is_some() {
            let _ = sqlx::query!(
                "UPDATE latest_stash SET stash_id = $1 WHERE stash_id = $2",
                id,
                r.unwrap().stash_id
            )
            .execute(&mut *transaction.transaction)
            .await?;
            return Ok(());
        } else {
            let _ = sqlx::query!(
                r#"
INSERT INTO latest_stash (stash_id) 
VALUES ($1) 
ON CONFLICT (stash_id) DO UPDATE SET stash_id = $1"#,
                id,
            )
            .execute(&mut *transaction.transaction)
            .await?;
            return Ok(());
        }
    }

    pub async fn get_raw_items_cursor(
        &self,
        base_types: &[&str],
    ) -> futures_core::stream::BoxStream<'_, Result<RawItem, sqlx::Error>> {
        sqlx::query_as!(
            RawItem,
            r#"
SELECT id, account_name, stash, item as "item: Json<Item>" 
FROM raw_items
WHERE item['baseType'] ?| $1
            "#,
            base_types as _,
        )
        .fetch(&self.pool)
    }
}
