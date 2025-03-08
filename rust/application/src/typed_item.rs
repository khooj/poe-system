use core::convert::TryFrom;
use domain::{
    item::Item as DomainItem,
    types::{Category, Mod, ModType, Subcategory, SubcategoryError, TypeError},
};
use public_stash::models::Item;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use ts_rs::TS;
use uuid::Uuid;

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

impl TryFrom<Item> for TypedItem {
    type Error = TypedItemError;
    fn try_from(value: Item) -> core::result::Result<Self, Self::Error> {
        let ext = value.extended.ok_or(TypedItemError::Unknown)?;
        if !["weapons", "armour", "gems", "flasks", "jewels"].contains(&ext.category.as_str()) {
            return Err(TypedItemError::Unknown);
        }
        let basetype = value.base_type;
        let category = Category::get_from_basetype(&basetype)?;
        let subcategory = Subcategory::get_from_basetype(&basetype)?;
        let mods = value
            .explicit_mods
            .as_ref()
            .unwrap_or(&vec![])
            .as_slice()
            .iter()
            .filter_map(|s| Mod::try_by_stat(s.as_str(), ModType::Explicit).ok())
            .collect::<Vec<_>>();
        let props = value.properties.unwrap_or_default();
        let quality = props
            .iter()
            .find(|p| p.name == "Quality")
            .map(|q| q.values[0][0].value())
            .unwrap_or("+0%".to_string())[1..]
            .strip_suffix("%")
            .unwrap()
            .parse::<u8>()
            .unwrap_or(0);
        let info = match ext.category.as_str() {
            t @ ("weapons" | "armour") => {
                let properties = props
                    .iter()
                    .filter_map(|p| {
                        if p.values.is_empty() {
                            None
                        } else {
                            Some(Property {
                                augmented: p.values[0][1].value() == "1",
                                name: p.name.clone(),
                                value: p.values[0][0].value(),
                            })
                        }
                    })
                    .collect();

                Some(if t == "weapons" {
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
            "gems" => {
                let level = props
                    .iter()
                    .find(|p| p.name == "Level")
                    .map(|q| q.values[0][0].value())
                    .unwrap_or("0".to_string())
                    .parse::<u8>()
                    .unwrap_or(0);

                Some(ItemInfo::Gem { level, quality })
            }
            "flasks" => Some(ItemInfo::Flask { quality, mods }),
            "jewels" => Some(ItemInfo::Jewel { mods }),
            _ => None,
        };
        Ok(TypedItem {
            info: info.ok_or(TypedItemError::Unknown)?,
            id: value.id.unwrap_or(Uuid::new_v4().to_string()),
            basetype,
            category,
            subcategory,
        })
    }
}

// TODO: somehow unify item struct
impl TryFrom<DomainItem> for TypedItem {
    type Error = TypedItemError;
    fn try_from(value: DomainItem) -> core::result::Result<Self, Self::Error> {
        let cat = value.category;
        if ![
            Category::Weapons,
            Category::Armour,
            Category::Gems,
            Category::Flasks,
            Category::Jewels,
        ]
        .contains(&cat)
        {
            return Err(TypedItemError::Unknown);
        }

        let basetype = value.base_type;
        let category = Category::get_from_basetype(&basetype)?;
        let subcategory = Subcategory::get_from_basetype(&basetype)?;
        let mods = value.mods;
        let props = value.properties;
        let quality = value.quality as u8;
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
            _ => None,
        };
        Ok(TypedItem {
            info: info.ok_or(TypedItemError::Unknown)?,
            id: value.id,
            basetype,
            category,
            subcategory,
        })
    }
}
