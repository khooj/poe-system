use domain::types::{Mod, ModType};
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
        .map(|m| (m.as_str(), ModType::Explicit))
        .collect::<Vec<_>>();
    let mods = Mod::many_by_stat(&mods);

    mods
}
