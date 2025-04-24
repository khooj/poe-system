use std::collections::HashMap;

use rustler::{Encoder, Env, Error, SerdeTerm, Term};
use serde_json::Value;

mod build_calculation;
mod config;

mod atoms {
    rustler::atoms! {
        ok,
        error,
        get_items,
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RustError {
    #[error("serde_json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("import build from pob: {0}")]
    ImportPob(#[from] pob::build_import_pob::ImportPobError),
    #[error("import pob from pastebin: {0}")]
    ImportPastebin(#[from] pob::PobError),
    #[error("convert u8 to string error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("convert u8 to str error: {0}")]
    StrUtf8(#[from] std::str::Utf8Error),
    #[error("invalid user provided build info")]
    InvalidUserBuildInfo,
}

impl From<RustError> for Error {
    fn from(value: RustError) -> Self {
        Error::Term(Box::new(value.to_string()))
    }
}

fn decode_config<T>(conf: HashMap<String, Value>) -> Result<T, RustError>
where
    T: for<'a> serde::de::Deserialize<'a>,
{
    let json = serde_json::to_string(&conf)?;
    let info = serde_json::from_str(&json)?;
    Ok(info)
}

// TODO: optimize serialize/deserialize
fn encode_config<'a, T>(env: Env<'a>, bi: &T) -> Result<Term<'a>, RustError>
where
    T: serde::ser::Serialize,
{
    let json = serde_json::to_string(bi)?;
    let m: HashMap<String, Value> = serde_json::from_str(&json)?;
    Ok(SerdeTerm(m).encode(env))
}

rustler::init!("Elixir.RustPoe.Native");
