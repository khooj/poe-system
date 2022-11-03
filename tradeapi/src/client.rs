use super::models::{ItemsData, StaticData, StatsData};
use governor::{
    clock::DefaultClock,
    state::{direct::NotKeyed, InMemoryState},
    Jitter, Quota, RateLimiter,
};
use reqwest::{StatusCode, Request};
use std::num::NonZeroU32;
use std::str::FromStr;
use std::{convert::TryFrom, time::Duration};
use thiserror::Error;
use tracing::{debug, error};

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("try on next cycle")]
    NextCycle,
}

#[derive(Debug, PartialEq)]
struct Limits {
    hit_count: u32,
    watching_time: Duration,
    penalty_time: Duration,
}

impl Limits {
    fn new(h: u32, w: Duration, p: Duration) -> Limits {
        Limits {
            hit_count: h,
            watching_time: w,
            penalty_time: p,
        }
    }
}

fn parse_header(limit: &str) -> Limits {
    let lms: Vec<&str> = limit.split(":").collect();

    if lms.len() != 3 {
        return Limits::new(0, Duration::from_secs(0u64), Duration::from_secs(0u64));
    }

    let h = u32::from_str(lms[0]).unwrap();
    let w = u64::from_str(lms[1]).unwrap();
    let p = u64::from_str(lms[2]).unwrap();

    Limits::new(h, Duration::from_secs(w), Duration::from_secs(p))
}

struct Limiter {
    limiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock>,
    latest_limits: Limits,
}

pub struct Client {
    client: reqwest::Client,
    limiter_search: Option<Limiter>,
    limiter_fetch: Option<Limiter>,
}

impl Client {
    pub fn new(user_agent: String) -> Client {
        let client = reqwest::ClientBuilder::new()
            .user_agent(&user_agent)
            .build()
            .expect("can't build http client");
        Client {
            client,
            limiter_search: None,
            limiter_fetch: None,
        }
    }

    fn reinit_limiter(limits: Limits) -> Limiter {
        debug!("encountered new limits: {:?}", limits);
        let hit =
            NonZeroU32::try_from(limits.hit_count).map_or(NonZeroU32::new(1u32).unwrap(), |v| v);
        let quota = Quota::with_period(limits.watching_time)
            .unwrap()
            .allow_burst(hit);

        let lim = RateLimiter::direct(quota);

        Limiter {
            limiter: lim,
            latest_limits: limits,
        }
    }

    async fn process_request<T>(&mut self, req: Request) -> Result<(T, Limiter), ClientError> {

    }

    pub async fn get_latest_stash(&mut self, id: Option<&str>) -> Result<PublicStashData, ClientError> {
        if let Some(rl) = &self.limiter {
            rl.until_ready().await;
        }

        let mut req = self
            .client
            .get("https://api.pathofexile.com/public-stash-tabs");

        if let Some(id) = id {
            req = req.query(&[("id", id)]);
        }

        let req = req.build()?;

        let resp = self.client.execute(req).await?;

        let mut limiting = "1:1:60";

        if let Some(l) = resp.headers().get("X-Rate-Limit-Client") {
            debug!("limits header found");
            limiting = match l.to_str() {
                Ok(k) => k,
                Err(e) => {
                    error!("tostrerror, luck next time {}", e);
                    return Err(Error::NextCycle);
                }
            };
        }

        let limits = parse_header(&limiting);
        let do_reinit_limiter =
            self.latest_limits.is_some() && &limits != self.latest_limits.as_ref().unwrap();
        let do_reinit_limiter = do_reinit_limiter || self.limiter.is_none();
        if do_reinit_limiter {
            self.reinit_limiter(limits);
        }

        let limiting_state = resp
            .headers()
            .get("X-Rate-Limit-Client-State")
            .map(|e| e.to_str().unwrap_or("1:1:0"))
            .unwrap_or("1:1:0");
        debug!("current limits state: {:?}", parse_header(limiting_state));

        let st = resp.status();
        match st {
            StatusCode::TOO_MANY_REQUESTS => {
                let rl = self.limiter.as_ref().unwrap();
                let limits = self.latest_limits.as_ref().unwrap();
                rl.until_ready_with_jitter(Jitter::new(
                    limits.penalty_time,
                    Duration::from_secs(1),
                ))
                .await;
                return Err(Error::NextCycle);
            }
            x if x.is_success() => {}
            x => return Err(Error::StatusCode(x.as_u16())),
        };

        let body = resp.json::<PublicStashData>().await?;

        Ok(body)
    }
}
