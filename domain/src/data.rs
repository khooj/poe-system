use dashmap::{mapref::one::Ref, DashMap};
use lazy_static::lazy_static;
use regex::bytes::{Match, Regex};
use serde::Deserialize;
use serde_json::Value;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
    str::FromStr,
    sync::Mutex,
};

pub fn cut_numbers(val: &str) -> String {
    val.replace(|el: char| el == '{' || el == '}' || el.is_numeric(), "")
}

lazy_static! {
    static ref REGEX_REPLACE_NUMS: regex::bytes::Regex =
        regex::bytes::Regex::new(r"\+?(\([0-9-\.]+\)|[0-9\.]+)").unwrap();
    static ref REGEX_CAPTURE_GROUPS: regex::bytes::Regex =
        regex::bytes::Regex::new(r"(\+?\([0-9-\.]+\))").unwrap();
}

fn replace_for_regex(val: &str) -> (String, usize) {
    let count = REGEX_CAPTURE_GROUPS
        .captures(val.as_bytes())
        .map(|c| c.len().saturating_sub(1))
        .unwrap_or_default();
    let res = REGEX_REPLACE_NUMS.replace_all(val.as_bytes(), b"\\+?([0-9\\.]+)");

    let s = unsafe { String::from_utf8_unchecked(res.to_vec()) };
    (s, count)
}

fn extract_single_range(val: &str, mtch: Option<Match<'_>>) -> Option<RangeInclusive<i32>> {
    let mtch = mtch?;
    let p = &val[mtch.range()];
    let vals = p
        .trim_matches(['(', ')'])
        .split("-")
        .map(|p| p.parse().ok())
        .collect::<Vec<_>>();
    if vals.len() != 2 {
        return None;
    }

    match &vals[..] {
        [Some(v1), Some(v2)] => Some(*v1..=*v2),
        _ => None,
    }
}

fn get_range_reg(val: &str) -> (Option<RangeInclusive<i32>>, Option<RangeInclusive<i32>>) {
    dbg!(val);
    let captures = REGEX_REPLACE_NUMS
        .captures_iter(val.as_bytes())
        .map(|c| c.get(1))
        .collect::<Vec<_>>();
    dbg!(&captures);

    let range1 = extract_single_range(val, captures.first().and_then(|f| *f));
    let range2 = extract_single_range(val, captures.get(1).and_then(|f| *f));

    (range1, range2)
}

#[derive(thiserror::Error, Debug)]
pub enum ModValueError {
    #[error("parse error: {0}")]
    Parse(String),
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone)]
pub enum ModValue {
    Int(i32),
    Float(f32),
}

impl ModValue {
    pub fn as_int(&self) -> i32 {
        match self {
            ModValue::Int(i) => *i,
            _ => panic!("modvalue float"),
        }
    }
}

impl From<i32> for ModValue {
    fn from(value: i32) -> Self {
        ModValue::Int(value)
    }
}

impl From<f32> for ModValue {
    fn from(value: f32) -> Self {
        ModValue::Float(value)
    }
}

impl FromStr for ModValue {
    type Err = ModValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f = s.parse::<f32>();
        let i = s.parse::<i32>();

        Ok(match (f, i) {
            (Ok(fl), Err(_)) => fl.into(),
            (Err(_), Ok(int)) => int.into(),
            (Ok(_), Ok(int)) => int.into(),
            _ => return Err(ModValueError::Parse(s.to_string())),
        })
    }
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModInfo {}

#[derive(Debug, serde::Deserialize)]
struct ModTmp {
    text: Option<String>,
    domain: String,
    stats: Vec<ModStatsTmp>,
}

#[derive(Debug, serde::Deserialize, Clone)]
struct ModStatsTmp {
    id: String,
}

#[derive(
    Debug, Default, serde::Serialize, serde::Deserialize, PartialEq, Hash, PartialOrd, Eq, Ord,
)]
pub struct Id(String);

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum ModType {
    Variants(HashMap<Id, ModInfo>),
}

