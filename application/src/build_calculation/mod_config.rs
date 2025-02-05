use domain::types::{Mod, ModValue};
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

#[derive(Debug, Serialize, Deserialize)]
pub enum Config {
    Exact(i32),
    Range(RangeInclusive<i32>),
    Min(i32),
    Max(i32),
    Exist,
}

#[derive(Debug, Serialize, Deserialize)]
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
            (Config::Exact(v), ModValue::Exact(m)) => v.eq(m),
            (Config::Range(r), ModValue::Exact(m)) => r.contains(m),
            (Config::Min(m), ModValue::Exact(ref m2)) => m <= m2,
            (Config::Max(m), ModValue::Exact(ref m2)) => m >= m2,
            _ => unreachable!(),
        }
    }
}
