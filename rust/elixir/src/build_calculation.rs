use crate::{decode_config, encode_config, JsonHashMap, SerdeTermJson};

use super::atoms;
use domain::{
    build_calculation::{
        comparison::Comparator, required_item::RequiredItem, stored_item::StoredItem,
    },
    item::Item,
};
use public_stash::models::{PublicStashChange, PublicStashData};
use rustler::{Encoder, Env, NifResult, SerdeTerm, Term};

#[rustler::nif]
fn extract_mods_for_search(env: Env<'_>, req_item: SerdeTermJson) -> NifResult<Term<'_>> {
    let req_item: RequiredItem = decode_config(req_item)?;
    let mods = Comparator::extract_mods_for_search(&req_item);
    let mods: Vec<_> = mods
        .into_iter()
        .map(|m| encode_config(env, m).expect("cannot encode config"))
        .collect();
    Ok((atoms::ok(), mods).encode(env))
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
    Ok((atoms::ok(), encode_config(env, &result)?).encode(env))
}

#[rustler::nif]
fn get_items_from_stash_data<'a>(env: Env<'a>, data: &'a str) -> NifResult<Vec<SerdeTermJson>> {
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
            .map(|i| encode_config(env, &i).unwrap())
            .collect::<Vec<_>>();
        res_items.append(&mut items);
    }

    Ok(res_items)
}
