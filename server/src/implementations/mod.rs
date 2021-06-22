pub mod builds_repository;
pub mod http_controller;
pub mod item_repository;
pub mod models;
pub mod pob;
pub mod public_stash_retriever;
pub mod public_stash_timer;
pub mod connection_pool;

use builds_repository::DieselBuildsRepository;
use diesel::{r2d2::ConnectionManager, SqliteConnection as Conn};
use item_repository::DieselItemRepository;
use connection_pool::ConnectionPool;

pub type SqliteConnection = ConnectionManager<Conn>;
pub type TypedConnectionPool = ConnectionPool<SqliteConnection>;
pub type BuildsRepository = DieselBuildsRepository<TypedConnectionPool>;
pub type ItemsRepository = DieselItemRepository<TypedConnectionPool>;
