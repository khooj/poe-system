use std::num::NonZeroU32;
use std::str::FromStr;
use std::time::Duration;

use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Jitter, Quota, RateLimiter};

#[derive(Debug, PartialEq, Default)]
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
        let lms: Vec<&str> = limit.split(':').collect();

        if lms.len() != 3 {
            return Limits::new(0, Duration::from_secs(0u64), Duration::from_secs(0u64));
        }

        let h = u32::from_str(lms[0]).unwrap();
        let w = u64::from_str(lms[1]).unwrap();
        let p = u64::from_str(lms[2]).unwrap();

        Limits::new(h, Duration::from_secs(w), Duration::from_secs(p))
    }
}

#[derive(Debug, Default)]
pub struct MultipleLimits {
    limits: Vec<(Limits, RateLimiter<NotKeyed, InMemoryState, DefaultClock>)>,
}

impl MultipleLimits {
    pub fn parse_header(limits: &str, sep: &str) -> MultipleLimits {
        let split_limits = limits.split(sep).collect::<Vec<_>>();
        // todo: vec resize optimization
        let mut limits = MultipleLimits::default();
        split_limits
            .into_iter()
            .for_each(|lm| limits.add_parse_header(lm));
        limits
    }

    pub fn add_parse_header(&mut self, limits: &str) {
        let new_limits = Limits::parse_header(limits);
        let hit = NonZeroU32::try_from(new_limits.hit_count)
            .map_or(NonZeroU32::new(1u32).unwrap(), |v| v);
        let quota = Quota::new(hit, new_limits.watching_time).unwrap();

        let lim = RateLimiter::direct(quota);

        self.limits.push((new_limits, lim));
    }

    pub async fn adjust_current_states_or_wait_for_penalty(
        &mut self,
        states: Vec<Limits>,
    ) -> Result<(), anyhow::Error> {
        if states.len() != self.limits.len() {
            return Err(anyhow::anyhow!(
                "limits states len does not equal to limits"
            ));
        }
        for (i, lm) in &mut self.limits.iter_mut().enumerate() {
            let state_limit = states.get(i).unwrap();
            if lm.0.watching_time != state_limit.watching_time {
                return Err(anyhow::anyhow!(
                    "Current limits order does not correspond to initially provided"
                ));
            }
        }

        for (i, lm) in &mut self.limits.iter_mut().enumerate() {
            let state_limit = states.get(i).unwrap();
            let _ =
                lm.1.until_n_ready_with_jitter(
                    NonZeroU32::new(state_limit.hit_count).unwrap_or(NonZeroU32::new(1).unwrap()),
                    Jitter::new(state_limit.penalty_time, Duration::from_secs(1)),
                )
                .await;
        }

        Ok(())
    }

    pub async fn until_ready(&mut self) {
        for (_, lim) in &mut self.limits {
            lim.until_ready().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_quota() {
        let q = Quota::new(NonZeroU32::new(5).unwrap(), Duration::from_secs(1)).unwrap();

        assert_eq!(q.burst_size_replenished_in(), Duration::from_secs(1));
    }
}
