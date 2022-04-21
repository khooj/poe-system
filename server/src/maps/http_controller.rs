use actix::prelude::*;
use jsonrpc_v2::{Data, Error, Params};
use serde::Deserialize;

use super::http_service_layer::{MapsTiers, HttpServiceLayer};

pub async fn get_maps_list(data: Data<HttpServiceLayer>) -> Result<MapsTiers, Error> {
    Ok(data.get_maps_list().await?)
}
