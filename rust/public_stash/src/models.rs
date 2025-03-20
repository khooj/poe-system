use domain::item::{
    types::{
        Category, Influence, Mod, ModError, ModType, Property, Sockets, Subcategory,
        SubcategoryError, TypeError,
    },
    Item as DomainItem,
};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemSocket {
    pub group: i32,
    pub attr: Option<String>,
    pub s_colour: Option<String>,
}

struct OptSockets(Option<Vec<ItemSocket>>);

impl TryFrom<OptSockets> for Sockets {
    type Error = PublicStashError;

    fn try_from(value: OptSockets) -> Result<Self, Self::Error> {
        if let Some(v) = value.0 {
            let s = v.into_iter().chunk_by(|k| k.group).into_iter().fold(
                String::new(),
                |mut acc, s| {
                    acc.push_str(&s.1.map(|s| s.s_colour.unwrap()).join("-"));
                    acc.push(' ');
                    acc
                },
            );
            Ok(Sockets::try_from(s.trim_end())?)
        } else {
            Ok(Sockets::default())
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum PropertyValueType {
    Value(String),
    Type(i32),
}

impl PropertyValueType {
    pub fn value(&self) -> String {
        match self {
            PropertyValueType::Value(ref s) => s.clone(),
            PropertyValueType::Type(ref t) => t.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemProperty {
    pub name: String,
    pub values: Vec<Vec<PropertyValueType>>,
    pub display_mode: i32,
    pub progress: Option<f64>,
    #[serde(rename = "type")]
    pub item_type: Option<i32>,
    pub suffix: Option<String>,
}

impl TryFrom<ItemProperty> for Property {
    type Error = PublicStashError;

    fn try_from(value: ItemProperty) -> Result<Self, Self::Error> {
        if value.values.is_empty() {
            return Err(PublicStashError::EmptyProperty);
        }

        let [val, aug] = &value.values[0][..] else {
            return Err(PublicStashError::PropertyFormat);
        };
        Ok(Property {
            name: value.name,
            value: if let PropertyValueType::Value(s) = val {
                Some(s.clone())
            } else {
                None
            },
            augmented: matches!(aug, PropertyValueType::Type(1)),
        })
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Influences {
    pub shaper: Option<bool>,
    pub elder: Option<bool>,
    pub hunter: Option<bool>,
    pub crusader: Option<bool>,
    pub warlord: Option<bool>,
    pub redeemer: Option<bool>,
}

impl Influences {
    fn into_domain_influences(self) -> Vec<Influence> {
        let mut inf = vec![];
        if self.shaper.unwrap_or_default() {
            inf.push(Influence::Shaper);
        }
        if self.elder.unwrap_or_default() {
            inf.push(Influence::Elder);
        }
        if self.hunter.unwrap_or_default() {
            inf.push(Influence::Hunter);
        }
        if self.crusader.unwrap_or_default() {
            inf.push(Influence::Crusader);
        }
        if self.warlord.unwrap_or_default() {
            inf.push(Influence::Warlord);
        }
        if self.redeemer.unwrap_or_default() {
            inf.push(Influence::Redeemer);
        }
        inf
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UltimatumMod {
    #[serde(rename = "type")]
    pub mod_type: String,
    pub tier: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncubatedItem {
    pub name: String,
    pub level: i32,
    pub progress: i32,
    pub total: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Hybrid {
    pub is_vaal_gem: Option<bool>,
    pub base_type_name: String,
    pub properties: Option<Vec<ItemProperty>>,
    pub explicit_mods: Option<Vec<String>>,
    pub sec_descr_text: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Extended {
    pub category: String,
    pub subcategories: Option<Vec<String>>,
    pub prefixes: Option<i32>,
    pub suffixes: Option<i32>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub verified: bool,
    pub w: i32,
    pub h: i32,
    pub icon: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stack_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub league: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub influences: Option<Influences>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elder: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shaper: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tangled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abyss_jewel: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delve: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fractured: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthesised: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sockets: Option<Vec<ItemSocket>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socketed_items: Option<Vec<Item>>,
    pub name: String,
    pub type_line: String,
    pub base_type: String,
    pub identified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked_to_character: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked_to_account: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corrupted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<ItemProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notable_properties: Option<Vec<ItemProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<Vec<ItemProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<Vec<ItemProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_item_requirements: Option<Vec<ItemProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talisman_tier: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sec_descr_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utility_mods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logbook_mods: Option<Vec<LogbookMod>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enchant_mods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scourge_mods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_mods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultimatum_mods: Option<Vec<UltimatumMod>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_mods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crafted_mods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fractured_mods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cosmetic_mods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub veiled_mods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub veiled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descr_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavour_text: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavour_text_parsed: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prophecy_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_relic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incubated_item: Option<IncubatedItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scourged: Option<Scourged>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub art_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hybrid: Option<Hybrid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended: Option<Extended>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socket: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rarity: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum PublicStashError {
    #[error("unknown league: {0:?}")]
    UnknownLeague(Option<String>),
    #[error("mod parse error: {0}")]
    Mod(#[from] ModError),
    #[error("category type parse error: {0}")]
    Category(#[from] TypeError),
    #[error("subcategory parse error: {0}")]
    Subcategory(#[from] SubcategoryError),
    #[error("unknown property format")]
    PropertyFormat,
    #[error("empty property (not an error)")]
    EmptyProperty,
}

fn concat_option_vecs<T>(v: [Option<(Vec<T>, ModType)>; 7]) -> Vec<(Vec<T>, ModType)> {
    v.into_iter().fold(vec![], |mut acc, c| {
        if let Some(k) = c {
            acc.push(k);
        }
        acc
    })
}

impl TryFrom<Item> for DomainItem {
    type Error = PublicStashError;

    fn try_from(value: Item) -> Result<Self, Self::Error> {
        let mods = concat_option_vecs([
            value.explicit_mods.map(|m| (m, ModType::Explicit)),
            value.crafted_mods.map(|m| (m, ModType::Crafted)),
            value.veiled_mods.map(|m| (m, ModType::Veiled)),
            value.implicit_mods.map(|m| (m, ModType::Implicit)),
            value.utility_mods.map(|m| (m, ModType::Utility)),
            value.fractured_mods.map(|m| (m, ModType::Fractured)),
            value.enchant_mods.map(|m| (m, ModType::Enchant)),
        ]);

        let mods_it = mods
            .into_iter()
            .flat_map(|(m, t)| m.into_iter().map(move |s| Mod::try_by_stat(&s, t)));
        let mut mods = vec![];
        for it in mods_it {
            mods.push(it?);
        }

        let prop_it = value
            .properties
            .unwrap_or_default()
            .into_iter()
            .map(|p| p.try_into());
        let mut properties = vec![];
        for it in prop_it {
            match it {
                Err(PublicStashError::EmptyProperty) => continue,
                a => properties.push(a?),
            }
        }

        Ok(DomainItem {
            id: value.id.unwrap_or(Uuid::new_v4().to_string()),
            league: value.league.into(),
            rarity: value.rarity.unwrap_or_default().as_str().try_into()?,
            item_lvl: value.item_level,
            identified: value.identified,
            name: value.name,
            category: Category::get_from_basetype(&value.base_type)?,
            subcategories: Subcategory::get_from_basetype(&value.base_type)?,
            base_type: value.base_type,
            type_line: value.type_line,
            corrupted: value.corrupted.unwrap_or_default(),
            influences: value
                .influences
                .map(|i| i.into_domain_influences())
                .unwrap_or_default(),
            fractured: value.fractured.unwrap_or_default(),
            synthesised: value.synthesised.unwrap_or_default(),
            image_link: value.icon,
            sockets: OptSockets(value.sockets).try_into()?,
            properties,
            mods,
        })
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Scourged {
    // 1-3 for items, 1-10 for maps
    pub tier: i32,
    // monster level required to progress
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LogbookFaction {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LogbookMod {
    pub name: String,
    pub faction: LogbookFaction,
    pub mods: Vec<String>,
}

fn default_str() -> Option<String> {
    Some(String::new())
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublicStashChange {
    pub id: String,
    pub public: bool,
    #[serde(default = "default_str", skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_character_name: Option<String>,
    #[serde(default = "default_str")]
    pub stash: Option<String>,
    pub stash_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub league: Option<String>,
    pub items: Vec<Item>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PublicStashData {
    pub next_change_id: String,
    pub stashes: Vec<PublicStashChange>,
}

#[cfg(test)]
mod test {
    use super::PublicStashChange;

    const EXAMPLE_STASH_CHANGE: &str = include_str!("example-stash-influences.json");
    #[test]
    fn deserializing_public_stash_change() -> Result<(), anyhow::Error> {
        let _: PublicStashChange = serde_json::from_str(EXAMPLE_STASH_CHANGE)?;
        Ok(())
    }
}
