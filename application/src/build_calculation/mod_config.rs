use std::{cmp::Ordering, ops::RangeInclusive};
use domain::Mod;

pub enum Config {
    Exact(i32),
    Range(RangeInclusive<i32>),
    Min(i32),
    Max(i32),
    Exist,
}

pub struct ModConfig {
    pub(crate) stat_id: String,
    configuration: Config,
}

impl ModConfig {}

impl PartialEq<Mod> for ModConfig {
    fn eq(&self, other: &Mod) -> bool {
        if self.stat_id != other.stat_id {
            return false;
        }

        if matches!(self.configuration, Config::Exist) {
            return true;
        }

        if other.numeric_value.is_none() {
            return false;
        }

        let val = other.numeric_value.unwrap();
        match &self.configuration {
            Config::Exact(v) => v.eq(&val),
            Config::Range(r) => r.contains(&val),
            Config::Min(m) => m <= &val,
            Config::Max(m) => m >= &val,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd<Mod> for ModConfig {
    fn partial_cmp(&self, other: &Mod) -> Option<Ordering> {
        if self.stat_id != other.stat_id {
            return None;
        }

        if matches!(self.configuration, Config::Exist) {
            return Some(Ordering::Equal);
        }

        let val = other.numeric_value?;
        match &self.configuration {
            Config::Exact(v) => v.partial_cmp(&val),
            Config::Range(r) => {
                if r.contains(&val) {
                    Some(Ordering::Equal)
                } else if r.start() > &val {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            Config::Min(m) => {
                if m > &val {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Equal)
                }
            }
            Config::Max(m) => {
                if m < &val {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            _ => unreachable!(),
        }
    }
}
