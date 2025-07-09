use super::item::Item;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Default, TS)]
#[ts(export)]
pub struct BuildItems {
    pub helmet: Option<Item>,
    pub body: Option<Item>,
    pub boots: Option<Item>,
    pub gloves: Option<Item>,
    pub weapon1: Option<Item>,
    pub weapon2: Option<Item>,
    pub ring1: Option<Item>,
    pub ring2: Option<Item>,
    pub belt: Option<Item>,
    pub flasks: Option<Vec<Item>>,
    pub gems: Option<Vec<Item>>,
    pub jewels: Option<Vec<Item>>,
    pub amulet: Option<Item>,
}
