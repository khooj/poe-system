use public_stash::models::Item;
use core::convert::TryFrom;
use domain::{Mod, ModType};
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug)]
pub struct Stash {
    pub id: String,
    pub account: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Property {
    pub augmented: bool,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Clone)]
pub enum ItemInfo {
    Gem {
        basetype: String,
        level: u8,
        quality: u8,
    },
    Armor {
        basetype: String,
        quality: u8,
        mods: Vec<Mod>,
        properties: Vec<Property>,
    },
    Weapon {
        basetype: String,
        quality: u8,
        mods: Vec<Mod>,
        properties: Vec<Property>,
    },
    Jewel {
        basetype: String,
        mods: Vec<Mod>,
    },
    Flask {
        basetype: String,
        quality: u8,
        mods: Vec<Mod>,
    },
}

#[derive(Debug, Serialize, Clone)]
pub struct TypedItem {
    pub id: String,
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
}

impl TryFrom<Item> for TypedItem {
    type Error = TypedItemError;
    fn try_from(value: Item) -> core::result::Result<Self, Self::Error> {
        let ext = value.extended.ok_or(TypedItemError::Unknown)?;
        if !["weapons", "armour", "gems", "flasks", "jewels"].contains(&ext.category.as_str()) {
            return Err(TypedItemError::Unknown);
        }
        let basetype = value.base_type;
        let mods = Mod::many_by_stat(
            value
                .explicit_mods
                .as_ref()
                .unwrap_or(&vec![])
                .as_slice()
                .iter()
                .map(|s| (s.as_str(), ModType::Explicit))
                .collect::<Vec<_>>()
                .as_slice(),
        );
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
                        basetype,
                        quality,
                        mods,
                        properties,
                    }
                } else {
                    ItemInfo::Armor {
                        basetype,
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

                Some(ItemInfo::Gem {
                    basetype,
                    level,
                    quality,
                })
            }
            "flasks" => Some(ItemInfo::Flask {
                basetype,
                quality,
                mods,
            }),
            "jewels" => Some(ItemInfo::Jewel { basetype, mods }),
            _ => None,
        };
        Ok(TypedItem {
            info: info.ok_or(TypedItemError::Unknown)?,
            id: value.id.unwrap_or(Uuid::new_v4().to_string()),
        })
    }
}
