use super::types::{Category, Class, Hybrid, Influence, ItemLvl, League, Mod, Subcategory};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::cmp::Eq;
use std::collections::HashMap;
use std::default::Default;
use std::ops::Deref;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Item {
    pub id: String,
    pub league: League,

    pub item_lvl: ItemLvl,
    pub identified: bool,
    pub name: String,
    pub category: Category,
    pub subcategories: Vec<Subcategory>,
    pub base_type: String,
    pub type_line: String,
    pub corrupted: bool,
    pub influences: Vec<Influence>,
    pub fractured: bool,
    pub synthesised: bool,
    pub mods: Vec<Mod>,
    pub hybrid: Hybrid,
    pub class: Class,
    pub image_link: String,
    pub rarity: String,
    pub lvl_req: i32,
    pub sockets: String,
    pub quality: i32,
}

#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct SimilarityScore(i64);

impl Deref for SimilarityScore {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Sockets {
    groups: Vec<SocketGroup>,
}

impl Sockets {
    pub fn max_links(&self) -> usize {
        self.groups
            .iter()
            .map(|s| s.sockets.len())
            .max()
            .unwrap_or_default()
    }

    pub fn colors(&self) -> HashMap<SocketColor, usize> {
        self.groups.iter().flat_map(|s| s.sockets.clone()).counts()
    }
}

pub struct SocketGroup {
    sockets: Vec<SocketColor>,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum SocketColor {
    R,
    G,
    B,
    W,
    NotSupported,
}

impl Item {
    pub fn sockets(&self) -> Sockets {
        let links: Vec<&str> = self.sockets.split(' ').collect();
        let l = links
            .into_iter()
            .map(|l| {
                let scks: Vec<&str> = l.split('-').collect();
                let s = scks
                    .into_iter()
                    .map(|s| match s {
                        "R" => SocketColor::R,
                        "G" => SocketColor::G,
                        "B" => SocketColor::B,
                        "W" => SocketColor::W,
                        _ => SocketColor::NotSupported,
                    })
                    .collect();
                SocketGroup { sockets: s }
            })
            .collect();
        Sockets { groups: l }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::ModType;
}

// Rarity: Unique
// Bones of Ullr
// Silk Slippers
// --------
// Energy Shield: 23 (augmented)
// --------
// Requirements:
// Level: 22
// Int: 42
// --------
// Sockets: B-B B
// --------
// Item Level: 33
// --------
// 51% increased Energy Shield
// +20 to maximum Life
// +20 to maximum Mana
// +1 to Level of all Raise Zombie Gems
// +1 to Level of all Raise Spectre Gems
// 12% increased Movement Speed
// --------
// The dead man walks where
// the living fear to tread.
// --------
// Note: ~price 1 chance

// Rarity: Gem
// Stormblast Mine
// --------
// Mine, Spell, AoE, Lightning, Aura, Nova
// Level: 12
// Mana Reserved: 4
// Cast Time: 0.75 sec
// Critical Strike Chance: 6.00%
// Effectiveness of Added Damage: 110%
// --------
// Requirements:
// Level: 40
// Dex: 40
// Int: 58
// --------
// Throws a mine that deals damage in an area when detonated.
// --------
// Deals 57 to 172 Lightning Damage
// Mine lasts 5 seconds
// Base Mine Detonation Time is 0.25 seconds
// +3 to radius
// 20% chance to Shock enemies
// 31% increased Effect of Lightning Ailments
// Each Mine applies 3% increased Damage Taken to Enemies near it, up
// to a maximum of 150%
// --------
// Experience: 1044491/1061223
// --------
// Place into an item socket of the right colour to gain this skill. Right click to remove from a socket.
// --------
// Note: ~price 1 alch

// Rarity: Rare
// Demon Strike
// Sniper Bow
// --------
// Bow
// Physical Damage: 62-162 (augmented)
// Elemental Damage: 1-7 (augmented)
// Critical Strike Chance: 6.70%
// Attacks per Second: 1.25
// --------
// Requirements:
// Level: 44
// Dex: 143
// --------
// Sockets: G G-R-G-B-B
// --------
// Item Level: 50
// --------
// +18% to Global Critical Strike Multiplier (implicit)
// --------
// 78% increased Physical Damage
// Adds 12 to 23 Physical Damage
// Adds 1 to 7 Lightning Damage
// +14% to Global Critical Strike Multiplier
// Attacks have 25% chance to cause Bleeding
// 33% increased Damage with Bleeding
// 20% chance to Poison on Hit
// 25% increased Damage with Poison
// --------
// Corrupted
// --------
// Note: ~price 1 alch
