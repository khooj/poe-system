pub mod postgres;

use crate::configuration::Database;
use sqlx::postgres::PgPoolOptions;

pub type Repositories = (
    postgres::task_repository::TaskRepository,
    postgres::build_repository::BuildRepository,
);

pub async fn create_repositories(db: &Database) -> anyhow::Result<Repositories> {
    match db {
        Database::Postgres { ref dsn } => {
            let pool = PgPoolOptions::new().connect(dsn).await?;
            let tasks = postgres::task_repository::TaskRepository::new(pool.clone());
            let builds = postgres::build_repository::BuildRepository::new(pool.clone());
            Ok((tasks, builds))
        }
        _ => {
            panic!("database not supported");
        }
    }
}
