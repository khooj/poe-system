use lazy_static::lazy_static;
use macros::static_array_from_file;
use serde::Deserialize;
use std::{collections::HashMap, fs::read};

pub fn cut_numbers(val: &str) -> String {
    val.replace(|el: char| el == '{' || el == '}' || el.is_numeric(), "")
}

#[derive(Deserialize)]
struct StatTranslation {
    #[serde(rename = "English")]
    english: Vec<LanguageStatTranslation>,
    ids: Vec<String>,
}

#[derive(Deserialize)]
struct LanguageStatTranslation {
    condition: Vec<StatCondition>,
    format: Vec<String>,
    string: String,
}

#[derive(Deserialize)]
struct StatCondition {
    min: Option<i16>,
    max: Option<i16>,
}

#[derive(Deserialize)]
struct BaseItem {
    name: String,
}

#[derive(Deserialize)]
struct Stat {
    is_aliased: bool,
    is_local: bool,
}

lazy_static! {
    static ref STATS: HashMap<String, Stat> = {
        let stats_file = include_bytes!("../dist/stats.min.json");
        serde_json::from_slice(stats_file).unwrap()
    };
    static ref STAT_TRANSLATIONS: Vec<StatTranslation> = {
        let stats_translations_file = include_bytes!("../dist/stat_translations.min.json");
        serde_json::from_slice(stats_translations_file).unwrap()
    };
    pub static ref STATS_CUTTED: HashMap<String, usize> = {
        STAT_TRANSLATIONS
            .iter()
            .enumerate()
            .map(|(idx, e)| (cut_numbers(&e.english[0].string), idx))
            .collect::<HashMap<String, usize>>()
    };
    static ref BASE_ITEMS: HashMap<String, BaseItem> = {
        let base_items_file = include_bytes!("../dist/base_items.min.json");
        serde_json::from_slice(base_items_file).unwrap()
    };
    pub static ref BASE_TYPES: Vec<&'static str> =
        BASE_ITEMS.iter().map(|(_, v)| v.name.as_str()).collect();
}

impl STATS_CUTTED {
    pub fn get_original_stat(idx: usize) -> String {
        STAT_TRANSLATIONS[idx].english[0].string.clone()
    }

    pub fn get_stat_id(idx: usize) -> String {
        STAT_TRANSLATIONS[idx].ids[0].clone()
    }
}
