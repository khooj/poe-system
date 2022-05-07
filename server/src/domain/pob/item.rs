use crate::domain::item::Item as DomainItem;
use crate::domain::types::{Class, ItemLvl, League, Mod, ModType};

#[derive(Clone, Default, Debug)]
pub struct Item {
    pub name: String,
    pub item_lvl: ItemLvl,
    pub league: League,
    pub base_type: String,
    pub mods: Vec<Mod>,
    pub class: Class,
}

impl Into<DomainItem> for Item {
    fn into(self) -> DomainItem {
        let Item {
            name,
            item_lvl,
            league,
            base_type,
            mods,
            class,
        } = self;
        DomainItem {
            name,
            item_lvl,
            league,
            base_type,
            mods,
            class,
            ..DomainItem::default()
        }
    }
}
