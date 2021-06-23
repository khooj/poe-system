pub mod builds_repository;
pub mod http_controller;
pub mod item_repository;
pub mod models;
pub mod pob;
pub mod public_stash_retriever;
pub mod public_stash_timer;

use builds_repository::DieselBuildsRepository;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection as Conn,
};
use item_repository::DieselItemRepository;

pub type SqliteConnection = ConnectionManager<Conn>;
pub type TypedConnectionPool = Pool<SqliteConnection>;
pub type BuildsRepository = DieselBuildsRepository;
pub type ItemsRepository = DieselItemRepository;
