use std::collections::HashMap;

use crate::{
    atoms,
    build_calculation::{build_import_pob::import_build_from_pob, builds::BuildItems},
    decode_config, encode_config, RustError, SerdeTermJson,
};

use pob::Pob;
use rustler::{Atom, Binary, Decoder, Encoder, Env, Error, NifResult, SerdeTerm, Term};
use serde_json::Value;

#[rustler::nif(schedule = "DirtyCpu")]
fn extract_build_config<'a>(
    env: Env<'a>,
    pobdata: &'a str,
    itemset: &'a str,
    skillset: &'a str,
) -> NifResult<(Atom, SerdeTerm<BuildItems>)> {
    Ok(extract_build_config_impl(env, pobdata, itemset, skillset)?)
}

fn extract_build_config_impl<'a>(
    env: Env<'a>,
    pobdata: &'a str,
    itemset: &'a str,
    skillset: &'a str,
) -> Result<(Atom, SerdeTerm<BuildItems>), RustError> {
    let pob = Pob::from_pastebin_data(pobdata.to_string())?;
    let info = import_build_from_pob(&pob, itemset, skillset)?;
    Ok((atoms::ok(), SerdeTerm(info)))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn validate_config(env: Env<'_>, config: SerdeTerm<BuildItems>) -> NifResult<Atom> {
    Ok(atoms::ok())
}
