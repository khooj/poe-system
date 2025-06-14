use std::collections::HashMap;

use crate::{decode_config, encode_config, RustError, SerdeTermJson};

use super::atoms;
use domain::build_calculation::{
    validate_and_apply_config, BuildInfo, FillRules, UnverifiedBuildItemsWithConfig,
};
use pob::{build_import_pob::import_build_from_pob, Pob};
use rustler::{Binary, Decoder, Encoder, Env, Error, NifResult, SerdeTerm, Term};
use serde_json::Value;

#[rustler::nif(schedule = "DirtyCpu")]
fn extract_build_config<'a>(
    env: Env<'a>,
    pobdata: &'a str,
    itemset: &'a str,
    skillset: &'a str,
) -> NifResult<Term<'a>> {
    Ok(extract_build_config_impl(env, pobdata, itemset, skillset)?)
}

fn extract_build_config_impl<'a>(
    env: Env<'a>,
    pobdata: &'a str,
    itemset: &'a str,
    skillset: &'a str,
) -> Result<Term<'a>, RustError> {
    let pob = Pob::from_pastebin_data(pobdata.to_string())?;
    let mut info = import_build_from_pob(&pob, itemset, skillset)?;
    info.provided.fill_configs_by_rule(FillRules::AllExist);
    let term = encode_config(env, &info)?;
    Ok((atoms::ok(), SerdeTerm(term)).encode(env))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn validate_and_apply_config(
    env: Env<'_>,
    extracted_config: SerdeTermJson,
    user_config: SerdeTermJson,
) -> NifResult<Term<'_>> {
    Ok(validate_and_apply_config_impl(
        env,
        extracted_config,
        user_config,
    )?)
}

fn validate_and_apply_config_impl(
    env: Env<'_>,
    extracted_config: SerdeTermJson,
    user_config: SerdeTermJson,
) -> Result<Term<'_>, RustError> {
    let mut extracted_config: BuildInfo = decode_config(extracted_config)?;
    let mut user_config: BuildInfo = decode_config(user_config)?;
    if validate_and_apply_config(
        &mut extracted_config.provided,
        UnverifiedBuildItemsWithConfig(&mut user_config.provided),
    ) {
        Ok((
            atoms::ok(),
            SerdeTerm(encode_config(env, &extracted_config)?),
        )
            .encode(env))
    } else {
        Err(RustError::InvalidUserBuildInfo)
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn validate_config(env: Env<'_>, config: SerdeTermJson) -> NifResult<Term<'_>> {
    Ok(validate_config_impl(env, config)?)
}

fn validate_config_impl(env: Env<'_>, config: SerdeTermJson) -> Result<Term<'_>, RustError> {
    // TODO: hint to compiler to not cut out?
    #![allow(unused)]
    let user_config: BuildInfo = decode_config(config)?;
    Ok(atoms::ok().encode(env))
}
