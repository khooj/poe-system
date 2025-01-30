mod limit_middleware;
pub use limit_middleware::Limits as LimitMiddleware;
pub use reqwest;
pub use reqwest_middleware::{
    ClientBuilder, ClientWithMiddleware, Error as ReqwestMiddlewareError, Middleware,
};
pub mod stream_stashes;

pub static DEFAULT_USER_AGENT: &'static str = "Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0";
