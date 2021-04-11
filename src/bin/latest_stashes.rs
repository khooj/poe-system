use anyhow::anyhow;
use governor::{
    clock::{DefaultClock, QuantaInstant},
    state::{direct::NotKeyed, InMemoryState},
    Quota, RateLimiter,
};
use serde::Deserialize;
use serde_json::{to_writer, Value};
use std::str::FromStr;
use std::{
    convert::TryFrom,
    time::{Duration, Instant},
};
use std::{
    env::args,
    fs::{File, OpenOptions},
};
use std::{
    io::{BufWriter, Write},
    num::NonZeroU32,
};
use thiserror::Error;
use tokio::time::Instant as TokioInstant;

#[derive(Debug, Error)]
enum MyError {
    #[error("limited for {0} seconds")]
    RateLimited(u32),
    #[error("client error {0}")]
    ClientError(#[from] reqwest::Error),
    #[error("next cycle")]
    NextCycle,
}

#[derive(Deserialize)]
struct PublishStash {
    next_change_id: String,
    stashes: Vec<Value>,
}

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

struct Client {
    client: reqwest::Client,
    limiter: Option<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
    latest_limiter: Option<String>,
}

async fn wait_for(d: u64) {
    let until = Instant::now() + Duration::from_millis(d);
    println!("waiting for 500ms");
    tokio::time::sleep_until(TokioInstant::from_std(until)).await;
}

impl Client {
    fn new() -> Client {
        let client_builder = reqwest::ClientBuilder::new();
        let client = client_builder
            .user_agent("OAuth latest-stashes/0.1.0 (contact: bladoff@gmail.com)")
            .build()
            .unwrap();
        Client {
            client,
            limiter: None,
            latest_limiter: None,
        }
    }

    async fn get_stashes(&mut self, id: Option<&str>) -> Result<PublishStash, MyError> {
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
            println!("limits header found");
            limiting = match l.to_str() {
                Ok(l) => l,
                Err(e) => {
                    println!("cant to_str header {}, using default", e);
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
        println!("current limits state: {:?}", parse_header(limiting_state));

        let st = resp.error_for_status_ref();
        match st {
            Ok(e) => e,
            Err(e) => {
                if e.status() == Some(reqwest::StatusCode::TOO_MANY_REQUESTS) {
                    let lims = parse_header(limiting);
                    wait_for(lims.penalty_time as u64).await;
                }
                return Err(MyError::NextCycle);
            }
        };

        let body = resp.json::<PublishStash>().await?;

        Ok(body)
    }

    fn reinit_limiter(&mut self, limiting: &str) {
        let limits = parse_header(&limiting);
        println!("encountered new limits: {:?}", limits);
        let hit =
            NonZeroU32::try_from(limits.hit_count).map_or(NonZeroU32::new(1u32).unwrap(), |v| v);
        let quota = Quota::with_period(Duration::from_secs(limits.watching_time as u64))
            .unwrap()
            .allow_burst(hit);

        self.limiter = Some(RateLimiter::direct(quota));
        self.latest_limiter = Some(limiting.to_owned());
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "wrong argument size",
        ));
    }

    let mut stashes_info = Vec::with_capacity(110_000);
    let mut client = Client::new();
    let mut id: Option<String> = None;
    let f = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&args[1])?;
    let mut buf = BufWriter::new(f);

    loop {
        let mut resp = match client.get_stashes(id.as_deref()).await {
            Ok(r) => r,
            Err(e) => match e {
                MyError::NextCycle => continue,
                _ => panic!("{}", e),
            },
        };

        if resp.stashes.len() == 0 {
            break;
        }

        stashes_info.append(&mut resp.stashes);
        id = Some(resp.next_change_id);
        println!("now stashes info len: {}", stashes_info.len());

        if stashes_info.len() >= 100_000 {
            println!("writing {} entries", stashes_info.len());
            serde_json::to_writer(&mut buf, &stashes_info)?;
            stashes_info.clear();
        }
    }

    println!("flushing");
    serde_json::to_writer(&mut buf, &stashes_info)?;
    buf.flush()
}
