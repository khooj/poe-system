use gloo::net::http::{Request, Response};
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

const API_ROOT: &str = dotenv_codegen::dotenv!("API_ROOT");

#[derive(Debug, Clone, ThisError, PartialEq)]
pub enum Error {
    #[error("request error: {0}")]
    RequestError(String),
}

async fn make_get_request(path: &str) -> Result<Response, Error> {
    Ok(Request::get(&format!("{}{}", API_ROOT, path))
        .send()
        .await
        .map_err(|e| Error::RequestError(e.to_string()))?)
}

async fn make_post_request<T>(path: &str, body: T) -> Result<Response, Error>
where
    T: Serialize,
{
    Ok(Request::post(&format!("{}{}", API_ROOT, path))
        .json(&body)
        .map_err(|e| Error::RequestError(e.to_string()))?
        .send()
        .await
        .map_err(|e| Error::RequestError(e.to_string()))?)
}

#[derive(Serialize)]
pub struct NewBuild {
    pub url: String,
    pub itemset: String,
}

#[derive(Deserialize)]
struct BuildId {
    id: String,
}

pub async fn post_new_build(build: NewBuild) -> Result<String, Error> {
    let resp = make_post_request("/new", build).await?;
    let resp: BuildId = resp
        .json()
        .await
        .map_err(|e| Error::RequestError(e.to_string()))?;
    Ok(resp.id)
}

#[derive(Deserialize)]
pub struct BuildInfo {}

pub async fn get_build(id: &str) -> Result<BuildInfo, Error> {
    let resp = make_get_request("/bui").await?;
    Ok(resp
        .json()
        .await
        .map_err(|e| Error::RequestError(e.to_string()))?)
}
