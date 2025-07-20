use std::collections::HashMap;

use crate::{decode_config, encode_config, JsonHashMap, RustError, SerdeTermJson};

use super::atoms;
use domain::{
    build_calculation::{
        comparison::Comparator,
        stored_item::{ItemInfo as StoredItemInfo, StoredItem},
    },
    item::Item,
};
use pob::Pob;
use public_stash::models::PublicStashData;
use rustler::{Atom, Encoder, Env, NifResult, SerdeTerm, Term};
use serde::Serialize;
use serde_json::{Map, Value};
use uuid::Uuid;

struct WrapperMap<'a>(&'a Map<String, Value>);

impl<'a> From<WrapperMap<'a>> for HashMap<String, Value> {
    fn from(value: WrapperMap<'a>) -> Self {
        value
            .0
            .into_iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}

#[rustler::nif]
fn closest_item(
    env: Env<'_>,
    req_item: SerdeTermJson,
    SerdeTerm(items): SerdeTerm<Vec<JsonHashMap>>,
) -> NifResult<Term<'_>> {
    let req_item = decode_config(req_item)?;
    let items = items
        .into_iter()
        .map(|i| decode_config(SerdeTerm(i)).expect("cannot decode stored item in vec"))
        .collect();
    let result = Comparator::closest_item(&req_item, items);
    Ok((atoms::ok(), SerdeTerm(encode_config(env, &result)?)).encode(env))
}

#[rustler::nif]
fn get_items_from_stash_data<'a>(env: Env<'a>, data: &'a str) -> NifResult<Vec<SerdeTerm<Value>>> {
    let k: PublicStashData = serde_json::from_str(data).unwrap();
    let mut res_items = vec![];

    for d in k.stashes {
        if d.account_name.is_none() || d.stash.is_none() {
            continue;
        }
        let stash = d.stash.as_ref().unwrap();

        if d.items.is_empty() {
            continue;
        }

        let mut items = d
            .items
            .into_iter()
            .filter_map(|i| Item::try_from(i).ok())
            .map(|mut i| {
                if i.note.is_none() {
                    i.note = Some(stash.clone());
                }
                i
            })
            .filter_map(|i| StoredItem::try_from(i).ok())
            .map(|i| SerdeTerm(encode_config(env, &i).unwrap()))
            .collect::<Vec<_>>();
        res_items.append(&mut items);
    }

    Ok(res_items)
}

#[derive(Serialize)]
struct StashData {
    remove_stashes: Vec<String>,
    stashes: HashMap<String, (String, Vec<Value>)>,
    next_change_id: String,
}

#[rustler::nif]
fn process_stash_data<'a>(
    env: Env<'a>,
    data: &'a str,
    without_zero_price: bool,
) -> NifResult<(Atom, SerdeTerm<Value>)> {
    let k: PublicStashData = serde_json::from_str(data).unwrap();
    let mut result = StashData {
        stashes: HashMap::new(),
        remove_stashes: vec![],
        next_change_id: k.next_change_id,
    };

    for d in k.stashes {
        if d.account_name.is_none() || d.stash.is_none() {
            continue;
        }
        let stash_name = d.stash.as_ref().unwrap();

        if d.items.is_empty() {
            result.remove_stashes.push(d.id.clone());
            continue;
        }

        let items = d
            .items
            .into_iter()
            .filter_map(|i| Item::try_from(i).ok())
            .map(|mut i| {
                if i.note.is_none() {
                    i.note = Some(stash_name.clone());
                }
                if i.id.is_empty() {
                    i.id = Uuid::new_v4().to_string();
                }
                i
            })
            .filter_map(|i| StoredItem::try_from(i).ok())
            .filter_map(|i| {
                if without_zero_price && i.price.is_zero() {
                    None
                } else {
                    Some(i)
                }
            })
            .map(|i| encode_config(env, &i).unwrap())
            .collect::<Vec<_>>();
        result
            .stashes
            .insert(d.id.clone(), (d.league.unwrap_or(String::new()), items));
    }

    Ok((atoms::ok(), SerdeTerm(encode_config(env, &result).unwrap())))
}

#[rustler::nif]
fn get_stored_item_type(info: SerdeTermJson) -> NifResult<(Atom, Atom)> {
    let info: StoredItemInfo = decode_config(info)?;
    let atom = match info {
        StoredItemInfo::Accessory { .. } => atoms::accessory(),
        StoredItemInfo::Gem { .. } => atoms::gem(),
        StoredItemInfo::Armor { .. } => atoms::armor(),
        StoredItemInfo::Weapon { .. } => atoms::weapon(),
        StoredItemInfo::Jewel { .. } => atoms::jewel(),
        StoredItemInfo::Flask { .. } => atoms::flask(),
    };
    Ok((atoms::ok(), atom))
}

#[rustler::nif]
fn extract_gem_props(env: Env<'_>, req_item: SerdeTermJson) -> NifResult<Term<'_>> {
    let req_item: StoredItem = decode_config(req_item)?;
    if let StoredItemInfo::Gem { quality, level } = req_item.info {
        Ok((atoms::ok(), quality, level).encode(env))
    } else {
        Err(RustError::InvalidItem.into())
    }
}

#[rustler::nif]
fn extract_flask_props(env: Env<'_>, req_item: SerdeTermJson) -> NifResult<Term<'_>> {
    let req_item: StoredItem = decode_config(req_item)?;
    if let StoredItemInfo::Flask { quality, .. } = req_item.info {
        Ok((atoms::ok(), quality).encode(env))
    } else {
        Err(RustError::InvalidItem.into())
    }
}

#[rustler::nif]
fn get_itemsets_skillsets(pobdata: &str) -> NifResult<(Atom, Vec<String>, Vec<String>)> {
    let p = Pob::from_pastebin_data(pobdata.to_string()).map_err(RustError::from)?;
    let doc = p.as_document().map_err(RustError::from)?;
    let itemsets = doc.get_itemsets_list().map_err(RustError::from)?;
    let skillsets = doc.get_skillsets_list().map_err(RustError::from)?;
    Ok((atoms::ok(), itemsets, skillsets))
}
