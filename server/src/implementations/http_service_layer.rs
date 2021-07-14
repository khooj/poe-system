use super::builds_repository::BuildsRepositoryError;
use super::{BuildsRepository, ItemsRepository};
use crate::ports::outbound::repository::RepositoryError;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("anyhow")]
    Anyhow(#[from] anyhow::Error),
    #[error("repo")]
    ItemRepo(#[from] RepositoryError),
    #[error("repo")]
    BuildRepo(#[from] BuildsRepositoryError),
}

impl jsonrpc_v2::ErrorLike for ServiceError {
    fn code(&self) -> i64 {
        match self {
            ServiceError::Anyhow(_) => 1,
            _ => 2,
        }
    }

    fn message(&self) -> String {
        match self {
            ServiceError::Anyhow(e) => format!("{}", e),
            ServiceError::ItemRepo(e) => format!("{}", e),
            ServiceError::BuildRepo(e) => format!("{}", e),
        }
    }
}

#[derive(Serialize)]
pub struct BuildMatches {
    pub items: Vec<String>,
}

pub struct HttpServiceLayer {
    pub item_repo: ItemsRepository,
    pub build_repo: BuildsRepository,
}

impl HttpServiceLayer {
    pub async fn get_build_matches(&self, build_id: &str) -> Result<BuildMatches, ServiceError> {
        let ids = self.build_repo.get_items_id_for_build(build_id)?;
        let items = self.item_repo.get_items_by_ids(ids)?;
        Ok(BuildMatches {
            items: items.into_iter().map(|el| format!("{:?}", el)).collect(),
        })
    }
}
