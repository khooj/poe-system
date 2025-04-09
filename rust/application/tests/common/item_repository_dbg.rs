use std::collections::{HashMap, HashSet};

use application::storage::SearchItemsByModsTrait;
use domain::{build_calculation::stored_item::StoredItem, item::Item as DomainItem};
use public_stash::models::PublicStashData;

pub struct ItemRepositoryDbg {
    items: HashMap<String, StoredItem>,
}

#[async_trait::async_trait]
impl SearchItemsByModsTrait for ItemRepositoryDbg {
    async fn search_items_by_attrs(
        &mut self,
        basetype: Option<&str>,
        category: Option<domain::item::types::Category>,
        subcategory: Option<domain::item::types::Subcategory>,
        mods: Option<Vec<&domain::build_calculation::required_item::Mod>>,
    ) -> Result<
        Vec<domain::build_calculation::stored_item::StoredItem>,
        application::storage::ItemRepositoryError,
    > {
        Ok(self
            .items
            .values()
            .filter(|it| {
                let p1 = basetype.as_ref().is_none_or(|x| it.basetype == *x);
                let p2 = category.as_ref().is_none_or(|x| it.category == *x);
                let p3 = subcategory.as_ref().is_none_or(|x| it.subcategory == *x);
                let p4 = mods.as_ref().is_none_or(|x| {
                    let hs1: HashSet<String> =
                        HashSet::from_iter(x.iter().map(|s| s.stat_id.clone()));
                    let hs2 = HashSet::from_iter(it.info.mods().iter().map(|s| s.stat_id.clone()));
                    hs1.is_superset(&hs2)
                });
                p1 && p2 && p3 && p4
            })
            .cloned()
            .collect())
    }
}

impl ItemRepositoryDbg {
    pub fn import_items<T: AsRef<str>>(dir: T) -> anyhow::Result<Self> {
        let stashes = utils::stream_stashes::open_stashes(dir.as_ref());
        let mut items = HashMap::new();
        for (_, content) in stashes {
            let data: PublicStashData = serde_json::from_str(&content)?;
            for st in data.stashes {
                for itm in st.items {
                    let itm: DomainItem = match itm.try_into() {
                        Ok(i) => i,
                        Err(_) => continue,
                    };
                    let itm: StoredItem = itm.try_into()?;
                    items.insert(itm.id.clone(), itm);
                }
            }
        }

        Ok(ItemRepositoryDbg { items })
    }
}
