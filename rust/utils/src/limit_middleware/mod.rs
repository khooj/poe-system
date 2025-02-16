mod limits;

use core::convert::TryFrom;
use http::Extensions;
use limits::MultipleLimits;
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next, Result};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LimitHeadersError {
    #[error("unknown variant")]
    Variant,
}

#[derive(PartialEq, Eq, Hash)]
pub enum LimitHeaders {
    Ip,
    Account,
    Client,
}

impl TryFrom<&str> for LimitHeaders {
    type Error = LimitHeadersError;

    fn try_from(value: &str) -> core::result::Result<Self, Self::Error> {
        Ok(match value {
            "x-rate-limit-account-state" => LimitHeaders::Account,
            "x-rate-limit-ip-state" => LimitHeaders::Ip,
            "x-rate-limit-client-state" => LimitHeaders::Client,
            _ => return Err(LimitHeadersError::Variant),
        })
    }
}

#[derive(Default)]
pub struct Limits {
    lm: dashmap::DashMap<LimitHeaders, limits::MultipleLimits>,
}

impl Limits {
    // pub fn new(init_limits: HashMap<&str, &str>) -> Limits {
    //     let lm = init_limits
    //         .into_iter()
    //         .filter_map(|(k, v)| {
    //             k.try_into()
    //                 .ok()
    //                 .map(|k| (k, MultipleLimits::new_with_limit(v, ",")))
    //         })
    //         .collect();
    //     Limits { lm }
    // }
}

#[async_trait::async_trait]
impl Middleware for Limits {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        for limiter in &self.lm {
            limiter.value().until_ready().await;
        }

        let resp = next.run(req, extensions).await?;

        if self.lm.is_empty() {
            for (k, v) in resp.headers().iter() {
                if let Ok(lh) = k.as_str().try_into() {
                    self.lm.insert(
                        lh,
                        MultipleLimits::new_with_limit(v.to_str().unwrap_or(""), ","),
                    );
                }
            }
        }

        Ok(resp)
    }
}
