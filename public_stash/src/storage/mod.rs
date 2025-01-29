pub mod postgresql;

use crate::typed_item::TypedItem;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ItemRepositoryError {
    #[error("sqlx error")]
    Sqlx(#[from] sqlx::Error)
}

#[derive(Debug)]
pub struct LatestStashId {
    pub id: Option<String>,
}

#[async_trait::async_trait]
pub trait ItemRepositoryTrait {
    async fn get_stash_id(&mut self) -> Result<LatestStashId, ItemRepositoryError>;
    async fn set_stash_id(&mut self, next: LatestStashId) -> Result<(), ItemRepositoryError>;
    async fn clear_stash(&mut self, stash_id: &str) -> Result<(), ItemRepositoryError>;
    async fn insert_items(&mut self, items: Vec<TypedItem>, stash_id: &str) -> Result<(), ItemRepositoryError>;
}

pub type DynItemRepository = Box<dyn ItemRepositoryTrait>;
