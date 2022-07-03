use serde::{Deserialize, Deserializer};

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
    Ok(r.unwrap_or(String::new()))
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
    use super::{ItemsData, StaticData, StatsData};
    static DATA: &'static str = include_str!("../dist/items.json");
    static STATS_DATA: &'static str = include_str!("../dist/stats.json");
    static STATIC_DATA: &'static str = include_str!("../dist/static.json");

    #[test]
    fn check_data() {
        let _: ItemsData = serde_json::from_str(&DATA).unwrap();
        let _: StatsData = serde_json::from_str(&STATS_DATA).unwrap();
        let _: StaticData = serde_json::from_str(&STATIC_DATA).unwrap();
    }
}
