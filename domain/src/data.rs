use lazy_static::lazy_static;
use regex::Regex;
use serde::{
    de::{Deserializer, Visitor},
    Deserialize,
};
use sonic_rs::{JsonValueTrait, Value};
use std::{cell::RefCell, collections::HashMap, sync::Mutex};

pub fn cut_numbers(val: &str) -> String {
    val.replace(|el: char| el == '{' || el == '}' || el.is_numeric(), "")
}

fn replace_for_regex(val: &str) -> String {
    let b = REGEX_REPLACE_NUMS.replace_all(val.as_bytes(), b"([0-9]+)");
    unsafe { String::from_utf8_unchecked(b.to_vec()) }
}

#[derive(Deserialize)]
struct BaseItem {
    name: String,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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

pub fn prepare_data(mods_file: &[u8]) -> HashMap<String, Vec<ModValues>> {
    let mods: Vec<ModTmp> = sonic_rs::from_slice(mods_file).unwrap();
    mods.into_iter().fold(HashMap::new(), |mut acc, m| {
        if m.text.is_none() {
            return acc;
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
        let v = ModValues {
            id: stats.id.clone(),
            min,
            max,
            text: stat_name.clone(),
        };

        let k = replace_for_regex(&stat_name);
        let _regex = Regex::new(&k).unwrap();
        let en = acc.entry(k).or_default();
        en.push(v);
        acc
    })
}

lazy_static! {
    pub(crate) static ref REGEX_REPLACE_NUMS: regex::bytes::Regex =
        regex::bytes::Regex::new(r"\+?(\(.+\)|[0-9]+)").unwrap();
    static ref MAX_LOAD_RECORD_FOR_TEST: Mutex<RefCell<Option<usize>>> =
        Mutex::new(RefCell::new(None));
    static ref BASE_ITEMS: HashMap<String, BaseItem> = {
        let base_items_file = include_bytes!("../dist/base_items.min.json");
        serde_json::from_slice(base_items_file).unwrap()
    };
    pub static ref BASE_TYPES: Vec<&'static str> =
        BASE_ITEMS.iter().map(|(_, v)| v.name.as_str()).collect();
    pub static ref MODS: HashMap<String, (Vec<ModValues>, Regex)> = {
        let mods_file = include_bytes!("../dist/mods.data");
        let mods: HashMap<String, Vec<ModValues>> = bincode::deserialize(mods_file).unwrap();
        let size = {
            let sz = MAX_LOAD_RECORD_FOR_TEST.lock().unwrap();
            let x = sz.borrow().unwrap_or(mods.len());
            x
        };
        mods.into_iter()
            .take(size)
            .map(|(k, v)| {
                let regex = Regex::new(&k).unwrap();
                (k, (v, regex))
            })
            .collect()
    };
}

impl MODS {
    pub(crate) fn get_mod_data(value: &str) -> Option<(&ModValues, Option<i32>)> {
        let (mods, reg) = MODS.get(&replace_for_regex(value))?;
        #[allow(clippy::never_loop)]
        for (_, [num]) in reg.captures_iter(value).map(|c| c.extract()) {
            let num = num.parse::<i32>().ok();
            return mods
                .iter()
                .find(|m| match (m.min, m.max, num) {
                    (Some(m1), Some(m2), Some(num)) => (m1..=m2).contains(&num),
                    (None, None, _) => true,
                    _ => unreachable!(),
                })
                .map(|m| (m, num));
        }
        mods.first().map(|m| (m, None))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    #[test]
    #[ignore]
    fn check_load_time() {
        let start = Instant::now();
        super::MODS::get_mod_data("+50 to Evasion Rating");
        let delta = start.elapsed();
        println!("time loading mods: {}ms", delta.as_millis());
    }

    #[test]
    #[ignore]
    fn get_mod_data() {
        {
            let m = super::MAX_LOAD_RECORD_FOR_TEST.lock().unwrap();
            m.borrow_mut().replace(100);
        }

        // println!("{:?}", *super::MODS);

        let (res, val) = super::MODS::get_mod_data("+22 to Strength").unwrap();
        assert_eq!(
            res,
            &super::ModValues {
                id: "additional_strength".to_string(),
                text: "+(18-22) to Strength".to_string(),
                min: Some(18),
                max: Some(22),
            }
        );
        assert_eq!(val, Some(22));
    }

    #[test]
    #[ignore]
    fn get_mod_regex() {
        {
            let m = super::MAX_LOAD_RECORD_FOR_TEST.lock().unwrap();
            m.borrow_mut().replace(100);
        }

        let res = super::MODS.get(" to Strength").unwrap();
        let re = &res.1;
        assert!(re.is_match("+10 to Strength"));
    }

    #[test]
    fn replace_for_regex() {
        assert_eq!(
            super::replace_for_regex("+(10-20)% increased Spell Damage"),
            "([0-9]+)% increased Spell Damage"
        );
        assert_eq!(
            super::replace_for_regex("+10 to Strength"),
            "([0-9]+) to Strength"
        );
    }
}
