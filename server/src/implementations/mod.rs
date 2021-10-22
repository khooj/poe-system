pub mod http_controller;
pub mod models;
pub mod pob;
pub mod public_stash_retriever;
pub mod public_stash_timer;
pub mod wrapped_connection_pool;
pub mod http_service_layer;
mod rmdb;
mod mongo;

use rmdb::builds_repository::DieselBuildsRepository;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use rmdb::item_repository::DieselItemRepository;
use wrapped_connection_pool::WrappedConnectionPool;

pub type Conn = WrappedConnectionPool<SqliteConnection>;
pub type TypedConnectionPool = Pool<ConnectionManager<Conn>>;
pub type BuildsRepository = DieselBuildsRepository;
pub type ItemsRepository = mongo::items_repository::ItemsRepository;
