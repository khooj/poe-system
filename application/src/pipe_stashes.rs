use cassandra_cpp::{CassCollection, Map, Statement};
use domain::{Mod, ModType};
use public_stash::models::Item;
use uuid::Uuid;

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

pub fn insert_mods(mut stmt: Statement, item: &Item) -> Statement {
    stmt.bind_string(0, item.id.as_ref().unwrap_or(&Uuid::new_v4().to_string()))
        .unwrap();
    stmt.bind_string(1, &item.base_type).unwrap();
    let mods = parse_mods(&item);
    let mut affixes = Map::new();
    for m in mods {
        affixes.append_string(&m.stat_id).unwrap();
        affixes
            .append_string(
                &m.numeric_value
                    .map(|n| n.to_string())
                    .unwrap_or("-1".to_string()),
            )
            .unwrap();
    }
    stmt.bind_map(2, affixes).unwrap();
    stmt
}
