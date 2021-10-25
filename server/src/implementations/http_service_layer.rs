use super::pob::pob::Pob;
use super::ItemsRepository;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("anyhow")]
    Anyhow(#[from] anyhow::Error),
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
        }
    }
}

#[derive(Serialize)]
pub struct BuildMatches {
    pub items: Vec<(String, String)>,
}

pub struct HttpServiceLayer {
    pub item_repo: ItemsRepository,
}

impl HttpServiceLayer {}
