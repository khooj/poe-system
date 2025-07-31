use crate::{decode_config, encode_config, RustError, SerdeTermJson};

use super::atoms;
use domain::build_calculation::BuildInfo;
use pob::{build_import_pob::import_build_from_pob, Pob};
use rustler::{Atom, Encoder, Env, NifResult, NifStruct, SerdeTerm, Term};

#[rustler::nif(schedule = "DirtyCpu")]
fn extract_build_config<'a>(
    pobdata: &'a str,
    itemset: &'a str,
    skillset: &'a str,
    profile: &'a str,
) -> NifResult<(Atom, BuildInfo)> {
    Ok(extract_build_config_impl(
        pobdata, itemset, skillset, profile,
    )?)
}

fn extract_build_config_impl<'a>(
    pobdata: &'a str,
    itemset: &'a str,
    skillset: &'a str,
    profile: &'a str,
) -> Result<(Atom, BuildInfo), RustError> {
    let pob = Pob::from_pastebin_data(pobdata.to_string())?;
    let mut build = import_build_from_pob(&pob, itemset, skillset)?;
    build.provided.fill_configs_by_rule_s(profile);
    Ok((atoms::ok(), build))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn validate_config(config: BuildInfo) -> NifResult<Atom> {
    Ok(atoms::ok())
}

#[rustler::nif]
fn fill_configs_by_rule(cfg: BuildInfo, profile: &str) -> NifResult<(Atom, BuildInfo)> {
    let mut cfg = cfg;
    cfg.provided.fill_configs_by_rule_s(profile);
    Ok((atoms::ok(), cfg))
}
