use std::collections::HashMap;
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

pub struct HttpServiceLayer {}

impl HttpServiceLayer {
    pub async fn get_maps_list(&self) -> Result<MapsTiers, ServiceError> {
        todo!()
    }
}
