use thiserror::Error;

#[derive(Error, Debug)]
pub enum ItemRepositoryError {}

#[derive(Debug)]
pub struct LatestStashId {
    pub id: String,
}

#[derive(Debug)]
pub struct Item {}

#[async_trait::async_trait]
pub trait ItemRepository {
    async fn get_stash_id(&mut self) -> Result<LatestStashId, ItemRepositoryError>;
    async fn set_stash_id(&mut self, next: LatestStashId) -> Result<(), ItemRepositoryError>;
    async fn delete_item(&mut self, acc: &str, stash: &str) -> Result<(), ItemRepositoryError>;
    async fn insert_items(&mut self, items: Vec<Item>) -> Result<(), ItemRepositoryError>;
}

pub type DynItemRepository = Box<dyn ItemRepository>;
