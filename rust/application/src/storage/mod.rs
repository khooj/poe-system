pub mod postgresql;
pub mod redis;

use domain::{build_calculation::typed_item::TypedItem, item::types::Mod};
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
pub trait ItemRepositoryTrait {
    async fn get_stash_id(&mut self) -> Result<LatestStashId, ItemRepositoryError>;
    async fn set_stash_id(&mut self, next: LatestStashId) -> Result<(), ItemRepositoryError>;
    async fn clear_stash(&mut self, stash_id: &str) -> Result<Vec<String>, ItemRepositoryError>;
    async fn insert_items(
        &mut self,
        items: Vec<TypedItem>,
        stash_id: &str,
    ) -> Result<(), ItemRepositoryError>;
    async fn search_items_by_mods(
        &mut self,
        mods: Vec<Mod>,
    ) -> Result<Vec<TypedItem>, ItemRepositoryError>;
}

pub type DynItemRepository = Box<dyn ItemRepositoryTrait>;
