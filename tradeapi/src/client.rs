use crate::limits::{Limits, MultipleLimits};
use crate::models::{ClientFetchResponse, ClientSearchResponse};
use crate::query::Builder;
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
    search_limiter: MultipleLimits,
    fetch_limiter: MultipleLimits,
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

        Client {
            client,
            search_limiter: MultipleLimits::default(),
            fetch_limiter: MultipleLimits::default(),
            league: league.to_string(),
            failed_check: None,
        }
    }

    async fn make_limiter_request<T>(
        failed_check: &mut Option<String>,
        client: &mut reqwest::Client,
        limiter: &mut MultipleLimits,
        req: Request,
        limit_policy: &str,
    ) -> Result<T, ClientError>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        if failed_check.is_some() {
            return Err(ClientError::FailedCheck(failed_check.clone().unwrap()));
        }
        limiter.until_ready().await;

        let resp = client.execute(req).await?;

        if let Some(l) = resp.headers().get("x-rate-limit-policy") {
            if l != limit_policy {
                let s = format!("unknown rate limit policy, doing nothing until you check tradeapi: expected {} got {}", limit_policy, l.to_str().unwrap());
                *failed_check = Some(s.clone());
                return Err(ClientError::FailedCheck(s));
            }
        }

        let acc_limits = resp
            .headers()
            .get("x-rate-limit-account")
            .unwrap()
            .to_str()
            .unwrap();
        let acc_limiting_state = resp
            .headers()
            .get("x-rate-limit-account-state")
            .unwrap()
            .to_str()
            .unwrap();

        let ip_limits = resp
            .headers()
            .get("x-rate-limit-ip")
            .unwrap()
            .to_str()
            .unwrap();
        let ip_limits_state = resp
            .headers()
            .get("x-rate-limit-ip-state")
            .unwrap()
            .to_str()
            .unwrap();

        let mut limits = MultipleLimits::parse_header(ip_limits, ",");
        limits.add_parse_header(acc_limits);
        let mut current_limits = ip_limits_state
            .split(',')
            .map(Limits::parse_header)
            .collect::<Vec<_>>();
        current_limits.push(Limits::parse_header(acc_limiting_state));
        debug!("current limits state: {:?}", current_limits);
        // todo: need to explicitly wait for penalty time
        if let Err(e) = limits
            .adjust_current_states_or_wait_for_penalty(current_limits)
            .await
        {
            *failed_check = Some(e.to_string());
            return Err(ClientError::FailedCheck(e.to_string()));
        }

        *limiter = limits;

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

    pub async fn get_search_id(
        &mut self,
        query: &Builder,
    ) -> Result<ClientSearchResponse, ClientError> {
        let mut req = self.client.post(format!(
            "https://www.pathofexile.com/api/trade/search/{}",
            self.league
        ));
        req = req.json(&query);

        let req = req.build()?;
        Self::make_limiter_request(
            &mut self.failed_check,
            &mut self.client,
            &mut self.search_limiter,
            req,
            "trade-search-request-limit",
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
            .client
            .request(
                Method::GET,
                format!("https://www.pathofexile.com/api/trade/fetch/{}", v),
            )
            .query(&[("query", req_id)]);

        let req = req.build()?;

        Self::make_limiter_request(
            &mut self.failed_check,
            &mut self.client,
            &mut self.fetch_limiter,
            req,
            "trade-fetch-request-limit",
        )
        .await
    }
}
