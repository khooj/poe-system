pub mod builds_repository;
pub mod http_controller;
pub mod item_repository;
pub mod models;
pub mod pob;
pub mod public_stash_retriever;
pub mod public_stash_timer;
pub mod wrapped_connection_pool;

use builds_repository::DieselBuildsRepository;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use item_repository::DieselItemRepository;
use wrapped_connection_pool::WrappedConnectionPool;

pub type Conn = WrappedConnectionPool<SqliteConnection>;
pub type TypedConnectionPool = Pool<ConnectionManager<Conn>>;
pub type BuildsRepository = DieselBuildsRepository;
pub type ItemsRepository = DieselItemRepository;
