use std::collections::HashMap;

use crate::{decode_config, encode_config, JsonHashMap, RustError, SerdeTermJson};

use super::item::{Item, ItemInfo};
use crate::atoms;
use domain::item::Item as DomainItem;
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
fn get_items_from_stash_data<'a>(env: Env<'a>, data: &'a str) -> NifResult<SerdeTerm<Vec<Item>>> {
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
            .filter_map(|i| DomainItem::try_from(i).ok())
            .map(|mut i| {
                if i.note.is_none() {
                    i.note = Some(stash.clone());
                }
                i
            })
            .filter_map(|i| Item::try_from(i).ok())
            // .map(|i| SerdeTerm(encode_config(env, &i).unwrap()))
            .collect::<Vec<_>>();
        res_items.append(&mut items);
    }

    Ok(SerdeTerm(res_items))
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
            .filter_map(|i| DomainItem::try_from(i).ok())
            .map(|mut i| {
                if i.note.is_none() {
                    i.note = Some(stash_name.clone());
                }
                if i.id.is_empty() {
                    i.id = Uuid::new_v4().to_string();
                }
                i
            })
            .filter_map(|i| Item::try_from(i).ok())
            .filter_map(|i| {
                if without_zero_price && i.price.is_some() && i.price.as_ref().unwrap().is_zero() {
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
fn get_item_type(info: SerdeTerm<Item>) -> NifResult<(Atom, Atom)> {
    let atom = match info.0.info {
        ItemInfo::Accessory { .. } => atoms::accessory(),
        ItemInfo::Armor { .. } => atoms::armor(),
        ItemInfo::Weapon { .. } => atoms::weapon(),
        ItemInfo::Jewel { .. } => atoms::jewel(),
        ItemInfo::Flask { .. } => atoms::flask(),
    };
    Ok((atoms::ok(), atom))
}

#[rustler::nif]
fn extract_flask_props(env: Env<'_>, req_item: SerdeTerm<Item>) -> NifResult<Term<'_>> {
    if let ItemInfo::Flask { quality, .. } = req_item.0.info {
        Ok((atoms::ok(), quality).encode(env))
    } else {
        Err(RustError::InvalidItem.into())
    }
}
