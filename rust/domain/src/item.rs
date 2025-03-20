pub mod types;

use serde::{Deserialize, Serialize};
use std::default::Default;
use types::{Category, Hybrid, Influence, League, Mod, Property, Rarity, Sockets, Subcategory};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Item {
    pub id: String,
    pub league: League,

    pub rarity: Rarity,
    pub item_lvl: Option<i32>,
    pub identified: bool,
    pub name: String,
    pub category: Category,
    pub subcategories: Subcategory,
    pub base_type: String,
    pub type_line: String,
    pub corrupted: bool,
    pub influences: Vec<Influence>,
    pub fractured: bool,
    pub synthesised: bool,
    pub mods: Vec<Mod>,
    pub image_link: String,
    pub sockets: Sockets,
    pub properties: Vec<Property>,
}

impl Item {}

#[cfg(test)]
mod test {}

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
