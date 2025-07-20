use std::{
    collections::HashMap,
    ops::{Deref, RangeInclusive},
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, TS, PartialEq, Eq, Hash)]
pub struct ModStatId(String);

impl Deref for ModStatId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&String> for ModStatId {
    fn from(value: &String) -> Self {
        ModStatId(value.clone())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, TS, PartialEq)]
pub struct Level(u32);

#[derive(Debug, Serialize, Deserialize, Clone, TS, PartialEq)]
pub struct Quality(u32);

#[derive(Debug, Serialize, Deserialize, Clone, TS, PartialEq, Default)]
pub struct ItemConfig {
    pub basetype: bool,
    pub option: Option<ItemConfigOption>,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS, PartialEq)]
pub enum ItemConfigOption {
    Mods(HashMap<ModStatId, ModOption>),
    Unique,
}

#[derive(Debug, Serialize, Deserialize, TS, PartialEq, Clone)]
#[ts(export)]
pub enum ModOption {
    Exact(i32),
    Range(RangeInclusive<i32>),
    Exist,
    Ignore,
}