impl ModType {
    pub fn get_id(&self) -> String {
        match self {
            ModType::Variants(hm) => hm.keys().nth(0).map(|v| v.0.clone()).unwrap_or_default(),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct BasetypeInfo {
    pub properties: BasetypeProperties,
    pub name: String,
    pub tags: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct BasetypeProperties {
    pub attack_time: Option<i32>,
    pub critical_strike_chance: Option<i32>,
    pub range: Option<i32>,
    // TODO: add damage types
}

type SerializedModData = HashMap<String, ModType>;

const WHITELIST_DOMAINS: &[&str] = &[
    "abyss_jewel",
    "affliction_jewel",
    "crafted",
    "flask",
    "item",
    "unveiled",
];

const DOUBLE_MOD: &[&str] = &["Immunity to Ignite during Effect", "Removes Burning on use"];

macro_rules! hashmap {
    ($($k:expr => $v:expr),+) => {{
        use std::collections::HashMap;
        let mut hm = HashMap::new();
        $(
            hm.insert($k, $v);
        )+
        hm
    }};
}

fn fix_splitting(stats: Vec<String>) -> Vec<String> {
    let mut ret = vec![];
    if &stats[0] == "30% of Fire and Cold Damage taken as Lightning Damage while" {
        ret.push(format!("{}\n{}", &stats[0], &stats[1]));
        ret.extend(stats.into_iter().skip(2));
    } else {
        ret.extend(stats);
    }
    ret
}

pub fn prepare_data(mods_file: &[u8]) -> SerializedModData {
    let mods: HashMap<String, ModTmp> = serde_json::from_slice(mods_file).unwrap();
    mods.into_iter().fold(HashMap::new(), |mut acc, (_, m)| {
        if m.text.is_none() {
            return acc;
        }

        if !WHITELIST_DOMAINS.contains(&m.domain.as_str()) {
            return acc;
        }

        let mut mods = vec![];
        let stat_names: Vec<_> = m
            .text
            .as_ref()
            .unwrap()
            .split("\n")
            .map(String::from)
            .collect();
        let stat_names = fix_splitting(stat_names);
        let mut stats = m.stats.clone().into_iter();
        // used for mods with 2 stat_names and single size stats vec (equals stats)
        let mut prev_stat_count1 = None;
        // for mods with 2 stat_names and one stat for both
        let mut advanced_once = false;
        let mut prev_stat_count0 = None;
        for stat_name in stat_names {
            let (key, capture_groups_count) = replace_for_regex(&stat_name);
            // println!("key: {}, count: {}", key, capture_groups_count);
            if capture_groups_count == 0 {
                let id = if DOUBLE_MOD.contains(&stat_name.as_str()) {
                    if !advanced_once {
                        prev_stat_count0 = stats.next();
                        advanced_once = true;
                        prev_stat_count0
                            .as_ref()
                            .map(|v| v.id.clone())
                            .unwrap_or_default()
                    } else {
                        prev_stat_count0
                            .as_ref()
                            .map(|v| v.id.clone())
                            .unwrap_or_default()
                    }
                } else {
                    stats.next().map(|v| v.id).unwrap_or_default()
                };
                // TODO: skip empty ids for now
                if id.is_empty() {
                    continue;
                }

                mods.push((
                    key,
                    ModType::Variants(hashmap! {
                        Id(id) => ModInfo { }
                    }),
                ));
                continue;
            }

            if capture_groups_count == 1 {
                let stat = stats.next().or(prev_stat_count1).unwrap_or_else(|| {
                    panic!(
                        "unexpected mod in group: {:?} = {:?} (it len: {}, current stat: {})",
                        m.text,
                        m.stats,
                        stats.len(),
                        stat_name,
                    )
                });
                mods.push((
                    key,
                    ModType::Variants(hashmap! {
                        Id(stat.id.clone()) => ModInfo {
                    }}),
                ));
                prev_stat_count1 = Some(stat);
            } else if capture_groups_count == 2 {
                let stat1 = stats.next().unwrap();
                let stat2 = stats.next().unwrap();
                mods.push((
                    key,
                    ModType::Variants(hashmap! {
                        Id(stat1.id)  => ModInfo {
                        },
                        Id(stat2.id) =>  ModInfo { }
                    }),
                ));
            } else {
                panic!("unknown mod: {} = {:?}", stat_name, m.stats);
            }
        }

        for (key, modinfo) in mods {
            let _regex = Regex::new(&key).unwrap();
            let en = acc.entry(key).or_insert(ModType::Variants(HashMap::new()));
            match modinfo {
                ModType::Variants(hm) => match en {
                    ModType::Variants(hm2) => {
                        hm2.extend(hm.into_iter());
                    }
                },
            };
        }

        acc
    })
}

lazy_static! {
    static ref MAX_LOAD_RECORD_FOR_TEST: Mutex<RefCell<Option<usize>>> =
        Mutex::new(RefCell::new(None));
    pub static ref BASE_ITEMS: HashMap<String, BasetypeInfo> = {
        let base_items_file = include_bytes!("../dist/base_items.min.json");
        let data: HashMap<String, BasetypeInfo> = serde_json::from_slice(base_items_file).unwrap();
        data.into_iter().fold(HashMap::new(), |mut acc, info| {
            if info.1.name.is_empty() {
                return acc;
            }
            acc.entry(info.1.name.clone()).or_insert(info.1);
            acc
        })
    };
    pub static ref BASE_TYPES: HashSet<String> = BASE_ITEMS.keys().cloned().collect();
    pub static ref MODS: SerializedModData = {
        let mods_file = include_bytes!("../dist/mods.data");
        bincode::deserialize(mods_file).unwrap()
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

pub struct ModExtractor<'a> {
    re: Ref<'a, String, Regex>,
    m: &'a ModType,
}

impl<'a> ModExtractor<'a> {
    pub fn extract_values(&self, value: &str) -> (Option<ModValue>, Option<ModValue>) {
        match self.m {
            ModType::Variants(_) => {
                if let Some((num, num2)) = self
                    .re
                    .captures(value.as_bytes())
                    .map(|c| (c.get(1), c.get(2)))
                {
                    let num = num.and_then(|v| {
                        ModValue::from_str(std::str::from_utf8(v.as_bytes()).unwrap()).ok()
                    });
                    let num2 = num2.and_then(|v| {
                        ModValue::from_str(std::str::from_utf8(v.as_bytes()).unwrap()).ok()
                    });
                    (num, num2)
                } else {
                    (None, None)
                }
            }
        }
    }

    pub fn extract_by_range(
        &self,
        value: &str,
        range: f32,
    ) -> (Option<ModValue>, Option<ModValue>) {
        let (range1, range2) = get_range_reg(value);
        let v1 = range1
            .map(|s| (*s.start(), *s.end()))
            .map(|(st, en)| st + (((en - st) as f32) * range).trunc() as i32)
            .map(|val| ModValue::Int(val));
        let v2 = range2
            .map(|s| (*s.start(), *s.end()))
            .map(|(st, en)| st + (((en - st) as f32) * range).trunc() as i32)
            .map(|val| ModValue::Int(val));
        (v1, v2)
    }

    pub fn mod_type(&self) -> &ModType {
        self.m
    }
}

impl MODS {
    pub(crate) fn get_mod_data(value: &str) -> Option<ModExtractor<'static>> {
        let (k, _) = replace_for_regex(value);
        let mods = MODS.get(&k)?;
        let reg = LAZY_MODS_REGEX.get_regex(&k);
        Some(ModExtractor { re: reg, m: mods })
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::{Id, ModInfo, ModType, ModValue, MODS};

    #[test]
    fn check_load_time() {
        let start = Instant::now();
        MODS::get_mod_data("+50 to Evasion Rating");
        let delta = start.elapsed();
        println!("time loading mods: {}ms", delta.as_millis());
    }

    #[test]
    fn get_mod_data() {
        let m = "+22 to Strength";
        let ext = MODS::get_mod_data(m).unwrap();
        let (val1, val2) = ext.extract_values(m);
        assert_eq!(
            ext.mod_type(),
            &ModType::Variants(hashmap! {
                Id("additional_strength".to_string()) => ModInfo {
            }})
        );
        assert_eq!(val1, Some(ModValue::Int(22)));
        assert_eq!(val2, None);
    }

    #[test]
    fn get_mod_data_float() {
        let v = "+6.5% chance to Suppress Spell Damage";
        let ext = MODS::get_mod_data(v).unwrap();
        let (val1, val2) = ext.extract_values(v);
        assert_eq!(val1, Some(ModValue::Float(6.5)));
        assert_eq!(val2, None);
    }

    #[test]
    fn replace_for_regex() {
        assert_eq!(
            super::replace_for_regex("+(10-20)% increased Spell Damage"),
            ("\\+?([0-9\\.]+)% increased Spell Damage".to_string(), 1),
        );
        assert_eq!(
            super::replace_for_regex("+10 to Strength"),
            ("\\+?([0-9\\.]+) to Strength".to_string(), 0)
        );
        assert_eq!(
            super::replace_for_regex("Adds 2 Passive Skills"),
            ("Adds \\+?([0-9\\.]+) Passive Skills".to_string(), 0)
        );
        assert_eq!(
            super::replace_for_regex("+2 to Level of Socketed Support Gems"),
            (
                "\\+?([0-9\\.]+) to Level of Socketed Support Gems".to_string(),
                0
            )
        );
        assert_eq!(
            super::replace_for_regex("+42% to Fire Resistance"),
            ("\\+?([0-9\\.]+)% to Fire Resistance".to_string(), 0)
        );
        assert_eq!(
            super::replace_for_regex("+42% to Fire Resistance\nlong mod"),
            (
                "\\+?([0-9\\.]+)% to Fire Resistance\nlong mod".to_string(),
                0
            )
        );
    }

    #[test]
    fn get_range() {
        assert_eq!(
            super::get_range_reg("+(10-20)% increased Spell Damage"),
            (Some(10..=20), None),
        );
        assert_eq!(
            super::get_range_reg("+(10-20)% (30-40) increased Spell Damage"),
            (Some(10..=20), Some(30..=40)),
        );
        assert_eq!(super::get_range_reg("+10 to Strength"), (None, None),);
    }
}
