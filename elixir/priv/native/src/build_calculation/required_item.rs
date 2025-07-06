use std::ops::RangeInclusive;

use crate::{
    data::ModValue as DataModValue,
    item::{
        types::{Category, Mod as DomainMod, ModValue, Subcategory, SubcategoryError, TypeError},
        Item,
    },
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use ts_rs::TS;

use super::mod_config::ModConfig;

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
#[ts(export)]
pub struct Mod {
    pub stat_id: String,
    pub text: String,
    pub current_value_int: Option<(i32, Option<i32>)>,
    pub current_value_float: Option<(f32, Option<f32>)>,
}

impl From<DomainMod> for Mod {
    fn from(value: DomainMod) -> Self {
        Mod {
            stat_id: value.stat_id,
            text: value.text,
            current_value_int: match value.numeric_value {
                ModValue::Exact(DataModValue::Int(i)) => Some((i, None)),
                ModValue::DoubleExact {
                    from: DataModValue::Int(a),
                    to: DataModValue::Int(b),
                } => Some((a, Some(b))),
                _ => None,
            },
            current_value_float: match value.numeric_value {
                ModValue::Exact(DataModValue::Float(i)) => Some((i, None)),
                ModValue::DoubleExact {
                    from: DataModValue::Float(a),
                    to: DataModValue::Float(b),
                } => Some((a, Some(b))),
                _ => None,
            },
        }
    }
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
        mods: Vec<(Mod, Option<ModConfig>)>,
        properties: Vec<Property>,
    },
    Weapon {
        quality: u8,
        mods: Vec<(Mod, Option<ModConfig>)>,
        properties: Vec<Property>,
    },
    Jewel {
        mods: Vec<(Mod, Option<ModConfig>)>,
    },
    Flask {
        quality: u8,
        mods: Vec<(Mod, Option<ModConfig>)>,
    },
    Accessory {
        quality: u8,
        mods: Vec<(Mod, Option<ModConfig>)>,
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
    pub fn mod_ids(&self) -> Vec<&str> {
        match self {
            ItemInfo::Armor { mods, .. } => mods.iter().map(|m| m.0.stat_id.as_str()).collect(),
            ItemInfo::Weapon { mods, .. } => mods.iter().map(|m| m.0.stat_id.as_str()).collect(),
            ItemInfo::Jewel { mods, .. } => mods.iter().map(|m| m.0.stat_id.as_str()).collect(),
            ItemInfo::Flask { mods, .. } => mods.iter().map(|m| m.0.stat_id.as_str()).collect(),
            ItemInfo::Accessory { mods, .. } => mods.iter().map(|m| m.0.stat_id.as_str()).collect(),
            ItemInfo::Gem { .. } => panic!("gems have no mods"),
        }
    }

    pub fn mods(&self) -> &[(Mod, Option<ModConfig>)] {
        match self {
            ItemInfo::Weapon { mods, .. } => &mods[..],
            ItemInfo::Armor { mods, .. } => &mods[..],
            ItemInfo::Gem { .. } => &[],
            ItemInfo::Flask { mods, .. } => &mods[..],
            ItemInfo::Jewel { mods, .. } => &mods[..],
            ItemInfo::Accessory { mods, .. } => &mods[..],
        }
    }

    pub fn mut_mods(&mut self) -> Option<&mut Vec<(Mod, Option<ModConfig>)>> {
        Some(match self {
            ItemInfo::Weapon { mods, .. } => mods,
            ItemInfo::Armor { mods, .. } => mods,
            ItemInfo::Gem { .. } => return None,
            ItemInfo::Flask { mods, .. } => mods,
            ItemInfo::Jewel { mods, .. } => mods,
            ItemInfo::Accessory { mods, .. } => mods,
        })
    }

    pub fn add_or_update_config(&mut self, stat_id: &str, cfg: ModConfig) -> bool {
        if matches!(self, ItemInfo::Gem { .. }) {
            return false;
        }

        let modcfg = self
            .mut_mods()
            .map(|mods| mods.iter_mut().find(|m| m.0.stat_id == stat_id))
            .unwrap_or_default();
        if let Some(mc) = modcfg {
            mc.1 = Some(cfg);
            return true;
        }
        false
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, TS, PartialEq)]
#[ts(export)]
pub struct RequiredItem {
    pub id: String,
    pub basetype: String,
    pub category: Category,
    pub subcategory: Subcategory,
    pub info: ItemInfo,
    pub name: String,
    pub search_basetype: bool,
    pub search_subcategory: bool,
    pub rarity: String,
}

impl RequiredItem {}

#[derive(Error, Debug)]
pub enum RequiredItemError {
    #[error("unknown item: {0}")]
    Unknown(String),
    #[error("unknown category: {0}")]
    UnknownCategory(#[from] TypeError),
    #[error("unknown subcategory: {0}")]
    UnknownSubcategory(#[from] SubcategoryError),
}

impl TryFrom<Item> for RequiredItem {
    type Error = RequiredItemError;
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
            return Err(RequiredItemError::Unknown(format!(
                "at category check: {} {}",
                value.name, value.base_type
            )));
        }

        let basetype = value.base_type;
        let category = Category::get_from_basetype(&basetype)?;
        let subcategory = Subcategory::get_from_basetype(&basetype)?;
        let mods = value.mods.into_iter().map(|m| (m.into(), None)).collect();
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
        Ok(RequiredItem {
            info: info.ok_or(RequiredItemError::Unknown(format!(
                "at info: {} {}",
                value.name, basetype
            )))?,
            id: value.id,
            basetype,
            category,
            subcategory,
            name: value.name,
            search_basetype: false,
            search_subcategory: false,
            rarity: value.rarity.into(),
        })
    }
}
