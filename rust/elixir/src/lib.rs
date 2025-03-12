use std::collections::HashMap;

use pob::{build_import_pob::import_build_from_pob, Pob};
use rustler::{Binary, Encoder, Env, Error, SerdeTerm, Term};
use serde_json::Value;

mod atoms {
    rustler::atoms! {
        ok,
        error,
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
}

impl From<RustError> for Error {
    fn from(value: RustError) -> Self {
        Error::Term(Box::new(value.to_string()))
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn extract_build_config<'a>(
    env: Env<'a>,
    pobdata: Binary<'a>,
    itemset: Binary<'a>,
) -> Result<Term<'a>, Error> {
    let pobdata = String::from_utf8(pobdata.as_slice().to_vec()).map_err(RustError::from)?;
    let pob = Pob::from_pastebin_data(pobdata).map_err(RustError::from)?;
    let itemset = std::str::from_utf8(itemset.trim_ascii()).map_err(RustError::from)?;
    let info = import_build_from_pob(&pob, itemset).map_err(RustError::from)?;
    // TODO: optimize serialize/deserialize
    let json = serde_json::to_string(&info).map_err(RustError::from)?;
    let m: HashMap<String, Value> = serde_json::from_str(&json).map_err(RustError::from)?;
    let term = SerdeTerm(m).encode(env);
    Ok((atoms::ok(), term).encode(env))
}

rustler::init!("Elixir.RustPoe.Native");
