use std::ops::Deref;

use tracing::{instrument, Level};

use crate::{
    build_calculation::item_config::{ItemConfigOption, ModOption},
    data::MODS,
};

use super::{stored_item::StoredItem, ItemWithConfig};

pub struct Comparator {}

impl Comparator {
    #[instrument(level = Level::TRACE)]
    pub fn closest_item<'a>(
        required_item: &'a ItemWithConfig,
        items: Vec<StoredItem>,
    ) -> Option<StoredItem> {
        let mut preds: Vec<Box<dyn Fn(&StoredItem) -> bool>> = vec![];

        if let Some(ic) = &required_item.config.option {
            match ic {
                ItemConfigOption::Unique => {
                    preds.push(Box::new(|it: &StoredItem| {
                        required_item.item.name == it.name
                    }));
                }
                ItemConfigOption::Mods(mods) => {
                    mods.iter().for_each(|(k, v)| match v {
                        ModOption::Exist => preds.push(Box::new(|it| {
                            it.info.mods().iter().any(|m| &m.stat_id == k.deref())
                        })),
                        ModOption::Exact(val) => preds.push(Box::new(|it| {
                            let m = it.info.mods().iter().find(|m| &m.stat_id == k.deref());
                            if let Some(mm) = m {
                                let mod_data =
                                    MODS::get_mod_data(&mm.text).expect("mod should be found");
                                let v = mod_data.extract_values(&mm.text);
                                match v {
                                    (Some(mv1), None) => mv1 == *val,
                                    (Some(mv1), Some(mv2)) => mv1 <= *val && mv2 >= *val,
                                    _ => false,
                                }
                            } else {
                                false
                            }
                        })),
                        _ => {}
                    });
                }
            }
        }

        let candidates: Vec<_> = items
            .iter()
            .filter(|it| preds.iter().all(|pr| pr(it)))
            .collect();

        if candidates.is_empty() {
            None
        } else {
            candidates.first().cloned().cloned()
        }
    }
}
