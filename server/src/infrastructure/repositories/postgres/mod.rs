pub mod raw_item_repository;
pub mod raw_item;
pub mod task_repository;
pub mod build_repository;

use sqlx::{Postgres, Transaction};
use std::ops::Deref;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct LatestStashId {
    pub latest_stash_id: String,
}

pub struct PgTransaction<'a> {
    transaction: Transaction<'a, Postgres>,
}

impl<'a> Deref for PgTransaction<'a> {
    type Target = Transaction<'a, Postgres>;

    fn deref(&self) -> &Self::Target {
        &self.transaction
    }
}

impl<'a> PgTransaction<'a> {
    pub async fn commit(self) -> Result<()> {
        Ok(self.transaction.commit().await?)
    }
}
