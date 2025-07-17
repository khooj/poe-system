pub mod comparison;
pub mod item_config;
pub mod stored_item;

use std::{collections::HashMap, str::FromStr};

use item_config::{ItemConfig, ItemConfigOption, ModOption, ModStatId};
use serde::{Deserialize, Serialize};
use stored_item::{ItemInfo, StoredItem};
use strum::EnumString;
use ts_rs::TS;

use crate::data::MODS;

#[derive(Serialize, Deserialize, Debug, Default, TS)]
#[ts(export)]
pub struct BuildInfo {
    pub provided: BuildItemsWithConfig,
    pub found: FoundBuildItems,
}

#[derive(Serialize, Deserialize, Debug, Default, TS)]
#[ts(export)]
pub enum BuildV1 {
    #[default]
    V1,
}

#[derive(Serialize, Deserialize, Debug, Default, TS)]
#[ts(export)]
pub struct BuildItemsWithConfig {
    pub helmet: ItemWithConfig,
    pub body: ItemWithConfig,
    pub boots: ItemWithConfig,
    pub gloves: ItemWithConfig,
    pub weapon1: ItemWithConfig,
    pub weapon2: ItemWithConfig,
    pub ring1: ItemWithConfig,
    pub ring2: ItemWithConfig,
    pub belt: ItemWithConfig,
    pub flasks: Vec<ItemWithConfig>,
    pub gems: Vec<ItemWithConfig>,
    pub jewels: Vec<ItemWithConfig>,
    pub amulet: ItemWithConfig,
}

pub struct UnverifiedBuildItemsWithConfig<'a>(pub &'a mut BuildItemsWithConfig);

impl<'a> UnverifiedBuildItemsWithConfig<'a> {
    pub fn validate(self) -> Option<&'a mut BuildItemsWithConfig> {
        let mut items = vec![
            &self.0.helmet,
            &self.0.body,
            &self.0.boots,
            &self.0.gloves,
            &self.0.weapon1,
            &self.0.weapon2,
            &self.0.ring1,
            &self.0.ring2,
            &self.0.belt,
            &self.0.amulet,
        ];

        items.extend(
            [
                self.0.flasks.iter(),
                self.0.gems.iter(),
                self.0.jewels.iter(),
            ]
            .into_iter()
            .flatten(),
        );

        Some(self.0)
    }
}

#[derive(EnumString)]
#[strum(ascii_case_insensitive)]
pub enum FillRules {
    // for all rares set exist every mod,
    // uniques searched by name
    SimpleEverything,
    // for all rares set exist every mod except elemental resistances,
    // uniques searched by name
    SimpleNoRes,
}

impl BuildItemsWithConfig {
    pub fn mut_iter(&mut self) -> impl Iterator<Item = &mut ItemWithConfig> {
        let mut items = vec![
            &mut self.helmet,
            &mut self.body,
            &mut self.boots,
            &mut self.gloves,
            &mut self.weapon1,
            &mut self.weapon2,
            &mut self.ring1,
            &mut self.ring2,
            &mut self.belt,
            &mut self.amulet,
        ];

        items.extend(
            [
                self.flasks.iter_mut(),
                self.gems.iter_mut(),
                self.jewels.iter_mut(),
            ]
            .into_iter()
            .flatten(),
        );

        items.into_iter()
    }

    pub fn fill_configs_by_rule_s<T>(&mut self, rule: T)
    where
        T: AsRef<str>,
    {
        let rule = FillRules::from_str(rule.as_ref()).unwrap_or(FillRules::SimpleEverything);
        self.fill_configs_by_rule(rule);
    }

    pub fn fill_configs_by_rule(&mut self, rule: FillRules) {
        match rule {
            FillRules::SimpleEverything => self.fill_all(BuildItemsWithConfig::simple_everything),
            FillRules::SimpleNoRes => self.fill_all(BuildItemsWithConfig::simple_nores),
        }
    }

    fn simple_everything(item: &mut ItemWithConfig) {
        if item.item.rarity == "unique" {
            item.config.option = Some(ItemConfigOption::Unique);
        } else if matches!(item.item.info, ItemInfo::Gem { .. }) {
            item.config.basetype = true;
        } else {
            let mods = item
                .item
                .info
                .mut_mods()
                .iter_mut()
                .flat_map(|x| x.iter_mut())
                .fold(HashMap::new(), |mut acc, m| {
                    acc.insert(ModStatId::from(&m.stat_id), ModOption::Exist);
                    acc
                });
            item.config.option = Some(ItemConfigOption::Mods(mods));
        }
    }

    fn simple_nores(item: &mut ItemWithConfig) {
        if item.item.rarity == "unique" {
            item.config.option = Some(ItemConfigOption::Unique);
        } else if matches!(item.item.info, ItemInfo::Gem { .. }) {
            item.config.basetype = true;
        } else {
            let mods = item
                .item
                .info
                .mut_mods()
                .iter_mut()
                .flat_map(|x| x.iter_mut())
                .fold(HashMap::new(), |mut acc, m| {
                    if let Some(mt) = MODS::get_mod_data(&m.text) {
                        let tags = mt.mod_type().get_tags();
                        let elemental = tags.iter().any(|x| x == "elemental");
                        let resistance = tags.iter().any(|x| x == "resistance");
                        if !(elemental && resistance) {
                            acc.insert(ModStatId::from(&m.stat_id), ModOption::Exist);
                        }
                    }
                    acc
                });
            item.config.option = Some(ItemConfigOption::Mods(mods));
        }
    }

    fn fill_all<T>(&mut self, func: T)
    where
        for<'a> T: FnMut(&'a mut ItemWithConfig),
    {
        self.mut_iter().for_each(func);
    }
}

#[derive(Serialize, Deserialize, Debug, Default, TS, PartialEq)]
#[ts(export)]
pub struct ItemWithConfig {
    pub item: StoredItem,
    pub config: ItemConfig,
}

#[derive(Serialize, Deserialize, Debug, Default, TS)]
#[ts(export)]
pub struct FoundBuildItems {
    pub helmet: Option<StoredItem>,
    pub body: Option<StoredItem>,
    pub boots: Option<StoredItem>,
    pub gloves: Option<StoredItem>,
    pub weapon1: Option<StoredItem>,
    pub weapon2: Option<StoredItem>,
    pub ring1: Option<StoredItem>,
    pub ring2: Option<StoredItem>,
    pub belt: Option<StoredItem>,
    pub flasks: Option<Vec<StoredItem>>,
    pub gems: Option<Vec<StoredItem>>,
    pub jewels: Option<Vec<StoredItem>>,
    pub amulet: Option<StoredItem>,
}
