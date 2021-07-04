use super::public_stash_retriever::PublicStashData;
use diesel::Queryable;
use thiserror::Error;

#[derive(Queryable, Debug)]
pub struct LatestStashId {
    pub latest_stash_id: Option<String>,
}

impl Default for LatestStashId {
    fn default() -> Self {
        LatestStashId {
            latest_stash_id: None,
        }
    }
}

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("orm error")]
    OrmError(#[from] diesel::result::ConnectionError),
    #[error("query error")]
    QueryError(#[from] diesel::result::Error),
    #[error("pool error")]
    PoolError(#[from] r2d2::Error),
    #[error("not found")]
    NotFound,
    #[error("t")]
    Ttt,
    #[error("skipped item")]
    Skipped,
    #[error("database error")]
    Db,
}

pub trait ItemRepository {
    fn insert_raw_item(&self, public_data: PublicStashData) -> Result<(), RepositoryError>;
    fn get_stash_id(&self) -> Result<LatestStashId, RepositoryError>;
}
