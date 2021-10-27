use std::collections::HashMap;

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
pub struct MapInfo {
    pub tiers: Vec<i32>,
}

#[derive(Serialize)]
pub struct MapsTiers {
    pub maps: HashMap<String, MapInfo>,
}

pub struct HttpServiceLayer {
    pub item_repo: ItemsRepository,
}

impl HttpServiceLayer {
    pub async fn get_maps_list(&self) -> Result<MapsTiers, ServiceError> {
        let mut maps_hash = HashMap::new();
        let maps = self.item_repo.get_available_maps().await?;
        for map in maps {
            let tiers = self.item_repo.get_map_tiers(&map).await?;
            maps_hash.insert(map, MapInfo { tiers });
        }

        Ok(MapsTiers { maps: maps_hash })
    }
}
