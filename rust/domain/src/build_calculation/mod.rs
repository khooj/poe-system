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
