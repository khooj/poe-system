use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use sonic_rs::{JsonValueTrait, Value};
use std::collections::HashMap;

pub fn cut_numbers(val: &str) -> String {
    val.replace(|el: char| el == '{' || el == '}' || el.is_numeric(), "")
}

fn cut_numbers_inside(val: &str, st: char, end: char) -> String {
    replace_numbers_by(val, st, end, "")
}

fn cut_for_regex(val: &str, st: char, end: char, tmpl: &str) -> String {
    let val = val.replace(|el: char| ['(', ')', '+'].contains(&el), "");
    replace_numbers_by(&val, st, end, tmpl)
}

fn replace_numbers_by(val: &str, st: char, end: char, tmpl: &str) -> String {
    let mut val = val.to_string();
    let mut idx = 0usize;
    while let Some(start_idx) = val[idx..].find(st) {
        let stop_idx = val.find(end).unwrap();
        let range = start_idx+idx..=stop_idx;
        val.replace_range(range.clone(), tmpl);
        idx = stop_idx-(range.end()-range.start())+tmpl.len();
    }
    val
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

#[derive(Debug)]
pub struct ModValues {
    pub id: String,
    pub text: String,
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[derive(serde::Deserialize)]
struct ModTmp {
    text: Option<String>,
    stats: Vec<ModStatsTmp>,
}

#[derive(serde::Deserialize)]
struct ModStatsTmp {
    id: String,
    min: Value,
    max: Value,
}

lazy_static! {
    static ref REGEX_DEFAULT: Regex = Regex::new("\\d").unwrap();
    static ref STATS: HashMap<String, Stat> = {
        let stats_file = include_bytes!("../dist/stats.min.json");
        serde_json::from_slice(stats_file).unwrap()
    };
    static ref STAT_TRANSLATIONS: Vec<StatTranslation> = {
        let stats_translations_file = include_bytes!("../dist/stat_translations.min.json");
        serde_json::from_slice(stats_translations_file).unwrap()
    };
    pub(crate) static ref STATS_CUTTED: HashMap<String, usize> = {
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
    pub static ref MODS: HashMap<String, (Vec<ModValues>, Option<Regex>)> = {
        let mods_file = include_bytes!("../dist/mods.extracted.json");
        let mods: Vec<ModTmp> = sonic_rs::from_slice(mods_file).unwrap();
        mods.into_iter().fold(HashMap::new(), |mut acc, m| {
            if m.text.is_none() {
                return acc
            }

            let stat_name = m.text.unwrap();
            let stats = m.stats.first().unwrap();
            let min = if stats.min.is_number() {
                stats.min.as_i64().map(|v| v as i32)
            } else {
                stats.min.as_str().and_then(|s| s.parse().ok())
            };
            let max = if stats.max.is_number() {
                stats.max.as_i64().map(|v| v as i32)
            } else {
                stats.max.as_str().and_then(|s| s.parse().ok())
            };
            // let min = match &stats.min {
            //     Value::String(v) => v.parse().ok(),
            //     Value::Number(v) => v.as_i64().map(|v| v as i32),
            //     _ => None,
            // };
            // let max = match &stats.max {
            //     Value::String(v) => v.parse().ok(),
            //     Value::Number(v) => v.as_i64().map(|v| v as i32),
            //     _ => None,
            // };
            let v = ModValues {
                id: stats.id.clone(),
                min,
                max,
                text: stat_name.clone(),
            };

            let regex = Regex::new(&cut_for_regex(&stat_name, '(', ')', "([0-9]+)")).unwrap();
            let en = acc.entry(cut_numbers(&stat_name)).or_default();
            en.0.push(v);
            en.1 = Some(regex);
            acc
        })
    };
}

impl MODS {
    pub(crate) fn get_mod_data(value: &str) -> Option<&ModValues> {
        let (mods, reg) = MODS.get(&cut_numbers(value))?;
        if let Some(reg) = reg {
            for (_, [num]) in reg.captures_iter(value).map(|c| c.extract()) {
                let num = num.parse::<i32>().unwrap_or_default();
                return mods.iter().find(|m| match (m.min, m.max) {
                    (Some(m1), Some(m2)) => (m1..=m2).contains(&num),
                    (None, None) => true,
                    _ => unreachable!(),
                });
            }
            mods.first()
        } else {
            mods.first()
        }
    }
}

impl STATS_CUTTED {
    pub(crate) fn get_original_stat(idx: usize) -> String {
        STAT_TRANSLATIONS[idx].english[0].string.clone()
    }

    pub(crate) fn get_stat_id(idx: usize) -> String {
        STAT_TRANSLATIONS[idx].ids[0].clone()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    #[test]
    fn check_load_time() {
        let start = Instant::now();
        super::MODS::get_mod_data("+50 to Evasion Rating");
        let delta = start.elapsed();
        println!("time loading mods: {}ms", delta.as_millis());
    }

    #[test]
    fn replace_range() {
        let s = super::replace_numbers_by("asd(dsa)ds(ds)", '(', ')', "");
        assert_eq!(s, "asdds");
        let s = super::replace_numbers_by("(ddd)asd(dsa)ds(ds)", '(', ')', "");
        assert_eq!(s, "asdds");
        let s = super::replace_numbers_by("(ddd)asd", '(', ')', "");
        assert_eq!(s, "asd");
        let s = super::replace_numbers_by("+(10-20) to Evasion Rating", '(', ')', "([0-9]+)");
        assert_eq!(s, "+([0-9]+) to Evasion Rating");
    }
}
