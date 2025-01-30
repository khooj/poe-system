mod limit_middleware;
pub use limit_middleware::Limits as LimitMiddleware;
pub use reqwest;
pub use reqwest_middleware::{
    ClientBuilder, ClientWithMiddleware, Error as ReqwestMiddlewareError, Middleware,
};
pub mod stream_stashes;

pub static DEFAULT_USER_AGENT: &'static str = "OAuth costmybuild/0.1.0 (contant: bladoff@gmail.com)";
