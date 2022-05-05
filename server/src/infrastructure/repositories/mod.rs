pub mod mongo;
pub mod postgres;

use crate::configuration::Database;
use crate::interfaces::public_stash_retriever::Item;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[derive(Deserialize, Serialize)]
pub struct LatestStashId {
    pub latest_stash_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DbItem {
    #[serde(flatten)]
    pub item: Item,
    pub account_name: Option<String>,
    pub stash: Option<String>,
}

pub type Repositories = (
    postgres::raw_item_repository::RawItemRepository,
    postgres::task_repository::TaskRepository,
    postgres::build_repository::BuildRepository,
);

pub async fn create_repositories(db: &Database) -> anyhow::Result<Repositories> {
    match db {
        Database::Postgres { ref dsn } => {
            let pool = PgPool::connect(dsn).await?;
            let raw_items = postgres::raw_item_repository::RawItemRepository::new(pool.clone()).await;
            let tasks = postgres::task_repository::TaskRepository::new(pool.clone());
            let builds = postgres::build_repository::BuildRepository::new(pool.clone());
            Ok((raw_items, tasks, builds))
        }
        _ => {
            panic!("database not supported");
        }
    }
}
