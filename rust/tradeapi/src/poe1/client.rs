use std::time::{Duration, SystemTime};

use crate::poe1::models::{ClientFetchResponse, ClientSearchResponse};
use crate::poe1::query::Builder;
use thiserror::Error;
use tracing::error;
use utils::{
    reqwest::{cookie::Jar, Method, Request, StatusCode, Url},
    ClientBuilder, ClientWithMiddleware, LimitMiddleware,
};

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("too many requests from endpoint")]
    TooManyRequestsOrSimilar,
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] utils::reqwest::Error),
    #[error("reqwest_middleware error: {0}")]
    ReqwestMiddleware(#[from] utils::ReqwestMiddlewareError),
    #[error("failed check: {0}")]
    FailedCheck(String),
    #[error("status code: {0}")]
    StatusCode(u16),
    #[error("incorrect args")]
    IncorrectArgs,
}

pub struct Client {
    search_client: ClientWithMiddleware,
    fetch_client: ClientWithMiddleware,
    league: String,
    failed_check: Option<String>,
    wait_time_on_error: Duration,
    last_wait: SystemTime,
}

impl Client {
    pub fn new(user_agent: &str, poesessid: &str, league: &str) -> Client {
        let client = Client::new_client(user_agent, poesessid);
        let search_client = ClientBuilder::new(client)
            .with(LimitMiddleware::default())
            .build();

        let client = Client::new_client(user_agent, poesessid);
        let fetch_client = ClientBuilder::new(client)
            .with(LimitMiddleware::default())
            .build();

        Client {
            search_client,
            fetch_client,
            league: league.to_string(),
            failed_check: None,
            wait_time_on_error: Duration::from_secs(5),
            last_wait: SystemTime::now(),
        }
    }

    fn new_client(user_agent: &str, poesessid: &str) -> utils::reqwest::Client {
        let jar = Jar::default();
        jar.add_cookie_str(
            &format!("POESESSID={}", poesessid),
            &"https://www.pathofexile.com".parse::<Url>().unwrap(),
        );

        utils::reqwest::ClientBuilder::new()
            .user_agent(user_agent)
            .cookie_store(true)
            .cookie_provider(jar.into())
            .build()
            .expect("can't build http client")
    }

    pub fn set_wait_time(&mut self, d: Duration) {
        self.wait_time_on_error = d;
    }

    async fn make_limiter_request<T>(
        failed_check: &mut Option<String>,
        client: &mut ClientWithMiddleware,
        req: Request,
        limit_policy: &str,
        last_wait: &mut SystemTime,
        wait_time_on_error: &Duration,
    ) -> Result<T, ClientError>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        if failed_check.is_some() {
            return Err(ClientError::FailedCheck(failed_check.clone().unwrap()));
        }

        if last_wait.elapsed().is_err() {
            return Err(ClientError::TooManyRequestsOrSimilar);
        }

        let resp = client.execute(req).await?;

        if let Some(l) = resp.headers().get("x-rate-limit-policy") {
            if l != limit_policy {
                let s = format!("unknown rate limit policy, doing nothing until you check tradeapi: expected {} got {}", limit_policy, l.to_str().unwrap());
                *failed_check = Some(s.clone());
                return Err(ClientError::FailedCheck(s));
            }
        }

        let st = resp.status();
        match st {
            StatusCode::TOO_MANY_REQUESTS => {
                *last_wait = SystemTime::now()
                    .checked_add(*wait_time_on_error)
                    .expect("cannot set timeout for next call");
                return Err(ClientError::TooManyRequestsOrSimilar);
            }
            x if x.is_success() => {}
            x => return Err(ClientError::StatusCode(x.as_u16())),
        };

        let body = resp.json::<T>().await?;

        Ok(body)
    }

    pub async fn get_search_id(
        &mut self,
        query: &Builder,
    ) -> Result<ClientSearchResponse, ClientError> {
        let mut req = self.search_client.post(format!(
            "https://www.pathofexile.com/api/trade/search/{}",
            self.league
        ));
        req = req.json(&query);

        let req = req.build()?;
        Self::make_limiter_request(
            &mut self.failed_check,
            &mut self.search_client,
            req,
            "trade-search-request-limit",
            &mut self.last_wait,
            &self.wait_time_on_error,
        )
        .await
    }

    pub async fn fetch_results(
        &mut self,
        ids: Vec<String>,
        req_id: &str,
    ) -> Result<ClientFetchResponse, ClientError> {
        if ids.is_empty() {
            return Ok(ClientFetchResponse { result: vec![] });
        }
        if ids.len() > 10 {
            return Err(ClientError::IncorrectArgs);
        }

        let v = ids
            .iter()
            .fold(String::new(), |acc, el| acc + el + ",")
            .strip_suffix(',')
            .unwrap()
            .to_string();
        let req = self
            .fetch_client
            .request(
                Method::GET,
                format!("https://www.pathofexile.com/api/trade/fetch/{}", v),
            )
            .query(&[("query", req_id)]);

        let req = req.build()?;

        Self::make_limiter_request(
            &mut self.failed_check,
            &mut self.fetch_client,
            req,
            "trade-fetch-request-limit",
            &mut self.last_wait,
            &self.wait_time_on_error,
        )
        .await
    }
}
