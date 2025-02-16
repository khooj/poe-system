use std::time::Duration;

use crate::build_calculation::BuildInfo;
use sqlx::prelude::FromRow;
use sqlx::types::{
    chrono::{DateTime, Utc},
    Uuid,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PostgresRepositoryError {
    #[error("connect error")]
    Connect(#[from] sqlx::Error),
    #[error("serde json error")]
    Serde(#[from] serde_json::Error),
}

#[derive(Debug, FromRow)]
pub struct BuildData {
    pub id: Uuid,
    #[sqlx(json)]
    pub data: BuildInfo,
    pub processed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl BuildData {
    pub fn new(id: Uuid, data: BuildInfo) -> Self {
        BuildData {
            id,
            data,
            processed: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Clone)]
pub struct BuildRepository {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl BuildRepository {
    pub async fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Result<Self, PostgresRepositoryError> {
        Ok(BuildRepository { pool })
    }

    pub async fn get_build<T: AsRef<str>>(
        &mut self,
        id: T,
    ) -> Result<BuildData, PostgresRepositoryError> {
        let data: BuildData = sqlx::query_as(
            "select id, data, processed, created_at, updated_at from builds where id = $1",
        )
        .bind(id.as_ref())
        .fetch_one(&self.pool)
        .await?;

        Ok(data)
    }

    pub async fn get_builds(&mut self) -> Result<Vec<BuildData>, PostgresRepositoryError> {
        let data: Vec<BuildData> =
            sqlx::query_as("select id, data, processed, created_at, updated_at from builds")
                .fetch_all(&self.pool)
                .await?;

        Ok(data)
    }

    pub async fn upsert_build(&mut self, data: &BuildData) -> Result<(), PostgresRepositoryError> {
        sqlx::query(
            r#"insert into builds(id, data, created_at, updated_at, processed) 
                values ($1, $2::jsonb, $3, $4, $5) 
                on conflict (id) do update 
                set data = EXCLUDED.data, updated_at = now(), processed = EXCLUDED.processed"#,
        )
        .bind(data.id)
        .bind(serde_json::to_string(&data.data)?)
        .bind(data.created_at)
        .bind(data.updated_at)
        .bind(data.processed)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_build_for_processing(
        &mut self,
    ) -> Result<Option<BuildData>, PostgresRepositoryError> {
        let mut tx = self.pool.begin().await?;
        let data: Option<(i32, Uuid)> = sqlx::query_as(
            r#"select id, build_id from new_builds where processing = FALSE and started_at is NULL 
            limit 1
            for update skip locked
            "#,
        )
        .fetch_optional(&mut *tx)
        .await?;
        if let Some((id, uid)) = data {
            sqlx::query(
                "update new_builds set processing = TRUE, started_at = now() where id = $1",
            )
            .bind(id)
            .execute(&mut *tx)
            .await?;

            let data: BuildData = sqlx::query_as(
                r#"select id, data, processed, created_at, updated_at from builds where id = $1"#,
            )
            .bind(uid)
            .fetch_one(&mut *tx)
            .await?;

            tx.commit().await?;
            return Ok(Some(data));
        }
        tx.commit().await?;
        Ok(None)
    }

    pub async fn unlock_failed_builds(
        &mut self,
        timeout: Duration,
    ) -> Result<(), PostgresRepositoryError> {
        sqlx::query(
            r#"update new_builds set processing = FALSE, started_at = NULL
            where processing = TRUE and started_at + $1 < now()"#,
        )
        .bind(timeout)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn unlock_build(&mut self, id: &Uuid) -> Result<(), PostgresRepositoryError> {
        sqlx::query(r#"delete from new_builds where processing = true and build_id = $1"#)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
