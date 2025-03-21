pub mod comparison;
pub mod mod_config;
pub mod typed_item;

use mod_config::ModConfig;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use typed_item::TypedItem;

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

        for item in items {
            for cfg in &item.config {
                let modcfg_not_exist = !item.item.mods().iter().any(|m| m.stat_id == cfg.stat_id);
                if modcfg_not_exist {
                    return None;
                }
            }
        }

        Some(self.0)
    }
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
            item.config = applied_item.config.clone();
        }
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
    pub config: Vec<ModConfig>,
    pub item: TypedItem,
}

#[derive(Serialize, Deserialize, Debug, Default, TS)]
#[ts(export)]
pub struct FoundBuildItems {
    pub helmet: Option<TypedItem>,
    pub body: Option<TypedItem>,
    pub boots: Option<TypedItem>,
    pub gloves: Option<TypedItem>,
    pub weapon1: Option<TypedItem>,
    pub weapon2: Option<TypedItem>,
    pub ring1: Option<TypedItem>,
    pub ring2: Option<TypedItem>,
    pub belt: Option<TypedItem>,
    pub flasks: Option<Vec<TypedItem>>,
    pub gems: Option<Vec<TypedItem>>,
    pub jewels: Option<Vec<TypedItem>>,
    pub amulet: Option<TypedItem>,
}
