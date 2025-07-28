use std::{
    collections::HashMap,
    ops::{Deref, RangeInclusive},
};

use rustler::{types::atom, Encoder, NifStruct, NifTaggedEnum, Term};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, NifStruct)]
#[module = "PoeSystem.Items.ModStatId"]
pub struct ModStatId {
    value: String,
}

impl Deref for ModStatId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<&String> for ModStatId {
    fn from(value: &String) -> Self {
        ModStatId {
            value: value.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, TS, PartialEq)]
pub struct Level(u32);

#[derive(Debug, Serialize, Deserialize, Clone, TS, PartialEq)]
pub struct Quality(u32);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, NifStruct)]
#[module = "PoeSystem.Items.ItemConfig"]
pub struct ItemConfig {
    pub basetype: bool,
    pub option: Option<ItemConfigOption>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, NifTaggedEnum)]
pub enum ItemConfigOption {
    Mods(HashMap<ModStatId, ModOption>),
    Unique,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, NifStruct)]
#[module = "PoeSystem.Items.RangeInclusive"]
pub struct RangeInclusiveI32Elixir {
    pub start: i32,
    pub end: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, NifTaggedEnum)]
pub enum ModOption {
    Exact(i32),
    Range(RangeInclusiveI32Elixir),
    Exist,
    Ignore,
}
