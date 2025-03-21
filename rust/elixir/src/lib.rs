use std::collections::HashMap;

use domain::build_calculation::{
    validate_and_apply_config, BuildInfo, UnverifiedBuildItemsWithConfig,
};
use pob::{build_import_pob::import_build_from_pob, Pob};
use rustler::{Binary, Decoder, Encoder, Env, Error, NifResult, SerdeTerm, Term};
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
    #[error("invalid user provided build info")]
    InvalidUserBuildInfo,
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
    skillset: Binary<'a>,
) -> NifResult<Term<'a>> {
    Ok(extract_build_config_impl(env, pobdata, itemset, skillset)?)
}

fn extract_build_config_impl<'a>(
    env: Env<'a>,
    pobdata: Binary<'a>,
    itemset: Binary<'a>,
    skillset: Binary<'a>,
) -> Result<Term<'a>, RustError> {
    let pobdata = String::from_utf8(pobdata.as_slice().to_vec())?;
    let pob = Pob::from_pastebin_data(pobdata)?;
    let itemset = std::str::from_utf8(itemset.trim_ascii())?;
    let skillset = std::str::from_utf8(skillset.trim_ascii())?;
    let info = import_build_from_pob(&pob, itemset, skillset)?;
    let term = encode_config(env, &info)?;
    Ok((atoms::ok(), term).encode(env))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn validate_and_apply_config<'a>(
    env: Env<'a>,
    extracted_config: Term<'a>,
    user_config: Term<'a>,
) -> NifResult<Term<'a>> {
    let extracted = SerdeTerm::<HashMap<String, Value>>::decode(extracted_config)?;
    let user = SerdeTerm::<HashMap<String, Value>>::decode(user_config)?;
    Ok(validate_and_apply_config_impl(env, extracted, user)?)
}

fn decode_config(conf: SerdeTerm<HashMap<String, Value>>) -> Result<BuildInfo, RustError> {
    let json = serde_json::to_string(&conf.0)?;
    let info = serde_json::from_str(&json)?;
    Ok(info)
}

// TODO: optimize serialize/deserialize
fn encode_config<'a>(env: Env<'a>, bi: &BuildInfo) -> Result<Term<'a>, RustError> {
    let json = serde_json::to_string(bi)?;
    let m: HashMap<String, Value> = serde_json::from_str(&json)?;
    Ok(SerdeTerm(m).encode(env))
}

fn validate_and_apply_config_impl(
    env: Env<'_>,
    extracted_config: SerdeTerm<HashMap<String, Value>>,
    user_config: SerdeTerm<HashMap<String, Value>>,
) -> Result<Term<'_>, RustError> {
    let mut extracted = decode_config(extracted_config)?;
    let mut user = decode_config(user_config)?;
    if validate_and_apply_config(
        &mut extracted.provided,
        UnverifiedBuildItemsWithConfig(&mut user.provided),
    ) {
        Ok((atoms::ok(), encode_config(env, &extracted)?).encode(env))
    } else {
        Err(RustError::InvalidUserBuildInfo)
    }
}

rustler::init!("Elixir.RustPoe.Native");
