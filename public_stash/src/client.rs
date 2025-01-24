use super::models::PublicStashData;
use thiserror::Error;
use tracing::error;
use utils::{reqwest::StatusCode, ClientBuilder, ClientWithMiddleware, LimitMiddleware};

#[derive(Debug, Error)]
pub enum Error {
    #[error("client error {0}")]
    ClientError(#[from] utils::reqwest::Error),
    #[error("reqwest_middleware error: {0}")]
    ReqwestMiddleware(#[from] utils::ReqwestMiddlewareError),
    #[error("io error {0}")]
    IoError(#[from] std::io::Error),
    #[error("next cycle")]
    NextCycle,
    #[error("status code")]
    StatusCode(u16),
    #[error("too many requests")]
    TooManyRequests,
}

pub struct Client {
    client: ClientWithMiddleware,
}

impl Client {
    pub fn new(user_agent: String) -> Client {
        let client = utils::reqwest::ClientBuilder::new()
            .user_agent(user_agent)
            .build()
            .expect("can't build http client");

        let client = ClientBuilder::new(client)
            .with(LimitMiddleware::default())
            .build();

        Client { client }
    }

    pub async fn get_latest_stash(&mut self, id: Option<&str>) -> Result<PublicStashData, Error> {
        let mut req = self
            .client
            .get("https://api.pathofexile.com/public-stash-tabs");

        if let Some(id) = id {
            req = req.query(&[("id", id)]);
        }

        let req = req.build()?;

        let resp = self.client.execute(req).await?;

        let st = resp.status();
        match st {
            StatusCode::TOO_MANY_REQUESTS => {
                return Err(Error::TooManyRequests);
            }
            x if x.is_success() => {}
            x => return Err(Error::StatusCode(x.as_u16())),
        };

        let body = resp.json::<PublicStashData>().await?;

        Ok(body)
    }
}
