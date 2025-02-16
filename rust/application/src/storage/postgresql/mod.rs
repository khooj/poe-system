use sqlx::migrate::Migrator;

pub mod items;
pub mod builds;

pub static MIGRATOR: Migrator = sqlx::migrate!();
