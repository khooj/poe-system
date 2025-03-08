use crate::types::{Mod, ModValue};
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum Config {
    Exact(i32),
    Range(RangeInclusive<i32>),
    Min(i32),
    Max(i32),
    Exist,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ModConfig {
    pub stat_id: String,
    pub configuration: Config,
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

        if matches!(other.numeric_value, ModValue::Nothing) {
            return false;
        }

        match (&self.configuration, &other.numeric_value) {
            (Config::Exact(v), ModValue::Exact(m)) => v.eq(&m.as_int()),
            (Config::Range(r), ModValue::Exact(m)) => r.contains(&m.as_int()),
            (Config::Min(m), ModValue::Exact(ref m2)) => *m <= m2.as_int(),
            (Config::Max(m), ModValue::Exact(ref m2)) => *m >= m2.as_int(),
            _ => unreachable!(),
        }
    }
}
