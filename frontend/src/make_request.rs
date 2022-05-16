use gloo::net::http::{Request, Response};
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

const API_ROOT: &str = dotenv_codegen::dotenv!("API_ROOT");

#[derive(Debug, Clone, ThisError, PartialEq)]
pub enum Error {
    #[error("request error: {0}")]
    RequestError(String),
    #[error("custom error: {0}")]
    CustomError(String),
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

pub async fn get_text_body(url: &str) -> Result<Response, Error> {
    Ok(Request::get(url)
        .send()
        .await
        .map_err(|e| Error::RequestError(e.to_string()))?)
}

#[derive(Serialize)]
pub struct NewBuild {
    pub pob: String,
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

#[derive(Deserialize, PartialEq, Default, Clone)]
pub struct ItemInfo {
    pub name: String,
    pub base_type: String,
    pub image_link: String,
    pub mods: Vec<String>,
}

#[derive(Deserialize, PartialEq, Clone)]
pub struct BuildsetInfo {
    pub weapon1: ItemInfo,
    pub weapon2: ItemInfo,
    pub helmet: ItemInfo,
    pub body_armour: ItemInfo,
    pub belt: ItemInfo,
    pub amulet: ItemInfo,
    pub ring1: ItemInfo,
    pub ring2: ItemInfo,
    pub gloves: ItemInfo,
    pub boots: ItemInfo,
}

#[derive(Deserialize, PartialEq, Clone)]
pub struct BuildInfo {
    pub required_items: BuildsetInfo,
    pub found_items: BuildsetInfo,
}

#[derive(Deserialize)]
struct NoBuildInfo {
    #[serde(flatten)]
    data: Option<BuildInfo>,
}

pub async fn get_build(id: &str) -> Result<Option<BuildInfo>, Error> {
    let resp = make_get_request(&format!("/build/{}", id)).await?;
    let data: NoBuildInfo = resp
        .json()
        .await
        .map_err(|e| Error::RequestError(e.to_string()))?;
    Ok(data.data)
}
