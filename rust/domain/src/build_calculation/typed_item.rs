use crate::item::{
    types::{Category, Mod, Subcategory, SubcategoryError, TypeError},
    Item,
};
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
    Accessory {
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
            ItemInfo::Accessory { mods, .. } => mods.first().map(|m| m.stat_id.as_str()).unwrap(),
            ItemInfo::Gem { .. } => panic!("gems have no mods"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, TS, PartialEq)]
#[ts(export)]
pub struct TypedItem {
    pub id: String,
    pub basetype: String,
    pub category: Category,
    pub subcategory: Subcategory,
    pub info: ItemInfo,
    pub name: String,
}

impl TypedItem {
    pub fn mods(&self) -> Vec<Mod> {
        match &self.info {
            ItemInfo::Weapon { mods, .. } => mods.clone(),
            ItemInfo::Armor { mods, .. } => mods.clone(),
            ItemInfo::Gem { .. } => vec![],
            ItemInfo::Flask { mods, .. } => mods.clone(),
            ItemInfo::Jewel { mods, .. } => mods.clone(),
            ItemInfo::Accessory { mods, .. } => mods.clone(),
        }
    }
}

#[derive(Error, Debug)]
pub enum TypedItemError {
    #[error("unknown item: {0}")]
    Unknown(String),
    #[error("unknown category: {0}")]
    UnknownCategory(#[from] TypeError),
    #[error("unknown subcategory: {0}")]
    UnknownSubcategory(#[from] SubcategoryError),
}

impl TryFrom<Item> for TypedItem {
    type Error = TypedItemError;
    fn try_from(value: Item) -> core::result::Result<Self, Self::Error> {
        let cat = value.category;
        if ![
            Category::Weapons,
            Category::Armour,
            Category::Gems,
            Category::Flasks,
            Category::Jewels,
            Category::Accessories,
        ]
        .contains(&cat)
        {
            return Err(TypedItemError::Unknown(format!(
                "at category check: {} {}",
                value.name, value.base_type
            )));
        }

        let basetype = value.base_type;
        let category = Category::get_from_basetype(&basetype)?;
        let subcategory = Subcategory::get_from_basetype(&basetype)?;
        let mods = value.mods;
        let props = value.properties;
        let quality = props
            .iter()
            .find_map(|p| {
                if p.name == "Quality" {
                    p.value
                        .as_ref()
                        .map(|s| s.trim_matches(['+', '%']))
                        .unwrap()
                        .parse()
                        .ok()
                } else {
                    None
                }
            })
            .unwrap_or_default();
        let info = match cat {
            t @ (Category::Weapons | Category::Armour) => {
                let properties = props
                    .iter()
                    .filter_map(|p| {
                        if p.value.is_none() {
                            None
                        } else {
                            Some(Property {
                                augmented: p.augmented,
                                name: p.name.clone(),
                                value: p.value.clone().unwrap(),
                            })
                        }
                    })
                    .collect();

                Some(if t == Category::Weapons {
                    ItemInfo::Weapon {
                        quality,
                        mods,
                        properties,
                    }
                } else {
                    ItemInfo::Armor {
                        quality,
                        mods,
                        properties,
                    }
                })
            }
            Category::Gems => {
                let level = props
                    .iter()
                    .find(|p| p.name == "Level")
                    .map(|q| q.value.clone().unwrap())
                    .unwrap_or("0".to_string())
                    .parse::<u8>()
                    .unwrap_or(0);

                Some(ItemInfo::Gem { level, quality })
            }
            Category::Flasks => Some(ItemInfo::Flask { quality, mods }),
            Category::Jewels => Some(ItemInfo::Jewel { mods }),
            Category::Accessories => Some(ItemInfo::Accessory { quality, mods }),
            _ => None,
        };
        Ok(TypedItem {
            info: info.ok_or(TypedItemError::Unknown(format!(
                "at info: {} {}",
                value.name, basetype
            )))?,
            id: value.id,
            basetype,
            category,
            subcategory,
            name: value.name,
        })
    }
}
