pub mod comparison;
pub mod item_config;
pub mod stored_item;

use std::{collections::HashMap, str::FromStr};

use item_config::{ItemConfig, ItemConfigOption, ModOption, ModStatId};
use rustler::NifStruct;
use serde::{Deserialize, Serialize};
use stored_item::{ItemInfo, StoredItem};
use strum::EnumString;
use ts_rs::TS;

use crate::data::MODS;

#[derive(Serialize, Deserialize, Debug, Default, NifStruct)]
#[module = "PoeSystem.Build.BuildInfo"]
pub struct BuildInfo {
    pub provided: BuildItemsWithConfig,
    pub found: FoundBuildItems,
}

#[derive(Serialize, Deserialize, Debug, Default, NifStruct)]
#[module = "PoeSystem.Build.ProvidedItems"]
pub struct BuildItemsWithConfig {
    pub helmet: Option<ItemWithConfig>,
    pub body: Option<ItemWithConfig>,
    pub boots: Option<ItemWithConfig>,
    pub gloves: Option<ItemWithConfig>,
    pub weapon1: Option<ItemWithConfig>,
    pub weapon2: Option<ItemWithConfig>,
    pub ring1: Option<ItemWithConfig>,
    pub ring2: Option<ItemWithConfig>,
    pub belt: Option<ItemWithConfig>,
    pub flasks: Vec<ItemWithConfig>,
    pub gems: Vec<ItemWithConfig>,
    pub jewels: Vec<ItemWithConfig>,
    pub amulet: Option<ItemWithConfig>,
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
        let mut items = [
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
        ]
        .into_iter()
        .filter_map(|m| m.as_mut())
        .collect::<Vec<_>>();

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

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, NifStruct)]
#[module = "PoeSystem.Items.NativeItem"]
pub struct ItemWithConfig {
    pub item: StoredItem,
    pub config: ItemConfig,
}

#[derive(Serialize, Deserialize, Debug, Default, NifStruct)]
#[module = "PoeSystem.Build.FoundItems"]
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
