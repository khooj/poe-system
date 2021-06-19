use actix::prelude::*;
use actix_web::{get, web, HttpResponse, Responder};
use jsonrpc_v2::{Data, Error, Params, Server};
use serde::Deserialize;

use crate::{
    actors::build_calculator::{BuildCalculatorActor, StartBuildCalculatingMsg},
    application::poe_data,
};

#[derive(Deserialize)]
pub struct CalculatePob {
    pub pob_url: String,
    pub itemset: Option<String>,
}

#[derive(Deserialize)]
pub struct BuildPrice {
    build_id: String,
}

pub async fn calculate_pob(
    Params(params): Params<CalculatePob>,
    actor: Data<Addr<BuildCalculatorActor>>,
) -> Result<String, Error> {
    match actor.try_send(StartBuildCalculatingMsg {
        itemset: params.itemset,
        pob_url: params.pob_url,
    }) {
        Ok(k) => Ok("".into()),
        Err(e) => Err(Error::Provided {
            code: 1,
            message: "cant start actor: {}",
        }),
    }
}

pub async fn get_build_price(Params(params): Params<BuildPrice>) -> Result<String, Error> {
    Ok("".into())
}
