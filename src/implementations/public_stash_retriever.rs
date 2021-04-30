use crate::ports::outbound::public_stash_retriever::{Error, PublicStashData, Retriever};
use async_trait::async_trait;
use governor::{
    clock::DefaultClock,
    state::{direct::NotKeyed, InMemoryState},
    Quota, RateLimiter,
};
use log::info;
use std::num::NonZeroU32;
use std::str::FromStr;
use std::{
    convert::TryFrom,
    time::{Duration, Instant},
};
use tokio::time::Instant as TokioInstant;

#[derive(Debug)]
struct Limits {
    hit_count: u32,
    watching_time: u32,
    penalty_time: u32,
}

impl Limits {
    fn new(h: u32, w: u32, p: u32) -> Limits {
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
        return Limits::new(0, 0, 0);
    }

    let h = u32::from_str(lms[0]).unwrap();
    let w = u32::from_str(lms[1]).unwrap();
    let p = u32::from_str(lms[2]).unwrap();

    Limits::new(h, w, p)
}

pub struct Client {
    client: reqwest::Client,
    limiter: Option<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
    latest_limiter: Option<String>,
}

async fn wait_for(d: u64) {
    let until = Instant::now() + Duration::from_millis(d);
    info!("waiting for {}", d);
    tokio::time::sleep_until(TokioInstant::from_std(until)).await;
}

impl Client {
    pub fn new(user_agent: String) -> Client {
        let client_builder = reqwest::ClientBuilder::new();
        let client = client_builder.user_agent(user_agent).build().unwrap();
        Client {
            client,
            limiter: None,
            latest_limiter: None,
        }
    }
    fn reinit_limiter(&mut self, limiting: &str) {
        let limits = parse_header(&limiting);
        info!("encountered new limits: {:?}", limits);
        let hit =
            NonZeroU32::try_from(limits.hit_count).map_or(NonZeroU32::new(1u32).unwrap(), |v| v);
        let quota = Quota::with_period(Duration::from_secs(limits.watching_time as u64))
            .unwrap()
            .allow_burst(hit);

        self.limiter = Some(RateLimiter::direct(quota));
        self.latest_limiter = Some(limiting.to_owned());
    }
}

#[async_trait]
impl Retriever for Client {
    async fn get_latest_stash(&mut self, id: Option<&str>) -> Result<PublicStashData, Error> {
        while let Some(rl) = &self.limiter {
            let result = rl.check();

            if let Err(_) = result {
                wait_for(500).await;
            } else {
                break;
            }
        }

        let mut req = self
            .client
            .get("https://api.pathofexile.com/public-stash-tabs");

        if let Some(id) = id {
            req = req.query(&[("id", id)]);
        }

        let resp = req.send().await?;

        let mut limiting = "1:1:60";

        if let Some(l) = resp.headers().get("X-Rate-Limit-Client") {
            info!("limits header found");
            limiting = match l.to_str() {
                Ok(l) => l,
                Err(e) => {
                    info!("cant to_str header {}, using default", e);
                    "1:1:60"
                }
            };
        }

        if self.latest_limiter.is_some() && limiting != self.latest_limiter.as_ref().unwrap() {
            self.reinit_limiter(limiting);
        }

        if self.limiter.is_none() {
            self.reinit_limiter(limiting);
        }

        let limiting_state = resp
            .headers()
            .get("X-Rate-Limit-Client-State")
            .map_or("1:1:0", |v| v.to_str().map_or("1:1:0", |x| x));
        info!("current limits state: {:?}", parse_header(limiting_state));

        let st = resp.error_for_status_ref();
        match st {
            Ok(e) => e,
            Err(e) => {
                if e.status() == Some(reqwest::StatusCode::TOO_MANY_REQUESTS) {
                    let lims = parse_header(limiting);
                    wait_for(lims.penalty_time as u64).await;
                }
                return Err(Error::NextCycle);
            }
        };

        let body = resp.json::<PublicStashData>().await?;

        Ok(body)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ports::outbound::public_stash_retriever::Retriever;
    use std::env::set_var;

    // #[tokio::test]
    async fn get_single() -> Result<(), anyhow::Error> {
        let mut ret = Client::new("OAuth poe-system/0.0.1 (contact: bladoff@gmail.com)".to_owned());
        let result = ret.get_latest_stash(None).await?;
        assert_ne!(result.next_change_id, "");
        Ok(())
    }
}
