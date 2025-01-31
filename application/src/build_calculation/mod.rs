mod mod_config;
pub mod comparison;

use mod_config::ModConfig;
use serde::{Serialize, Deserialize};

use crate::typed_item::TypedItem;

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildInfo {
    pub provided: BuildItemsWithConfig,
    pub found: FoundBuildItems,
}

#[derive(Serialize, Deserialize, Debug)]
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemWithConfig {
    pub config: Vec<ModConfig>,
    pub item: TypedItem,
}

#[derive(Serialize, Deserialize, Debug)]
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
}
