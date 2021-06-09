use actix_web::{get, web, HttpResponse, Responder};
use jsonrpc_v2::{Data, Error, Params, Server};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CalculatePob {
    pub pob_url: String,
    pub itemset: String,
}

#[derive(Deserialize)]
pub struct BuildPrice {
    build_id: String,
}

pub async fn calculate_pob(Params(params): Params<CalculatePob>) -> Result<String, Error> {
    Ok("".into())
}

pub async fn get_build_price(Params(params): Params<BuildPrice>) -> Result<String, Error> {
    Ok("".into())
}
