pub mod build_repository;
pub mod task_repository;

use anyhow::Result;
use sqlx::{Postgres, Transaction};
use std::ops::Deref;
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
