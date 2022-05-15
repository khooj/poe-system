use crate::application::build_calculating::BuildCalculating;
use actix_web::{
    get, post,
    web::{self, Data, Json},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use tracing::{error, trace};
use validator::{Validate, ValidationError};

fn not_empty(s: &str) -> Result<(), ValidationError> {
    if s.is_empty() {
        return Err(ValidationError::new("empty string"));
    }
    Ok(())
}

#[derive(Deserialize, Validate)]
struct NewBuild {
    #[validate(custom = "not_empty")]
    pobdata: String,
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
    if let Err(e) = new.validate() {
        trace!(err = %e, "error validating input");
        return HttpResponse::BadRequest().finish();
    }

    let id = build_srv
        .add_build_for_calculating(&new.pobdata, &new.itemset, "Sentinel")
        .await;

    let id = match id {
        Ok(k) => k,
        Err(e) => {
            error!("error adding build for calculation: {}", e);
            return HttpResponse::BadRequest().json(ErrorData {
                msg: e.to_string(),
            });
        }
    };

    let resp = NewBuildId { id };
    HttpResponse::Ok().json(resp)
}

#[derive(Serialize)]
struct NoBuildYet {}

#[get("/build/{id}")]
pub async fn get_build(build_srv: Data<BuildCalculating>, id: web::Path<String>) -> HttpResponse {
    let id = id.into_inner();

    match build_srv.get_calculated_build(&id).await {
        Ok(k) => match k {
            Some(d) => HttpResponse::Ok().json(d),
            None => HttpResponse::Ok().json(NoBuildYet{})
        }
        Err(e) => HttpResponse::BadRequest().json(ErrorData {
            msg: e.to_string()
        })
    }
}
