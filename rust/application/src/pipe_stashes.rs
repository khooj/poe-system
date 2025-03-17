use domain::item::types::{Mod, ModType};
use public_stash::models::Item;

pub fn parse_mods(item: &Item) -> Vec<Mod> {
    let mods = [
        &item.utility_mods,
        &item.enchant_mods,
        &item.scourge_mods,
        &item.implicit_mods,
        &item.explicit_mods,
        &item.crafted_mods,
        &item.fractured_mods,
        &item.veiled_mods,
    ];

    let mods: Vec<String> = mods
        .into_iter()
        .filter_map(|s| s.clone())
        .flatten()
        .collect::<Vec<_>>();

    let mods = mods
        .iter()
        .filter_map(|m| Mod::try_by_stat(m.as_str(), ModType::Explicit).ok())
        .collect::<Vec<_>>();

    mods
}
