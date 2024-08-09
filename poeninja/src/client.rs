use crate::limits::{Limits, MultipleLimits};
use crate::models::Response;
use reqwest::cookie::Jar;
use reqwest::{Method, Request, StatusCode, Url};
use thiserror::Error;
use tracing::{debug, error};

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("try on next cycle")]
    NextCycle,
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("failed check: {0}")]
    FailedCheck(String),
    #[error("status code: {0}")]
    StatusCode(u16),
    #[error("incorrect args")]
    IncorrectArgs,
}

pub struct Client {
    client: reqwest::Client,
    limiter: MultipleLimits,
    league: String,
    failed_check: Option<String>,
}

impl Client {
    pub fn new(user_agent: &str, poesessid: &str, league: &str) -> Client {
        let jar = Jar::default();
        jar.add_cookie_str(
            &format!("POESESSID={}", poesessid),
            &"https://www.pathofexile.com".parse::<Url>().unwrap(),
        );

        let client = reqwest::ClientBuilder::new()
            .user_agent(user_agent)
            .cookie_store(true)
            .cookie_provider(jar.into())
            .build()
            .expect("can't build http client");

        let limiter = MultipleLimits::parse_header("0:4:60", ",");

        Client {
            client,
            limiter,
            league: league.to_string(),
            failed_check: None,
        }
    }

    async fn make_limiter_request<T>(
        failed_check: &mut Option<String>,
        client: &mut reqwest::Client,
        limiter: &mut MultipleLimits,
        req: Request,
    ) -> Result<T, ClientError>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        if failed_check.is_some() {
            return Err(ClientError::FailedCheck(failed_check.clone().unwrap()));
        }
        limiter.until_ready().await;

        let resp = client.execute(req).await?;

        let st = resp.status();
        match st {
            StatusCode::TOO_MANY_REQUESTS => {
                // should be already handled
                return Err(ClientError::NextCycle);
            }
            x if x.is_success() => {}
            x => return Err(ClientError::StatusCode(x.as_u16())),
        };

        let body = resp.json::<T>().await?;

        Ok(body)
    }

    pub async fn get_items(&mut self, typ: &str) -> Result<Response, ClientError> {
        let req = self.client.get(format!("https://poe.ninja/api/data/itemoverview?league={}&type={}", self.league, typ));
        let req = req.build()?;
        Self::make_limiter_request(
            &mut self.failed_check,
            &mut self.client,
            &mut self.limiter,
            req,
        ).await
    }
}
