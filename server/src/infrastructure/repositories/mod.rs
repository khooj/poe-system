pub mod postgres;

use crate::configuration::Database;
use sqlx::postgres::PgPool;

pub type Repositories = (
    postgres::raw_item_repository::RawItemRepository,
    postgres::task_repository::TaskRepository,
    postgres::build_repository::BuildRepository,
);

pub async fn create_repositories(db: &Database) -> anyhow::Result<Repositories> {
    match db {
        Database::Postgres { ref dsn } => {
            let pool = PgPool::connect(dsn).await?;
            let raw_items =
                postgres::raw_item_repository::RawItemRepository::new(pool.clone()).await;
            let tasks = postgres::task_repository::TaskRepository::new(pool.clone());
            let builds = postgres::build_repository::BuildRepository::new(pool.clone());
            Ok((raw_items, tasks, builds))
        }
        _ => {
            panic!("database not supported");
        }
    }
}
