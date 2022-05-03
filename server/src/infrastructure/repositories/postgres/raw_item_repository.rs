use crate::{
    infrastructure::repositories::{DbItem, LatestStashId},
    interfaces::public_stash_retriever::PublicStashData,
};
use anyhow::{anyhow, Result};
use sqlx::postgres::PgPool;

pub struct RawItemRepository {
    pool: PgPool,
}

impl RawItemRepository {
    pub async fn new(pool: PgPool) -> RawItemRepository {
        RawItemRepository { pool }
    }

    pub async fn get_stash_id(&self) -> Result<LatestStashId> {
        todo!()
    }

    pub async fn insert_raw_item(&self, public_data: PublicStashData) -> Result<()> {
        todo!()
    }

    pub async fn set_stash_id(&self, id: &str) -> Result<()> {
        todo!()
    }
}
