use crate::build_calculation::BuildInfo;
use crate::storage::{ItemRepositoryError, ItemRepositoryTrait, LatestStashId};
use crate::typed_item::TypedItem;
use domain::Mod;
use serde::Serialize;
use sqlx::prelude::FromRow;
use sqlx::types::chrono;
use sqlx::types::{
    chrono::{DateTime, Utc},
    Json, Uuid,
};
use sqlx::{Encode, Row};
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

    pub async fn upsert_build(&mut self, data: BuildData) -> Result<(), PostgresRepositoryError> {
        sqlx::query(
            r#"insert into builds(id, data, created_at, updated_at, processed) 
                values ($1, $2, $3, $4, $5) 
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
        let data: Option<BuildData> = sqlx::query_as(
            r#"select id, data, processed, created_at, updated_at from builds
                            where processed = false
                            group by created_at
                            order by created_at asc
                            limit 1"#,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(data)
    }
}
