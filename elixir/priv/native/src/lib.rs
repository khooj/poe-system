use std::{collections::HashMap, error::Error as StdError};

use build_calculation::build_import_pob::ImportPobError;
use rustler::{Encoder, Env, Error, SerdeTerm, Term};
use serde_json::Value;
use serde_path_to_error::Path;

mod build_calculation;
mod config;

mod atoms {
    rustler::atoms! {
        ok,
        error,
        get_items,
        accessory,
        gem,
        armor,
        weapon,
        jewel,
        flask
    }
}

pub struct SerdePathError {}

#[derive(Debug, thiserror::Error)]
pub enum RustError {
    #[error("serde_json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("serde_json error path: {0}: {2} (data: {1})")]
    SerdeJsonPath(Path, String, Box<dyn StdError>),
    #[error("import build from pob: {0}")]
    ImportPob(#[from] ImportPobError),
    #[error("import pob from pastebin: {0}")]
    ImportPastebin(#[from] pob::PobError),
    #[error("convert u8 to string error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("convert u8 to str error: {0}")]
    StrUtf8(#[from] std::str::Utf8Error),
    #[error("invalid user provided build info")]
    InvalidUserBuildInfo,
    #[error("invalid item")]
    InvalidItem,
}

impl From<RustError> for Error {
    fn from(value: RustError) -> Self {
        Error::Term(Box::new(value.to_string()))
    }
}

type JsonHashMap = HashMap<String, Value>;
type SerdeTermJson = SerdeTerm<JsonHashMap>;

fn decode_config<T>(SerdeTerm(conf): SerdeTerm<HashMap<String, Value>>) -> Result<T, RustError>
where
    T: for<'a> serde::de::Deserialize<'a>,
{
    let json = serde_json::to_string(&conf)?;
    let info = &mut serde_json::Deserializer::from_str(&json);
    let info = serde_path_to_error::deserialize(info)
        .map_err(|e| RustError::SerdeJsonPath(e.path().clone(), json, Box::new(e)))?;
    Ok(info)
}

// TODO: optimize serialize/deserialize
fn encode_config<'a, T>(env: Env<'a>, bi: &T) -> Result<Value, RustError>
where
    T: serde::ser::Serialize,
{
    let json = serde_json::to_value(bi)?;
    Ok(json)
}

rustler::init!("Elixir.RustPoe.Native");
