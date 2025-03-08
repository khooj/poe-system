use crate::types::{Category, Mod, Subcategory, SubcategoryError, TypeError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use ts_rs::TS;

#[derive(Debug)]
pub struct Stash {
    pub id: String,
    pub account: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, TS)]
#[ts(export)]
pub struct Property {
    pub augmented: bool,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(tag = "type")]
#[ts(export)]
pub enum ItemInfo {
    Gem {
        level: u8,
        quality: u8,
    },
    Armor {
        quality: u8,
        mods: Vec<Mod>,
        properties: Vec<Property>,
    },
    Weapon {
        quality: u8,
        mods: Vec<Mod>,
        properties: Vec<Property>,
    },
    Jewel {
        mods: Vec<Mod>,
    },
    Flask {
        quality: u8,
        mods: Vec<Mod>,
    },
}

impl Default for ItemInfo {
    fn default() -> Self {
        ItemInfo::Gem {
            level: 0,
            quality: 0,
        }
    }
}

impl ItemInfo {
    pub fn first_mod_id(&self) -> &str {
        match self {
            ItemInfo::Armor { mods, .. } => mods.first().map(|m| m.stat_id.as_str()).unwrap(),
            ItemInfo::Weapon { mods, .. } => mods.first().map(|m| m.stat_id.as_str()).unwrap(),
            ItemInfo::Jewel { mods, .. } => mods.first().map(|m| m.stat_id.as_str()).unwrap(),
            ItemInfo::Flask { mods, .. } => mods.first().map(|m| m.stat_id.as_str()).unwrap(),
            ItemInfo::Gem { .. } => panic!("gems have no mods"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, TS)]
#[ts(export)]
pub struct TypedItem {
    pub id: String,
    pub basetype: String,
    pub category: Category,
    pub subcategory: Subcategory,
    pub info: ItemInfo,
}

impl TypedItem {
    pub fn mods(&self) -> Vec<Mod> {
        match &self.info {
            ItemInfo::Weapon { mods, .. } => mods.clone(),
            ItemInfo::Armor { mods, .. } => mods.clone(),
            ItemInfo::Gem { .. } => vec![],
            ItemInfo::Flask { mods, .. } => mods.clone(),
            ItemInfo::Jewel { mods, .. } => mods.clone(),
        }
    }
}

#[derive(Error, Debug)]
pub enum TypedItemError {
    #[error("unknown item")]
    Unknown,
    #[error("unknown category: {0}")]
    UnknownCategory(#[from] TypeError),
    #[error("unknown subcategory: {0}")]
    UnknownSubcategory(#[from] SubcategoryError),
}
