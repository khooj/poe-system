use gloo::net::http::{Error, Request, Response};
use serde::Deserialize;

async fn make_get_request(url: &str) -> Result<Response, Error> {
    Ok(Request::get(url).send().await?)
}

#[derive(Deserialize)]
pub struct BuildInfo {}

pub async fn get_build(id: &str) -> Result<BuildInfo, Error> {
    let resp = make_get_request(url).await?;
    Ok(resp.json().await?)
}

async fn make_post_request<T>(url: &str, body: T) -> Result<Response, Error>
where
    T: serde::Serialize,
{
    Ok(Request::post(url).json(body)?.send().await?)
}
