use super::typed_item::TypedItem;
use crate::item::types::Mod;

pub struct Comparator {}

impl Comparator {
    pub fn extract_mods_for_search(item: &TypedItem) -> Vec<&Mod> {
        item.info
            .mods()
            .iter()
            .filter_map(|mc| mc.1.as_ref().and(Some(&mc.0)))
            .collect()
    }

    pub fn closest_item(required_item: &TypedItem, items: Vec<TypedItem>) -> Option<TypedItem> {
        let mods = items
            .iter()
            .enumerate()
            .map(|(idx, it)| (idx, it.info.mods()))
            .collect::<Vec<_>>();

        let req_mods = required_item.info.mods();

        let candidates = mods
            .iter()
            .map(|(idx, mods)| {
                let accept = req_mods
                    .iter()
                    .all(|req_mc| mods.iter().any(|m| req_mc.0.stat_id == m.0.stat_id));
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
