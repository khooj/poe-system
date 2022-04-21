use crate::ports::outbound::public_stash_retriever::{Error, PublicStashData};
use governor::{
    clock::DefaultClock,
    state::{direct::NotKeyed, InMemoryState},
    Quota, RateLimiter,
};
use std::num::NonZeroU32;
use std::str::FromStr;
use std::{
    convert::TryFrom,
    time::{Duration, Instant},
};
use tokio::time::Instant as TokioInstant;
use tracing::info;

#[derive(Debug)]
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

pub struct Client {
    client: ureq::Agent,
    limiter: Option<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
    latest_limiter: Option<String>,
}

async fn _wait_for(d: u64) {
    let until = Instant::now() + Duration::from_millis(d);
    info!("waiting for {}", d);
    tokio::time::sleep_until(TokioInstant::from_std(until)).await;
}

impl Client {
    pub fn new(user_agent: String) -> Client {
        let client = ureq::AgentBuilder::new().user_agent(&user_agent).build();
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
        let quota = Quota::with_period(limits.watching_time)
            .unwrap()
            .allow_burst(hit);

        self.limiter = Some(RateLimiter::direct(quota));
        self.latest_limiter = Some(limiting.to_owned());
    }

    pub fn get_latest_stash(&mut self, id: Option<&str>) -> Result<PublicStashData, Error> {
        while let Some(rl) = &self.limiter {
            let result = rl.check();

            if let Err(_) = result {
                std::thread::sleep(Duration::from_millis(100));
            } else {
                break;
            }
        }

        let mut req = self
            .client
            .get("https://api.pathofexile.com/public-stash-tabs");

        if let Some(id) = id {
            req = req.query("id", id);
        }

        let resp = req.call()?;

        let mut limiting = "1:1:60";

        if let Some(l) = resp.header("X-Rate-Limit-Client") {
            info!("limits header found");
            limiting = l;
        }

        if self.latest_limiter.is_some() && limiting != self.latest_limiter.as_ref().unwrap() {
            self.reinit_limiter(limiting);
        }

        if self.limiter.is_none() {
            self.reinit_limiter(limiting);
        }

        let limiting_state = resp.header("X-Rate-Limit-Client-State").unwrap_or("1:1:0");
        info!("current limits state: {:?}", parse_header(limiting_state));

        let st = resp.status();
        match st {
            429 => {
                let lims = parse_header(limiting);
                std::thread::sleep(lims.penalty_time);
                return Err(Error::NextCycle);
            }
            0..=299 => {}
            300..=u16::MAX => return Err(Error::StatusCode(st)),
        };

        let body = resp.into_json::<PublicStashData>()?;

        Ok(body)
    }
}
