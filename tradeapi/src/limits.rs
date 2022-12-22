use governor::{
    clock::DefaultClock,
    state::{direct::NotKeyed, InMemoryState},
    Jitter, Quota, RateLimiter,
};
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub struct Limits {
    pub hit_count: u32,
    pub watching_time: Duration,
    pub penalty_time: Duration,
}

impl Limits {
    pub fn new(h: u32, w: Duration, p: Duration) -> Limits {
        Limits {
            hit_count: h,
            watching_time: w,
            penalty_time: p,
        }
    }

    pub fn parse_header(limit: &str) -> Limits {
        let lms: Vec<&str> = limit.split(":").collect();

        if lms.len() != 3 {
            return Limits::new(0, Duration::from_secs(0u64), Duration::from_secs(0u64));
        }

        let h = u32::from_str(lms[0]).unwrap();
        let w = u64::from_str(lms[1]).unwrap();
        let p = u64::from_str(lms[2]).unwrap();

        Limits::new(h, Duration::from_secs(w), Duration::from_secs(p))
    }
}

struct Limiter {
    limiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock>,
    latest_limits: Limits,
}
