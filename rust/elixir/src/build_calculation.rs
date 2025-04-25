use std::collections::HashMap;

use crate::{decode_config, encode_config};

use super::atoms;
use domain::build_calculation::{
    comparison::Comparator, required_item::RequiredItem, stored_item::StoredItem,
};
use rustler::{Encoder, Env, LocalPid, NifResult, SerdeTerm, Term};
use serde_json::Value;

#[rustler::nif]
fn extract_mods_for_search(
    env: Env<'_>,
    req_item: SerdeTerm<HashMap<String, Value>>,
) -> NifResult<Term<'_>> {
    let req_item: RequiredItem = decode_config(req_item.0)?;
    let mods = Comparator::extract_mods_for_search(&req_item);
    let mods: Vec<_> = mods.into_iter().map(|m| SerdeTerm(m.clone())).collect();
    Ok((atoms::ok(), mods).encode(env))
}

#[rustler::nif]
fn closest_item(
    env: Env<'_>,
    req_item: SerdeTerm<HashMap<String, Value>>,
    items: SerdeTerm<Vec<HashMap<String, Value>>>,
) -> NifResult<SerdeTerm<Option<StoredItem>>> {
    let req_item = decode_config(req_item.0)?;
    let items = items
        .0
        .into_iter()
        .map(|i| decode_config(i).expect("cannot decode stored item in vec"))
        .collect();
    let result = Comparator::closest_item(&req_item, items);
    Ok(SerdeTerm(result))
}
