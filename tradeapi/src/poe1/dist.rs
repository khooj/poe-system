use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct StatsImpl {
    result: Vec<StatCategory>,
}

#[derive(Deserialize)]
struct StatCategory {
    label: String,
    entries: Vec<Stat>,
}

#[derive(Deserialize)]
struct Stat {
    id: String,
    text: String,
    option: Option<Options>,
}

#[derive(Deserialize)]
struct Options {
    pub options: Vec<InnerOption>,
}

#[derive(Deserialize)]
struct InnerOption {
    id: u16,
    text: String,
}

lazy_static! {
    static ref STATS: StatsImpl = {
        let f = include_bytes!("../../dist/stats.min.json");
        serde_json::from_slice(f).unwrap()
    };
    pub static ref STAT_TO_ID: HashMap<&'static str, &'static str> = {
        STATS
            .result
            .iter()
            .flat_map(|el| &el.entries)
            .map(|el| (el.text.as_str(), el.id.as_str()))
            .collect()
    };
    pub static ref STATS_IDS: Vec<&'static str> = STAT_TO_ID.iter().map(|(_, v)| *v).collect();
}

#[cfg(test)]
mod tests {}
