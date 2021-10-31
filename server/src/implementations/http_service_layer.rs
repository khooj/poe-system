use std::collections::HashMap;

use super::pob::pob::Pob;
use super::ItemsRepository;
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize)]
pub struct MapOrder {
    pub name: String,
    pub count: i32,
    pub tier: i32,
}

#[derive(Deserialize)]
pub struct MapsOrder {
    pub should_fulfil: bool,
    pub maps: Vec<MapOrder>,
}

#[derive(Serialize)]
pub struct VendorMapInfo {
    pub account_name: String,
    pub can_fulfil: bool,
    pub maps: Vec<MapOrderInfo>,
}

#[derive(Serialize)]
pub struct MapOrderInfo {
    pub name: String,
    pub stash: String,
    pub count: i32,
    pub tier: i32,
    pub note: String,
}

#[derive(Serialize)]
pub struct MapsOrderResult {
    pub vendor: Vec<VendorMapInfo>,
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

    pub async fn get_vendors_for_maps(
        &self,
        maps: MapsOrder,
    ) -> Result<MapsOrderResult, ServiceError> {
        let result = self
            .item_repo
            .get_maps_data_by_account(maps.maps.iter().map(|e| e.into()).collect())
            .await?;
    }
}
