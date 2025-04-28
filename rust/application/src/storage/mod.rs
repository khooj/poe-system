pub mod postgresql;

use domain::{
    build_calculation::{required_item::Mod as RequiredMod, stored_item::StoredItem},
    item::types::{Category, Subcategory},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ItemRepositoryError {
    #[error("sqlx error")]
    Sqlx(#[from] sqlx::Error),
    #[error("serde json error")]
    SerdeJson(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct LatestStashId {
    pub id: Option<String>,
}

#[async_trait::async_trait]
pub trait StashRepositoryTrait {
    async fn get_stash_id(&mut self) -> Result<LatestStashId, ItemRepositoryError>;
    async fn set_stash_id(&mut self, next: LatestStashId) -> Result<(), ItemRepositoryError>;
    async fn clear_stash(&mut self, stash_id: &str) -> Result<Vec<String>, ItemRepositoryError>;
}

#[async_trait::async_trait]
pub trait ItemInsertTrait {
    async fn insert_items(
        &mut self,
        items: Vec<StoredItem>,
        stash_id: &str,
    ) -> Result<(), ItemRepositoryError>;
}

pub trait ItemRepositoryTrait: ItemInsertTrait {}

pub type DynItemRepository = Box<dyn ItemRepositoryTrait>;
