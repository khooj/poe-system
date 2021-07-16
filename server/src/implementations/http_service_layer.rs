use super::{builds_repository::BuildsRepositoryError, pob::pob::Pob};
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
    pub items: Vec<(String, String)>,
}

pub struct HttpServiceLayer {
    pub item_repo: ItemsRepository,
    pub build_repo: BuildsRepository,
}

impl HttpServiceLayer {
    pub async fn get_build_matches(&self, build_id: &str) -> Result<BuildMatches, ServiceError> {
        let build = self.build_repo.get_build(build_id)?;
        let pob_file = self.build_repo.get_pob_file(&build.id)?;
        let pob = Pob::from_pastebin_data(pob_file.encoded_pob)?;
        let pob_doc = pob.as_document()?;

        let itemset = pob_doc.get_itemset(&build.itemset)?;

        let ids = self.build_repo.get_items_id_for_build(build_id)?;
        let items = self.item_repo.get_items_by_ids(ids)?;
        Ok(BuildMatches {
            items: items
                .into_iter()
                .enumerate()
                .map(|(idx, el)| {
                    let itemset_item = match itemset.get_nth_item(idx) {
                        Some(k) => format!("{:?}", k),
                        None => String::new(),
                    };
                    (itemset_item, format!("{:?}", el))
                })
                .collect(),
        })
    }
}
