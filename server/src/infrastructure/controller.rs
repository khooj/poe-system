use crate::application::build_calculating::BuildCalculating;
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Deserialize)]
struct NewBuild {
    url: String,
    league: String,
    itemset: String,
}

#[derive(Serialize)]
struct ErrorData {
    msg: String,
}

#[derive(Serialize)]
struct NewBuildId {
    id: String,
}

#[post("/new")]
pub async fn new_build(build_srv: Data<BuildCalculating>, new: Json<NewBuild>) -> HttpResponse {
    let id = build_srv
        .add_build_for_calculating(&new.url, &new.itemset, &new.league)
        .await;

    let id = match id {
        Ok(k) => k,
        Err(e) => {
            error!("error adding build for calculation: {}", e);
            return HttpResponse::BadRequest().json(ErrorData {
                msg: "error".to_string(),
            });
        }
    };

    let resp = NewBuildId { id };
    HttpResponse::Ok().json(resp)
}