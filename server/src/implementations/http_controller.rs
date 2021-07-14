use actix::prelude::*;
use jsonrpc_v2::{Data, Error, Params};
use serde::Deserialize;

use crate::actors::build_calculator::{BuildCalculatorActor, StartBuildCalculatingMsg};

#[derive(Deserialize)]
pub struct CalculatePob {
    pub pob_url: String,
    pub itemset: Option<String>,
}

#[derive(Deserialize)]
pub struct BuildPrice {
    pub build_id: String,
}

pub async fn calculate_pob(
    Params(params): Params<CalculatePob>,
    actor: Data<Addr<BuildCalculatorActor>>,
) -> Result<String, Error> {
    match actor
        .send(StartBuildCalculatingMsg {
            itemset: params.itemset,
            pob_url: params.pob_url,
        })
        .await
    {
        Ok(k) => match k {
            Ok(s) => Ok(s),
            Err(_) => Err(Error::Provided {
                code: 1,
                message: "cant get build id",
            }),
        },
        Err(_) => Err(Error::Provided {
            code: 1,
            message: "cant start actor",
        }),
    }
}

pub async fn get_build_price(Params(_): Params<BuildPrice>) -> Result<String, Error> {
    Ok("".into())
}
