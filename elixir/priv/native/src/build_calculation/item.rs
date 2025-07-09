use domain::item::{
    types::{Category, Mod as DomainMod, Subcategory, SubcategoryError, TypeError},
    Item as DomainItem,
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
pub struct Mod {
    pub stat_id: String,
    pub text: String,
}

impl From<DomainMod> for Mod {
    fn from(value: DomainMod) -> Self {
        Mod {
            stat_id: value.stat_id,
            text: value.text,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(tag = "type")]
pub enum ItemInfo {
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

impl ItemInfo {
    pub fn mod_ids(&self) -> Vec<&str> {
        match self {
            ItemInfo::Armor { mods, .. } => mods.iter().map(|m| m.stat_id.as_str()).collect(),
            ItemInfo::Weapon { mods, .. } => mods.iter().map(|m| m.stat_id.as_str()).collect(),
            ItemInfo::Jewel { mods, .. } => mods.iter().map(|m| m.stat_id.as_str()).collect(),
            ItemInfo::Flask { mods, .. } => mods.iter().map(|m| m.stat_id.as_str()).collect(),
            ItemInfo::Accessory { mods, .. } => mods.iter().map(|m| m.stat_id.as_str()).collect(),
        }
    }

    pub fn mods(&self) -> &[Mod] {
        match self {
            ItemInfo::Weapon { mods, .. } => &mods[..],
            ItemInfo::Armor { mods, .. } => &mods[..],
            ItemInfo::Flask { mods, .. } => &mods[..],
            ItemInfo::Jewel { mods, .. } => &mods[..],
            ItemInfo::Accessory { mods, .. } => &mods[..],
        }
    }

    pub fn mut_mods(&mut self) -> Option<&mut Vec<Mod>> {
        Some(match self {
            ItemInfo::Weapon { mods, .. } => mods,
            ItemInfo::Armor { mods, .. } => mods,
            ItemInfo::Flask { mods, .. } => mods,
            ItemInfo::Jewel { mods, .. } => mods,
            ItemInfo::Accessory { mods, .. } => mods,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, TS, PartialEq)]
#[ts(export)]
pub enum Price {
    Chaos(i32),
    Divine(i32),
    Custom(String, i32),
}

impl Default for Price {
    fn default() -> Self {
        Price::Chaos(0)
    }
}

impl Price {
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Chaos(i) => *i == 0,
            Self::Divine(i) => *i == 0,
            Self::Custom(_, i) => *i == 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, TS, PartialEq)]
#[ts(export)]
pub struct Item {
    pub id: String,
    pub basetype: String,
    pub category: Category,
    pub subcategory: Subcategory,
    pub info: ItemInfo,
    pub name: String,
    pub price: Option<Price>,
    pub rarity: String,
}

lazy_static::lazy_static! {
    static ref PRICE_REGEX: regex::bytes::Regex = regex::bytes::Regex::new(r#"~(price|b/o) ([0-9\.]+) ([a-z]+)"#).unwrap();
}

impl Item {
    fn extract_price(s: &str) -> Option<Price> {
        let c = PRICE_REGEX.captures(s.as_bytes())?;
        let count = c.get(2)?;
        let curr = c.get(3)?;
        let count = std::str::from_utf8(count.as_bytes()).unwrap();
        let count: f32 = count.parse().unwrap_or_default();
        let count: i32 = unsafe { count.floor().to_int_unchecked() };
        Some(match curr.as_bytes() {
            b"chaos" => Price::Chaos(count),
            b"div" | b"divine" => Price::Divine(count),
            a => Price::Custom(String::from_utf8_lossy(a).to_string(), count),
        })
    }
}

#[derive(Error, Debug)]
pub enum ItemError {
    #[error("unknown item: {0}")]
    Unknown(String),
    #[error("unknown category: {0}")]
    UnknownCategory(#[from] TypeError),
    #[error("unknown subcategory: {0}")]
    UnknownSubcategory(#[from] SubcategoryError),
}

impl TryFrom<DomainItem> for Item {
    type Error = ItemError;
    fn try_from(value: DomainItem) -> core::result::Result<Self, Self::Error> {
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
            return Err(ItemError::Unknown(format!(
                "at category check: {} {}",
                value.name, value.base_type
            )));
        }

        let basetype = value.base_type;
        let category = Category::get_from_basetype(&basetype)?;
        let subcategory = Subcategory::get_from_basetype(&basetype)?;
        let mods = value.mods.into_iter().map(|m| m.into()).collect();
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
        let price = value.note.and_then(|s| Item::extract_price(&s));
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
            Category::Flasks => Some(ItemInfo::Flask { quality, mods }),
            Category::Jewels => Some(ItemInfo::Jewel { mods }),
            Category::Accessories => Some(ItemInfo::Accessory { quality, mods }),
            _ => None,
        };
        Ok(Item {
            info: info.ok_or(ItemError::Unknown(format!(
                "at info: {} {}",
                value.name, basetype
            )))?,
            id: value.id,
            basetype,
            category,
            subcategory,
            name: value.name,
            price,
            rarity: value.rarity.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_price() {
        assert_eq!(
            Some(Price::Chaos(10)),
            Item::extract_price("~price 10 chaos")
        );
        assert_eq!(Some(Price::Chaos(10)), Item::extract_price("~b/o 10 chaos"));
        assert_eq!(
            Some(Price::Divine(10)),
            Item::extract_price("~b/o 10 divine")
        );
        assert_eq!(
            Some(Price::Divine(10)),
            Item::extract_price("~b/o 10 divine custom text")
        );
        assert_eq!(
            Some(Price::Divine(10)),
            Item::extract_price("~b/o 10.99 divine custom text")
        );
        assert_eq!(
            Some(Price::Custom("alt".to_string(), 10)),
            Item::extract_price("~b/o 10.99 alt custom text")
        );
    }
}
