use std::collections::HashMap;

use crate::{decode_config, encode_config, JsonHashMap, RustError, SerdeTermJson};

use super::atoms;
use domain::{
    build_calculation::{
        comparison::Comparator,
        stored_item::{ItemInfo as StoredItemInfo, StoredItem},
        ItemWithConfig,
    },
    item::Item,
};
use pob::Pob;
use public_stash::models::PublicStashData;
use rustler::{Atom, Encoder, Env, NifResult, NifStruct, SerdeTerm, Term};
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
    req_item: ItemWithConfig,
    items: Vec<StoredItem>,
) -> NifResult<(Atom, Option<StoredItem>)> {
    let result = Comparator::closest_item(&req_item, items);
    Ok((atoms::ok(), result))
}

#[rustler::nif]
fn get_items_from_stash_data(data: &str) -> NifResult<Vec<StoredItem>> {
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
            .collect::<Vec<_>>();
        res_items.append(&mut items);
    }

    Ok(res_items)
}

#[derive(Serialize, NifStruct)]
#[module = "PoeSystem.StashData"]
struct StashData {
    remove_stashes: Vec<String>,
    stashes: HashMap<String, (String, Vec<StoredItem>)>,
    next_change_id: String,
}

#[rustler::nif]
fn process_stash_data(data: &str, without_zero_price: bool) -> NifResult<(Atom, StashData)> {
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
            .collect::<Vec<_>>();
        result
            .stashes
            .insert(d.id.clone(), (d.league.unwrap_or(String::new()), items));
    }

    Ok((atoms::ok(), result))
}

#[rustler::nif]
fn get_stored_item_type(item: StoredItem) -> NifResult<(Atom, Atom)> {
    let atom = match item.info {
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
fn extract_gem_props(req_item: StoredItem) -> NifResult<(Atom, u8, u8)> {
    if let StoredItemInfo::Gem { quality, level } = req_item.info {
        Ok((atoms::ok(), quality, level))
    } else {
        Err(RustError::InvalidItem.into())
    }
}

#[rustler::nif]
fn extract_flask_props(req_item: StoredItem) -> NifResult<(Atom, u8)> {
    if let StoredItemInfo::Flask { quality, .. } = req_item.info {
        Ok((atoms::ok(), quality))
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
