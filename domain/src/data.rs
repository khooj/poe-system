use dashmap::{mapref::one::Ref, DashMap};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::{cell::RefCell, collections::HashMap, ops::RangeInclusive, sync::Mutex};

pub fn cut_numbers(val: &str) -> String {
    val.replace(|el: char| el == '{' || el == '}' || el.is_numeric(), "")
}

fn replace_for_regex(val: &str) -> (String, usize) {
    lazy_static! {
        static ref REGEX_REPLACE_NUMS: regex::bytes::Regex =
            regex::bytes::Regex::new(r"\+?(\([0-9-]+\))").unwrap();
    }

    let mut count = 0;
    let mut res = val.as_bytes().to_owned();

    while REGEX_REPLACE_NUMS.is_match(&res) {
        let c = REGEX_REPLACE_NUMS.replace(&res, b"([0-9]+)");
        res = c.to_vec();
        count += 1;
    }

    let mut s = unsafe { String::from_utf8_unchecked(res.to_vec()) };
    if count == 0 {
        s = s.replace("+", "\\+");
    }
    (s, count)
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
    Static,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModInfo {
    pub id: String,
    pub text: String,
    pub value: ModValue,
}

#[derive(Debug, serde::Deserialize)]
struct ModTmp {
    text: Option<String>,
    domain: String,
    stats: Vec<ModStatsTmp>,
    groups: Vec<String>,
}

#[derive(Debug, serde::Deserialize, Clone)]
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

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum ModType {
    #[default]
    Unknown,
    Tiers(Vec<ModInfo>),
    Static(ModInfo),
}

impl ModType {
    pub fn get_id(&self) -> String {
        match self {
            ModType::Unknown => unreachable!(),
            ModType::Static(mi) => mi.id.clone(),
            ModType::Tiers(mis) => mis.first().map(|mi| mi.id.clone()).unwrap(),
        }
    }
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

const DOUBLE_MOD: &[&str] = &[
    "Immunity to Ignite during Effect",
    "Removes Burning on Use",
];

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
        let stat_names: Vec<_> = m.text.as_ref().unwrap().split("\n").collect();
        let mut stats = m.stats.clone().into_iter();
        // used for mods with 2 stat_names and single size stats vec (equals stats)
        let mut prev_stat_count1 = None;
        // for mods with 2 stat_names and one stat for both
        let mut advanced_once = false;
        let mut prev_stat_count0 = None;
        for stat_name in stat_names {
            let (key, capture_groups_count) = replace_for_regex(stat_name);
            // println!("key: {}, count: {}", key, capture_groups_count);
            if capture_groups_count == 0 {
                let id = if DOUBLE_MOD.contains(&stat_name) && !advanced_once {
                    prev_stat_count0 = stats.next();
                    advanced_once = true;
                    prev_stat_count0.as_ref().map(|v| v.id.clone()).unwrap_or_default()
                } else {
                    prev_stat_count0.as_ref().map(|v| v.id.clone()).unwrap_or_default()
                };
                mods.push((
                    key,
                    ModType::Static(ModInfo {
                        id,
                        text: stat_name.to_string(),
                        value: ModValue::Static,
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
                let min = extract_minmax(&stat.min);
                let max = extract_minmax(&stat.max);
                mods.push((
                    key,
                    ModType::Tiers(vec![ModInfo {
                        id: stat.id.clone(),
                        text: stat_name.to_string(),
                        value: match (min, max) {
                            (Some(m1), Some(m2)) => ModValue::MinMax(m1..=m2),
                            _ => panic!("unknown case"),
                        },
                    }]),
                ));
                prev_stat_count1 = Some(stat);
            } else if capture_groups_count == 2 {
                let stat = stats.next().unwrap();
                let min1 = extract_minmax(&stat.min);
                let max1 = extract_minmax(&stat.max);
                let stat = stats.next().unwrap();
                let min2 = extract_minmax(&stat.min);
                let max2 = extract_minmax(&stat.max);
                mods.push((
                    key,
                    ModType::Tiers(vec![ModInfo {
                        id: stat.id.clone(),
                        text: stat_name.to_string(),
                        value: match (min1, max1, min2, max2) {
                            (Some(min1), Some(max1), Some(min2), Some(max2)) => {
                                ModValue::DoubleMinMax {
                                    from: min1..=max1,
                                    to: min2..=max2,
                                }
                            }
                            _ => panic!("unknown case"),
                        },
                    }]),
                ));
            } else {
                panic!("unknown mod: {} = {:?}", stat_name, m.stats);
            }
        }

        for (key, modinfo) in mods {
            let _regex = Regex::new(&key).unwrap();
            let en = acc.entry(key).or_default();
            match modinfo {
                mt @ ModType::Static(_) => *en = mt,
                ModType::Tiers(mut v) => {
                    if let ModType::Tiers(v2) = en {
                        v2.append(&mut v);
                    } else {
                        *en = ModType::Tiers(v);
                    }
                }
                ModType::Unknown => continue,
            };
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

enum MatchVariant<'a> {
    String(&'a str),
    Number(i32),
}

impl<'a> From<&'a str> for MatchVariant<'a> {
    fn from(value: &'a str) -> Self {
        match value.parse().ok() {
            Some(v) => MatchVariant::Number(v),
            None => MatchVariant::String(value),
        }
    }
}

impl<'a> MatchVariant<'a> {
    fn as_number(&self) -> Option<i32> {
        match self {
            MatchVariant::Number(s) => Some(*s),
            _ => None,
        }
    }
}

// hashed key => enum {
//  Tiers(Vec<Info>),
//  Static(Info),
// }
// by capture groups count (stat => capture_group)
// 0 capture groups => static
// > 0 = tiers
//

impl MODS {
    pub(crate) fn get_mod_data(value: &str) -> Option<(&ModType, Option<i32>, Option<i32>)> {
        let (k, _) = replace_for_regex(value);
        let mods = MODS.get(&k)?;
        let reg = LAZY_MODS_REGEX.get_regex(&k);
        match mods {
            ModType::Unknown => None,
            m @ ModType::Static(_) => Some((m, None, None)),
            m @ ModType::Tiers(_) => {
                if let Some((num, num2)) = reg.captures(value).map(|c| (c.get(0), c.get(1))) {
                    let num = num.and_then(|v| v.as_str().parse().ok());
                    let num2 = num2.and_then(|v| v.as_str().parse().ok());
                    Some((m, num, num2))
                } else {
                    None
                }
            }
        }
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

        let (res, val1, val2) = super::MODS::get_mod_data("+22 to Strength").unwrap();
        assert_eq!(
            res,
            &super::ModType::Static(super::ModInfo {
                id: "additional_strength".to_string(),
                text: "+22 to Strength".to_string(),
                value: ModValue::Static,
            })
        );
        assert_eq!(val1, Some(22));
        assert_eq!(val2, None);
    }

    #[test]
    fn replace_for_regex() {
        assert_eq!(
            super::replace_for_regex("+(10-20)% increased Spell Damage"),
            ("([0-9]+)% increased Spell Damage".to_string(), 1),
        );
        assert_eq!(
            super::replace_for_regex("+10 to Strength"),
            ("\\+10 to Strength".to_string(), 0)
        );
        assert_eq!(
            super::replace_for_regex("Adds 2 Passive Skills"),
            ("Adds 2 Passive Skills".to_string(), 0)
        );
        assert_eq!(
            super::replace_for_regex("+2 to Level of Socketed Support Gems"),
            ("\\+2 to Level of Socketed Support Gems".to_string(), 0)
        );
    }
}
