use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ClientSearchResponse {
    pub id: String,
    pub complexity: i32,
    pub result: Vec<String>,
    pub total: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct ClientFetchResponse {
    pub result: Vec<ClientFetchEntry>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct ClientFetchEntry {
    pub id: String,
    pub listing: ClientFetchListing,
    pub item: ClientFetchItem,
    #[serde(flatten)]
    pub rest: HashMap<String, Value>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct ClientFetchListing {
    pub method: String,
    pub indexed: String,
    pub stash: Option<StashInfo>,
    pub whisper: String,
    pub whisper_token: String,
    pub account: AccountInfo,
    pub price: Option<PriceInfo>,
    #[serde(flatten)]
    pub rest: HashMap<String, Value>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct StashInfo {
    pub name: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct AccountInfo {
    pub name: String,
    #[serde(rename = "lastCharacterName")]
    pub last_character_name: String,
    pub realm: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct PriceInfo {
    #[serde(rename = "type")]
    pub typ: String,
    pub amount: f32,
    pub currency: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct ClientFetchItem {
    pub extended: FetchItemExtended,
    #[serde(rename = "typeLine")]
    pub type_line: String,
    #[serde(rename = "baseType")]
    pub base_type: String,
    pub ilvl: i32,
    pub id: String,
    pub sockets: Option<Vec<SocketGroup>>,
    #[serde(flatten)]
    pub rest: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct FetchItemExtended {
    pub dps: Option<f32>,
    pub dps_aug: Option<bool>,
    pub edps: Option<f32>,
    pub hashes: Option<HashMap<String, Value>>,
    pub mods: Option<FetchItemMods>,
    pub pdps: Option<f32>,
    pub pdps_aug: Option<bool>,
    pub text: String,
    #[serde(flatten)]
    pub rest: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct FetchItemMods {
    pub explicit: Option<Vec<FetchItemMod>>,
    pub implicit: Option<Vec<FetchItemMod>>,
    #[serde(flatten)]
    pub rest: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct FetchItemMod {
    pub level: Option<i32>,
    pub magnitudes: Option<Vec<FetchItemModInfo>>,
    pub name: String,
    pub tier: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct FetchItemModInfo {
    pub hash: String,
    pub max: f32,
    pub min: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct SocketGroup {
    pub attr: String,
    pub group: i32,
    #[serde(rename = "sColour")]
    pub s_colour: String,
}

#[derive(Deserialize)]
pub struct ItemsData {
    pub result: Vec<DataItem>,
}

#[derive(Deserialize)]
pub struct DataItem {
    pub id: String,
    pub label: String,
    pub entries: Vec<DataItemEntry>,
}

#[derive(Deserialize)]
pub struct DataItemEntry {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub text: String,
    #[serde(default)]
    pub flags: DataItemEntryFlags,
}

#[derive(Deserialize, Default)]
pub struct DataItemEntryFlags {
    #[serde(default)]
    pub unique: bool,
}

#[derive(Deserialize)]
pub struct StatsData {
    pub result: Vec<Stats>,
}

#[derive(Deserialize)]
pub struct Stats {
    pub label: String,
    pub entries: Vec<Stat>,
}

#[derive(Deserialize)]
pub struct Stat {
    pub id: String,
    pub text: String,
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(default)]
    pub option: Options,
}

#[derive(Deserialize, Default)]
pub struct Options {
    pub options: Vec<OptionItem>,
}

#[derive(Deserialize, Default)]
pub struct OptionItem {
    pub id: i32,
    pub text: String,
}

#[derive(Deserialize)]
pub struct StaticData {
    pub result: Vec<Static>,
}

fn string_if_null<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let r: Option<String> = Option::deserialize(deserializer)?;
    Ok(r.unwrap_or_default())
}

#[derive(Deserialize)]
pub struct Static {
    pub id: String,
    #[serde(deserialize_with = "string_if_null")]
    pub label: String,
    pub entries: Vec<StaticItem>,
}

#[derive(Deserialize)]
pub struct StaticItem {
    pub id: String,
    pub text: String,
    #[serde(default)]
    pub image: String,
}

#[cfg(test)]
mod tests {
    use super::{ClientFetchResponse, ItemsData, StaticData, StatsData};
    static DATA: &str = include_str!("../dist/items.json");
    static STATS_DATA: &str = include_str!("../dist/stats.json");
    static STATIC_DATA: &str = include_str!("../dist/static.json");

    #[test]
    fn check_data() {
        let _: ItemsData = serde_json::from_str(DATA).unwrap();
        let _: StatsData = serde_json::from_str(STATS_DATA).unwrap();
        let _: StaticData = serde_json::from_str(STATIC_DATA).unwrap();
    }

    static TESTCASE1: &str = include_str!("testcase1.json");
    static TESTCASE2: &str = include_str!("testcase2.json");

    #[test]
    fn testcase1() {
        let _: ClientFetchResponse = serde_json::from_str(TESTCASE1).unwrap();
        let _: ClientFetchResponse = serde_json::from_str(TESTCASE2).unwrap();
    }
}
