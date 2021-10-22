use actix::prelude::*;
use jsonrpc_v2::{Data, Error, Params};
use serde::Deserialize;

use super::http_service_layer::{BuildMatches, HttpServiceLayer};

#[derive(Deserialize)]
pub struct CalculatePob {
    pub pob_url: String,
    pub itemset: Option<String>,
}

#[derive(Deserialize)]
pub struct BuildPrice {
    pub build_id: String,
}

#[derive(Deserialize)]
pub struct BuildInfo {
    pub id: String,
}

// pub async fn get_build_matched_items(
//     Params(params): Params<BuildInfo>,
//     svc: Data<HttpServiceLayer>,
// ) -> Result<BuildMatches, Error> {
//     Ok(svc.get_build_matches(&params.id).await?)
// }

pub async fn get_build_price(Params(_): Params<BuildPrice>) -> Result<String, Error> {
    Ok("".into())
}
