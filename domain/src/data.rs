use dashmap::{mapref::one::Ref, DashMap};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::{cell::RefCell, collections::HashMap, ops::RangeInclusive, sync::Mutex};

pub fn cut_numbers(val: &str) -> String {
    val.replace(|el: char| el == '{' || el == '}' || el.is_numeric(), "")
}

fn replace_for_regex(val: &str) -> String {
    lazy_static! {
        static ref REGEX_REPLACE_NUMS: regex::bytes::Regex =
            regex::bytes::Regex::new(r"\+?(\(.+\)|[0-9]+)").unwrap();
    }

    let b = REGEX_REPLACE_NUMS.replace_all(val.as_bytes(), b"([0-9]+)");
    unsafe { String::from_utf8_unchecked(b.to_vec()) }
}

#[derive(Deserialize)]
struct BaseItem {
    name: String,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ModValue {
    MinMax(RangeInclusive<i32>),
    DoubleMinMax {
        from: RangeInclusive<i32>,
        to: RangeInclusive<i32>,
    },
    Exact(i32),
    DoubleExact {
        from: i32,
        to: i32,
    },
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModInfo {
    pub id: String,
    pub text: String,
    pub value: ModValue,
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

fn extract_minmax(val: &Value) -> Option<i32> {
    if val.is_number() {
        val.as_i64().map(|v| v as i32)
    } else {
        val.as_str().and_then(|s| s.parse().ok())
    }
}

type SerializedModData = HashMap<String, Vec<ModInfo>>;

pub fn prepare_data(mods_file: &[u8]) -> SerializedModData {
    let mods: HashMap<String, ModTmp> = serde_json::from_slice(mods_file).unwrap();
    mods.into_iter().fold(HashMap::new(), |mut acc, (_, m)| {
        if m.text.is_none() {
            return acc;
        }

        let mut mods = vec![];
        let stat_names: Vec<_> = m.text.as_ref().unwrap().split("\\n").collect();
        let stats = m.stats;
        if stat_names.len() == stats.len() {
            if stat_names.len() == 1 {
                let stat = stats.first().unwrap();
                let stat_name = stat_names.first().unwrap();
                let min = extract_minmax(&stat.min);
                let max = extract_minmax(&stat.max);
                mods.push((
                    stat_name.clone(),
                    ModInfo {
                        id: stat.id.clone(),
                        text: stat_name.to_string(),
                        value: match (min, max) {
                            (Some(m1), Some(m2)) if m1 == m2 => ModValue::Exact(m1),
                            (Some(m1), Some(m2)) => ModValue::MinMax(m1..=m2),
                            _ => panic!("unknown case"),
                        },
                    },
                ));
            } else if stat_names.len() == 2 {
                // double mod
                for (stat_name, stat) in stat_names.into_iter().zip(stats.into_iter()) {
                    let min = extract_minmax(&stat.min);
                    let max = extract_minmax(&stat.max);
                    mods.push((
                        stat_name.clone(),
                        ModInfo {
                            id: stat.id.clone(),
                            text: stat_name.to_string(),
                            value: match (min, max) {
                                (Some(m1), Some(m2)) if m1 == m2 => ModValue::Exact(m1),
                                (Some(m1), Some(m2)) => ModValue::MinMax(m1..=m2),
                                _ => panic!("unknown case"),
                            },
                        },
                    ));
                }
            }
        } else {
            // e.g. Adds 1 to 2 physical damage or Adds (2-3) to (3-4) damage
            let stat_name = stat_names.first().unwrap();
            let minmax: Vec<_> = stats
                .iter()
                .map(|s| (extract_minmax(&s.min), extract_minmax(&s.max)))
                .collect();
            if minmax.len() != 2 {
                eprintln!("unknown mod");
            } else {
                let id = format!("{};{}", stats[0].id, stats[1].id);
                let modvalue = match minmax[..] {
                    [(Some(f1), Some(f2)), (Some(t1), Some(t2))] if f1 == f2 && t1 == t2 => {
                        ModValue::DoubleExact { from: f1, to: t1 }
                    }
                    [(Some(f1), Some(f2)), (Some(t1), Some(t2))] if f2 > f1 && t2 > t1 => {
                        ModValue::DoubleMinMax {
                            from: f1..=f2,
                            to: t1..=t2,
                        }
                    }
                    _ => unreachable!(),
                };
                mods.push((
                    stat_name.clone(),
                    ModInfo {
                        id,
                        text: stat_name.to_string(),
                        value: modvalue,
                    },
                ));
            }
        }

        for (stat_name, modinfo) in mods {
            let k = replace_for_regex(&stat_name);
            let _regex = Regex::new(&k).unwrap();
            let en = acc.entry(k).or_default();
            en.push(modinfo);
        }

        acc
    })
}

lazy_static! {
    static ref MAX_LOAD_RECORD_FOR_TEST: Mutex<RefCell<Option<usize>>> =
        Mutex::new(RefCell::new(None));
    static ref BASE_ITEMS: HashMap<String, BaseItem> = {
        let base_items_file = include_bytes!("../dist/base_items.min.json");
        serde_json::from_slice(base_items_file).unwrap()
    };
    pub static ref BASE_TYPES: Vec<&'static str> =
        BASE_ITEMS.iter().map(|(_, v)| v.name.as_str()).collect();
    pub static ref MODS: SerializedModData = {
        let mods_file = include_bytes!("../dist/mods.data");
        let mods = bincode::deserialize(mods_file).unwrap();
        mods
    };
    static ref LAZY_MODS_REGEX: DashMap<String, Regex> = DashMap::new();
}

impl LAZY_MODS_REGEX {
    fn get_regex(&self, re: &str) -> Ref<'_, String, Regex> {
        self.entry(re.to_string())
            .or_insert(Regex::new(re).unwrap());
        self.get(re).unwrap()
    }
}

impl MODS {
    pub(crate) fn get_mod_data(value: &str) -> Option<(&ModInfo, Option<i32>)> {
        let k = replace_for_regex(value);
        let mods = MODS.get(&k)?;
        let reg = LAZY_MODS_REGEX.get_regex(&k);
        #[allow(clippy::never_loop)]
        for (num, num2) in reg.captures_iter(value).map(|c| (c.get(0), c.get(1))) {
            let num = num.map(|v| v.as_str().parse::<i32>().unwrap());
            let num2 = num2.map(|v| v.as_str().parse::<i32>().unwrap());
            return mods
                .iter()
                .find(|m| match (&m.value, num, num2) {
                    (ModValue::MinMax(range), Some(num), None) => range.contains(&num),
                    (ModValue::Exact(m), Some(num), None) => *m == num,
                    (ModValue::DoubleExact { from, to }, Some(num), Some(num2)) => {
                        *from == num && *to == num2
                    }
                    (ModValue::DoubleMinMax { from, to }, Some(num), Some(num2)) => {
                        from.contains(&num) && to.contains(&num2)
                    }
                    (_, _, _) => panic!("unknown mod"),
                })
                .map(|m| (m, num));
        }
        mods.first().map(|m| (m, None))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::ModValue;

    #[test]
    fn check_load_time() {
        let start = Instant::now();
        super::MODS::get_mod_data("+50 to Evasion Rating");
        let delta = start.elapsed();
        println!("time loading mods: {}ms", delta.as_millis());
    }

    #[test]
    fn get_mod_data() {
        {
            let m = super::MAX_LOAD_RECORD_FOR_TEST.lock().unwrap();
            m.borrow_mut().replace(100);
        }

        // println!("{:?}", *super::MODS);

        let (res, val) = super::MODS::get_mod_data("+22 to Strength").unwrap();
        assert_eq!(
            res,
            &super::ModInfo {
                id: "additional_strength".to_string(),
                text: "+(18-22) to Strength".to_string(),
                value: ModValue::MinMax(18..=22),
            }
        );
        assert_eq!(val, Some(22));
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
