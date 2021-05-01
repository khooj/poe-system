use super::public_stash_retriever::PublicStashData;
use thiserror::Error;
use diesel::Queryable;

#[derive(Queryable, Debug)]
pub struct LatestStashId {
    pub latest_stash_id: Option<String>,
}

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("orm error")]
    OrmError(#[from] diesel::result::ConnectionError),
    #[error("query error")]
    QueryError(#[from] diesel::result::Error),
    #[error("not found")]
    NotFound,
    #[error("t")]
    Ttt,
}

pub trait ItemRepository {
    fn insert_raw_item(&self, public_data: PublicStashData) -> Result<(), RepositoryError>;
    fn get_stash_id(&self) -> Result<LatestStashId, RepositoryError>;
}
