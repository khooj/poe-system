use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemSocket {
    pub group: u32,
    pub attr: Option<String>,
    pub s_colour: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemProperty {
    pub name: String,
    pub values: Vec<Value>,
    pub display_mode: u32,
    pub progress: Option<f64>,
    #[serde(rename = "type")]
    pub item_type: Option<u32>,
    pub suffix: Option<String>,
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
pub struct UltimatumMod {
    #[serde(rename = "type")]
    pub mod_type: String,
    pub tier: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncubatedItem {
    pub name: String,
    pub level: u32,
    pub progress: u32,
    pub total: u32,
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

fn default_subcategories() -> Option<Vec<String>> {
    Some(vec![])
}

fn default_int<T>() -> Option<T>
where
    T: num::Integer + std::default::Default,
{
    Some(T::default())
}

fn default_bool() -> Option<bool> {
    Some(false)
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Extended {
    pub category: String,
    pub subcategories: Option<Vec<String>>,
    pub prefixes: Option<u32>,
    pub suffixes: Option<u32>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub verified: bool,
    pub w: i32,
    pub h: i32,
    pub icon: String,
    pub support: Option<bool>,
    pub stack_size: Option<i32>,
    pub max_stack_size: Option<i32>,
    pub league: Option<String>,
    pub id: Option<String>,
    pub influences: Option<Influences>,
    pub elder: Option<bool>,
    pub shaper: Option<bool>,
    pub abyss_jewel: Option<bool>,
    pub delve: Option<bool>,
    pub fractured: Option<bool>,
    pub synthesised: Option<bool>,
    pub sockets: Option<Vec<ItemSocket>>,
    pub socketed_items: Option<Vec<Item>>,
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
    pub properties: Option<Vec<ItemProperty>>,
    pub notable_properties: Option<Vec<ItemProperty>>,
    pub requirements: Option<Vec<ItemProperty>>,
    pub additional_properties: Option<Vec<ItemProperty>>,
    pub next_item_requirements: Option<Vec<ItemProperty>>,
    pub talisman_tier: Option<i32>,
    pub sec_descr_text: Option<String>,
    pub utility_mods: Option<Vec<String>>,
    pub implicit_mods: Option<Vec<String>>,
    pub ultimatum_mods: Option<Vec<UltimatumMod>>,
    pub explicit_mods: Option<Vec<String>>,
    pub crafted_mods: Option<Vec<String>>,
    pub enchant_mods: Option<Vec<String>>,
    pub fractured_mods: Option<Vec<String>>,
    pub cosmetic_mods: Option<Vec<String>>,
    pub veiled_mods: Option<Vec<String>>,
    pub veiled: Option<bool>,
    pub descr_text: Option<String>,
    pub prophecy_text: Option<String>,
    pub is_relic: Option<bool>,
    pub replica: Option<bool>,
    pub incubated_item: Option<IncubatedItem>,
    pub frame_type: Option<i32>,
    pub hybrid: Option<Hybrid>,
    pub extended: Option<Extended>,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub inventory_id: Option<String>,
    pub socket: Option<i32>,
    pub colour: Option<String>,
}

fn default_str() -> Option<String> {
    Some(String::new())
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublicStashChange {
    pub id: String,
    pub public: bool,
    #[serde(default = "default_str")]
    pub account_name: Option<String>,
    pub last_character_name: Option<String>,
    #[serde(default = "default_str")]
    pub stash: Option<String>,
    pub stash_type: String,
    pub league: Option<String>,
    pub items: Vec<Item>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PublicStashData {
    pub next_change_id: String,
    pub stashes: Vec<PublicStashChange>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("client error {0}")]
    ClientError(#[from] reqwest::Error),
    #[error("next cycle")]
    NextCycle,
}

#[async_trait]
pub trait Retriever {
    async fn get_latest_stash(&mut self, id: Option<&str>) -> Result<PublicStashData, Error>;
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
