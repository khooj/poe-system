use std::num::NonZeroU32;
use std::str::FromStr;
use std::time::Duration;

use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Jitter, Quota, RateLimiter};
use thiserror::Error;

#[derive(Debug, PartialEq, Default, Hash, Eq)]
struct Limit {
    pub hit_count: u32,
    pub watching_time: Duration,
    pub penalty_time: Duration,
}

impl Limit {
    fn new(current_hit: u32, watching_time: Duration, penalty_time: Duration) -> Limit {
        Limit {
            hit_count: current_hit,
            watching_time,
            penalty_time,
        }
    }

    fn parse_header(limit: &str) -> Limit {
        let lms: Vec<&str> = limit.split(':').collect();

        if lms.len() != 3 {
            panic!("unknown limit format");
        }

        let h = u32::from_str(lms[0]).expect("cannot parse header value");
        let w = u64::from_str(lms[1]).expect("cannot parse header value");
        let p = u64::from_str(lms[2]).expect("cannot parse header value");

        Limit::new(h, Duration::from_secs(w), Duration::from_secs(p))
    }
}

#[derive(Error, Debug)]
pub enum MultipleLimitsError {
    #[error("current limits order does not correspond to initially provided")]
    LimitMatch,
    #[error("limits states len does not equal to limits")]
    LimitsLen,
}

#[derive(Debug, Default)]
pub struct MultipleLimits {
    limits: dashmap::DashMap<Limit, RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl MultipleLimits {
    pub fn new_with_limit(limits: &str, sep: &str) -> MultipleLimits {
        let split_limits = limits.split(sep).collect::<Vec<_>>();
        // todo: vec resize optimization
        let mut limits = MultipleLimits::default();
        split_limits.into_iter().for_each(|lm| limits.add_limit(lm));
        limits
    }

    fn add_limit(&self, limits: &str) {
        let new_limits = Limit::parse_header(limits);
        let hit = NonZeroU32::try_from(new_limits.hit_count)
            .expect("cannot add limit with zero hit count, probably you provided current limit");
        // let quota = Quota::new(hit, new_limits.watching_time).unwrap();
        let quota = Quota::with_period(new_limits.watching_time)
            .unwrap()
            .allow_burst(hit);

        let lim = RateLimiter::direct(quota);

        self.limits.insert(new_limits, lim);
    }

    pub fn is_empty(&self) -> bool {
        self.limits.is_empty()
    }

    // pub fn reinit_limits(&self, limits: &str, sep: &str) {
    //     let split_limits = limits.split(sep).collect::<Vec<_>>();
    //     // todo: vec resize optimization
    //     let mut limits = MultipleLimits::default();
    //     split_limits.into_iter().for_each(|lm| limits.add_limit(lm));
    //     self.limits.clear();
    //     self.limits.extend(limits.limits);
    // }

    // pub async fn adjust_current_states(
    //     &self,
    //     states: MultipleLimits,
    // ) -> Result<(), MultipleLimitsError> {
    //     if states.limits.len() != self.limits.len() {
    //         return Err(MultipleLimitsError::LimitsLen);
    //     }
    //
    //     for lm in self.limits.iter() {
    //         let state_limit = states.limits.get(lm.key()).unwrap();
    //         if lm.key().watching_time != state_limit.key().watching_time {
    //             return Err(MultipleLimitsError::LimitMatch);
    //         }
    //     }
    //
    //     // for (i, lm) in &mut self.limits.iter_mut().enumerate() {
    //     //     let state_limit = states.limits.get(i).unwrap();
    //     //     let _ =
    //     //         lm.1.until_ready_with_jitter(Jitter::new(
    //     //             Duration::from_secs(1),
    //     //             state_limit.0.penalty_time,
    //     //         ))
    //     //         .await;
    //     // }
    //
    //     Ok(())
    // }

    pub async fn until_ready(&self) {
        for lm in &self.limits {
            lm.value().until_ready().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_limits() {
        let lm = MultipleLimits::new_with_limit("5:5:60;20:180:180", ";");
        assert_eq!(lm.limits.len(), 2);
    }
}
