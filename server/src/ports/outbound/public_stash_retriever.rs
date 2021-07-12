use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use thiserror::Error;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemSocket<'a> {
    pub group: i32,
    #[serde(borrow)]
    pub attr: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub s_colour: Option<Cow<'a, str>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemProperty<'a> {
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    pub values: Vec<Vec<Value>>,
    pub display_mode: i32,
    pub progress: Option<f64>,
    #[serde(rename = "type")]
    pub item_type: Option<i32>,
    #[serde(borrow)]
    pub suffix: Option<Cow<'a, str>>,
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

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UltimatumMod<'a> {
    #[serde(borrow, rename = "type")]
    pub mod_type: Cow<'a, str>,
    pub tier: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncubatedItem<'a> {
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    pub level: i32,
    pub progress: i32,
    pub total: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Hybrid<'a> {
    pub is_vaal_gem: Option<bool>,
    #[serde(borrow)]
    pub base_type_name: Cow<'a, str>,
    #[serde(borrow)]
    pub properties: Option<Vec<ItemProperty<'a>>>,
    #[serde(borrow)]
    pub explicit_mods: Option<Vec<Cow<'a, str>>>,
    #[serde(borrow)]
    pub sec_descr_text: Option<Cow<'a, str>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Extended<'a> {
    #[serde(borrow)]
    pub category: Cow<'a, str>,
    #[serde(borrow)]
    pub subcategories: Option<Vec<Cow<'a, str>>>,
    pub prefixes: Option<i32>,
    pub suffixes: Option<i32>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item<'a> {
    pub verified: bool,
    pub w: i32,
    pub h: i32,
    pub icon: String,
    pub support: Option<bool>,
    pub stack_size: Option<i32>,
    pub max_stack_size: Option<i32>,
    pub league: Option<&'a str>,
    pub id: Option<&'a str>,
    pub influences: Option<Influences>,
    pub elder: Option<bool>,
    pub shaper: Option<bool>,
    pub abyss_jewel: Option<bool>,
    pub delve: Option<bool>,
    pub fractured: Option<bool>,
    pub synthesised: Option<bool>,
    pub sockets: Option<Vec<ItemSocket<'a>>>,
    pub socketed_items: Option<Vec<Item<'a>>>,
    pub name: String,
    pub type_line: String,
    pub base_type: String,
    pub identified: bool,
    pub item_level: Option<i32>,
    pub note: Option<String>,
    pub locked_to_character: Option<bool>,
    pub locked_to_account: Option<bool>,
    pub duplicated: Option<bool>,
    pub split: Option<bool>,
    pub corrupted: Option<bool>,
    pub properties: Option<Vec<ItemProperty<'a>>>,
    pub notable_properties: Option<Vec<ItemProperty<'a>>>,
    pub requirements: Option<Vec<ItemProperty<'a>>>,
    pub additional_properties: Option<Vec<ItemProperty<'a>>>,
    pub next_item_requirements: Option<Vec<ItemProperty<'a>>>,
    pub talisman_tier: Option<i32>,
    pub sec_descr_text: Option<String>,
    #[serde(borrow)]
    pub utility_mods: Option<Vec<Cow<'a, str>>>,
    #[serde(borrow)]
    pub implicit_mods: Option<Vec<Cow<'a, str>>>,
    #[serde(borrow)]
    pub ultimatum_mods: Option<Vec<UltimatumMod<'a>>>,
    #[serde(borrow)]
    pub explicit_mods: Option<Vec<Cow<'a, str>>>,
    #[serde(borrow)]
    pub crafted_mods: Option<Vec<Cow<'a, str>>>,
    #[serde(borrow)]
    pub enchant_mods: Option<Vec<Cow<'a, str>>>,
    #[serde(borrow)]
    pub fractured_mods: Option<Vec<Cow<'a, str>>>,
    #[serde(borrow)]
    pub cosmetic_mods: Option<Vec<Cow<'a, str>>>,
    #[serde(borrow)]
    pub veiled_mods: Option<Vec<Cow<'a, str>>>,
    pub veiled: Option<bool>,
    pub descr_text: Option<String>,
    pub prophecy_text: Option<&'a str>,
    pub is_relic: Option<bool>,
    pub replica: Option<bool>,
    #[serde(borrow)]
    pub incubated_item: Option<IncubatedItem<'a>>,
    pub frame_type: Option<i32>,
    pub hybrid: Option<Hybrid<'a>>,
    #[serde(borrow)]
    pub extended: Option<Extended<'a>>,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub inventory_id: Option<&'a str>,
    pub socket: Option<i32>,
    pub colour: Option<&'a str>,
}

fn default_str() -> Option<String> {
    Some(String::new())
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublicStashChange<'a> {
    pub id: &'a str,
    pub public: bool,
    #[serde(default = "default_str")]
    pub account_name: Option<String>,
    #[serde(borrow)]
    pub last_character_name: Option<Cow<'a, str>>,
    #[serde(default = "default_str")]
    pub stash: Option<String>,
    pub stash_type: &'a str,
    pub league: Option<&'a str>,
    #[serde(borrow)]
    pub items: Vec<Item<'a>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PublicStashData<'a> {
    pub next_change_id: &'a str,
    #[serde(borrow)]
    pub stashes: Vec<PublicStashChange<'a>>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("client error {0}")]
    ClientError(#[from] ureq::Error),
    #[error("io error {0}")]
    IoError(#[from] std::io::Error),
    #[error("next cycle")]
    NextCycle,
    #[error("status code")]
    StatusCode(u16),
}

#[cfg(test)]
mod test {
    use super::PublicStashChange;

    const EXAMPLE_STASH_CHANGE: &str = include_str!("example-stash.json");
    #[test]
    fn deserializing_public_stash_change() -> Result<(), anyhow::Error> {
        let _: PublicStashChange = serde_json::from_str(&EXAMPLE_STASH_CHANGE)?;
        Ok(())
    }
}
