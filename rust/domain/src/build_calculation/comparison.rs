use tracing::{instrument, Level};

use super::{
    stored_item::{Mod, StoredItem},
    ItemWithConfig,
};

pub struct Comparator {}

impl Comparator {
    #[instrument(level = Level::TRACE)]
    pub fn closest_item(required_item: &StoredItem, items: Vec<StoredItem>) -> Option<StoredItem> {
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
                    .all(|req_mc| mods.iter().any(|m| req_mc.stat_id == m.stat_id));
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
