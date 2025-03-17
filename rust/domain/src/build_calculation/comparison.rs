use super::{mod_config::ModConfig, typed_item::TypedItem};
use crate::item::types::Mod;

pub struct Comparator {}

impl Comparator {
    pub fn extract_mods_for_search(mods_conf: &[ModConfig], item: &TypedItem) -> Vec<Mod> {
        let mods = item.mods();
        mods_conf
            .iter()
            .filter_map(|mc| mods.iter().find(|m| mc.stat_id == m.stat_id).cloned())
            .collect()
    }

    pub fn closest_item(mods_conf: Vec<ModConfig>, items: Vec<TypedItem>) -> Option<TypedItem> {
        let mods = items
            .iter()
            .enumerate()
            .map(|(idx, it)| (idx, it.mods()))
            .collect::<Vec<_>>();

        let candidates = mods
            .iter()
            .map(|(idx, mods)| {
                let accept = mods_conf.iter().all(|mc| mods.iter().any(|m| mc == m));
                (idx, accept)
            })
            .collect::<Vec<_>>();

        if candidates.is_empty() {
            None
        } else {
            candidates
                .iter()
                .find(|(_, accept)| *accept)
                .map(|s| items[*s.0].clone())
        }
    }
}
