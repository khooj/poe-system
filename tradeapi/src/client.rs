use crate::limits::Limits;
use crate::models::{ClientFetchResponse, ClientSearchResponse};
use crate::query::Builder;
use governor::{
    clock::DefaultClock,
    state::{direct::NotKeyed, InMemoryState},
    Jitter, Quota, RateLimiter,
};
use reqwest::cookie::Jar;
use reqwest::{Method, Request, RequestBuilder, StatusCode, Url};
use std::num::NonZeroU32;
use std::{convert::TryFrom, time::Duration};
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

struct Limit(Limits, RateLimiter<NotKeyed, InMemoryState, DefaultClock>);

pub struct Client {
    client: reqwest::Client,
    search_limiter: Option<Limit>,
    fetch_limiter: Option<Limit>,
    league: String,
    failed_check: Option<String>,
}

impl Client {
    pub fn new(user_agent: String, poesessid: &str, league: &str) -> Client {
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
            search_limiter: None,
            fetch_limiter: None,
            league: league.to_string(),
            failed_check: None,
        }
    }

    fn reinit_limiter(limiter: &mut Option<Limit>, limits: Limits) {
        debug!("encountered new limits: {:?}", limits);
        let hit =
            NonZeroU32::try_from(limits.hit_count).map_or(NonZeroU32::new(1u32).unwrap(), |v| v);
        let quota = Quota::with_period(limits.watching_time)
            .unwrap()
            .allow_burst(hit);

        let lim = RateLimiter::direct(quota);

        let l = Limit(limits, lim);
        *limiter = Some(l);
    }

    pub async fn get_search_id(
        &mut self,
        query: Builder,
    ) -> Result<ClientSearchResponse, ClientError> {
        if self.failed_check.is_some() {
            return Err(ClientError::FailedCheck(self.failed_check.clone().unwrap()));
        }
        if let Some(Limit(_, rl)) = &self.search_limiter {
            rl.until_ready().await;
        }

        let mut req = self.client.post(format!(
            "https://www.pathofexile.com/api/trade/search/{}",
            self.league
        ));
        req = req.json(&query);

        let req = req.build()?;

        let resp = self.client.execute(req).await?;

        let mut limiting = "1:3:60";

        if let Some(l) = resp.headers().get("x-rate-limit-account") {
            debug!("limits header found");
            limiting = match l.to_str() {
                Ok(k) => k,
                Err(e) => {
                    error!("tostrerror, luck next time {}", e);
                    return Err(ClientError::NextCycle);
                }
            };
        }

        if let Some(l) = resp.headers().get("x-rate-limit-policy") {
            if l != "trade-search-request-limit" {
                let s = "unknown rate limit policy, doing nothing until you check tradeapi";
                self.failed_check = Some(s.to_string());
                return Err(ClientError::FailedCheck(s.to_string()));
            }
        }

        let limits = Limits::parse_header(limiting);
        let do_reinit_limiter =
            self.search_limiter.is_some() && limits != self.search_limiter.as_ref().unwrap().0;
        let do_reinit_limiter = do_reinit_limiter || self.search_limiter.is_none();
        if do_reinit_limiter {
            Self::reinit_limiter(&mut self.search_limiter, limits);
        }

        let limiting_state = resp
            .headers()
            .get("x-rate-limit-account-state")
            .map(|e| e.to_str().unwrap_or("1:3:0"))
            .unwrap_or("1:3:0");
        debug!(
            "current limits state: {:?}",
            Limits::parse_header(limiting_state)
        );

        let st = resp.status();
        match st {
            StatusCode::TOO_MANY_REQUESTS => {
                let rl = &self.search_limiter.as_ref().unwrap().1;
                let limits = &self.search_limiter.as_ref().unwrap().0;
                rl.until_ready_with_jitter(Jitter::new(
                    limits.penalty_time,
                    Duration::from_secs(1),
                ))
                .await;
                return Err(ClientError::NextCycle);
            }
            x if x.is_success() => {}
            x => return Err(ClientError::StatusCode(x.as_u16())),
        };

        let body = resp.json::<ClientSearchResponse>().await?;

        Ok(body)
    }

    pub async fn fetch_results(
        &mut self,
        ids: Vec<String>,
        req_id: &str,
    ) -> Result<ClientFetchResponse, ClientError> {
        if self.failed_check.is_some() {
            return Err(ClientError::FailedCheck(self.failed_check.clone().unwrap()));
        }
        if let Some(Limit(_, rl)) = &self.fetch_limiter {
            rl.until_ready().await;
        }

        if ids.is_empty() {
            return Ok(ClientFetchResponse { result: vec![] });
        }
        if ids.len() > 5 {
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

        let resp = self.client.execute(req).await?;

        let mut limiting = "6:4:10";

        if let Some(l) = resp.headers().get("x-rate-limit-account") {
            debug!("limits header found");
            limiting = match l.to_str() {
                Ok(k) => k,
                Err(e) => {
                    error!("tostrerror, luck next time {}", e);
                    return Err(ClientError::NextCycle);
                }
            };
        }

        if let Some(l) = resp.headers().get("x-rate-limit-policy") {
            if l != "trade-fetch-request-limit" {
                let s =
                    "unknown rate limit policy for fetch, doing nothing until you check tradeapi";
                self.failed_check = Some(s.to_string());
                return Err(ClientError::FailedCheck(s.to_string()));
            }
        }

        let limits = Limits::parse_header(limiting);
        let do_reinit_limiter =
            self.fetch_limiter.is_some() && limits != self.fetch_limiter.as_ref().unwrap().0;
        let do_reinit_limiter = do_reinit_limiter || self.fetch_limiter.is_none();
        if do_reinit_limiter {
            Self::reinit_limiter(&mut self.fetch_limiter, limits);
        }

        let limiting_state = resp
            .headers()
            .get("x-rate-limit-account-state")
            .map(|e| e.to_str().unwrap_or("1:4:0"))
            .unwrap_or("1:4:0");
        debug!(
            "current limits state: {:?}",
            Limits::parse_header(limiting_state)
        );

        let st = resp.status();
        match st {
            StatusCode::TOO_MANY_REQUESTS => {
                let rl = &self.fetch_limiter.as_ref().unwrap().1;
                let limits = &self.fetch_limiter.as_ref().unwrap().0;
                rl.until_ready_with_jitter(Jitter::new(
                    limits.penalty_time,
                    Duration::from_secs(1),
                ))
                .await;
                return Err(ClientError::NextCycle);
            }
            x if x.is_success() => {}
            x => return Err(ClientError::StatusCode(x.as_u16())),
        };

        let body = resp.json::<ClientFetchResponse>().await?;

        Ok(body)
    }
}
