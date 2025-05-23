pub mod comparison;
pub mod mod_config;
pub mod required_item;
pub mod stored_item;

use mod_config::ModConfig;
use required_item::{Mod, RequiredItem};
use serde::{Deserialize, Serialize};
use stored_item::StoredItem;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Default, TS)]
#[ts(export)]
pub struct BuildInfo {
    pub provided: BuildItemsWithConfig,
    pub found: FoundBuildItems,
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

pub enum FillRules {
    AllRanges,
    AllExist,
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

    pub fn apply(&mut self, other: &mut BuildItemsWithConfig) {
        let orig = self.mut_iter();
        let other = other.mut_iter();
        for (item, applied_item) in orig.zip(other) {
            let applied_mods = applied_item.item.info.mods();
            if let Some(mv) = item.item.info.mut_mods() {
                mv.iter_mut().for_each(|mc| {
                    mc.1 = applied_mods
                        .iter()
                        .find(|mc2| mc2.0.stat_id == mc.0.stat_id)
                        .map(|m| m.1.clone())
                        .unwrap_or_default();
                });
            };
        }
    }

    pub fn fill_configs_by_rule(&mut self, rule: FillRules) {
        match rule {
            FillRules::AllRanges => self.fill_all(BuildItemsWithConfig::all_ranges),
            FillRules::AllExist => self.fill_all(BuildItemsWithConfig::all_exist),
        }
    }

    fn all_ranges((m, cf): &mut (Mod, Option<ModConfig>)) {
        let (min, opt_max) = m.current_value_int.unwrap_or((0, Some(100)));
        *cf = Some(ModConfig::Range(min..=opt_max.unwrap_or(100)));
    }

    fn all_exist((_, cf): &mut (Mod, Option<ModConfig>)) {
        *cf = Some(ModConfig::Exist);
    }

    fn fill_all<T>(&mut self, func: T)
    where
        for<'a> T: FnMut(&'a mut (Mod, Option<ModConfig>)),
    {
        self.mut_iter()
            .filter_map(|ic| ic.item.info.mut_mods())
            .flat_map(|m| m.iter_mut())
            .for_each(func);
    }
}

pub fn validate_and_apply_config(
    original: &mut BuildItemsWithConfig,
    unverified: UnverifiedBuildItemsWithConfig,
) -> bool {
    if let Some(verified) = unverified.validate() {
        original.apply(verified);
        return true;
    }

    false
}

#[derive(Serialize, Deserialize, Debug, Default, TS, PartialEq)]
#[ts(export)]
pub struct ItemWithConfig {
    pub item: RequiredItem,
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
