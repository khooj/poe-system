use anyhow::anyhow;
use std::ops::Deref;
use std::{convert::TryFrom, default::Default};

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum Rarity {
    Normal,
    Magic,
    Rare,
    Unique,
}

impl Default for Rarity {
    fn default() -> Self {
        Rarity::Normal
    }
}

impl TryFrom<String> for Rarity {
    type Error = anyhow::Error;

    fn try_from(v: String) -> Result<Rarity, Self::Error> {
        match v.to_lowercase().as_str() {
            "magic" => Ok(Rarity::Normal),
            "rare" => Ok(Rarity::Rare),
            "unique" => Ok(Rarity::Unique),
            "normal" => Ok(Rarity::Normal),
            _ => Err(anyhow!("cant convert from {} to rarity enum", v)),
        }
    }
}

#[allow(unused, non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum __category_tmp {
    Flasks,
    Jewellery,
    OneHandedWeapon,
    TwoHandedWeapon,
    Gems,
    Offhand,
    Armor,
    Other,
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum Category {
    Accessories,
    Armour,
    Jewels,
    Weapons,
}

impl Default for Category {
    fn default() -> Self {
        Category::Accessories
    }
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum Class {
    LifeFlask,
    ManaFlask,
    HybridFlask,
    Currency,
    Amulet,
    Ring,
    Claws,
    Dagger,
    Wand,
    OneHandSword,
    ThrustingOneHandSword,
    OneHandAxe,
    OneHandMace,
    Bow,
    Staff,
    TwoHandSword,
    TwoHandAxe,
    TwoHandMace,
    ActiveSkillGem,
    SupportSkillGem,
    Quiver,
    Belt,
    Glove,
    Boot,
    BodyArmour,
    Helmet,
    Shield,
    StackableCurrency,
    Scepter,
    UtilityFlask,
    CriticalUtilityFlask,
    Map,
    FishingRod,
    MapFragment,
    Jewel,
    DivinationCard,
    LabyrinthItem,
    LabyrinthTrinket,
    LabyrinthMapItem,
    MiscMapItem,
    Leaguestones,
    PantheonSoul,
    Piece,
    AbyssJewel,
    IncursionItem,
    DelveSocketableCurrency,
    Incubator,
    Shard,
    ShardHeart,
    RuneDagger,
    Warstaff,
    DelveStackableSocketableCurrency,
    AtlasRegionUpgradeItem,
    MetamorphSample,
    HarvestSeed,
    SeedEnhancer,
    Contract,
    HeistGear,
    HeistTool,
    HeistCloak,
    HeistBrooch,
    Blueprint,
    Trinket,
    HeistTarget,
}

impl Default for Class {
    fn default() -> Self {
        Class::LifeFlask
    }
}

#[allow(unused)]
enum BootsBase {
    // str
    IronGreaves,
    SteelGreaves,
    BasemetalTreads,
    PlatedGreaves,
    ReinforcedGreaves,
    AntiqueGreaves,
    AncientGreaves,
    DarksteelTreads,
    GoliathGreaves,
    VaalGreaves,
    TitanGreaves,
    BrimstoneGreaves,
    // dex
    RawhideBoots,
    GoathideBoots,
    CloudwhisperBoots,
    DeerskinBoots,
    NubuckBoots,
    EelskinBoots,
    SharkskinBoots,
    WindbreakBoots,
    ShagreenBoots,
    StealthBoots,
    SlinkBoots,
    StormriderBoots,
    // int
    WoolShoes,
    VelvetSlippers,
    DuskwalkSlippers,
    SilkSlippers,
    ScholarBoots,
    SatinSlippers,
    SamiteSlippers,
    NightwindSlippers,
    ConjurerBoots,
    ArcanistSlippers,
    SorcererBoots,
    DreamquestSlippers,
}

#[allow(unused)]
enum BodyArmour {
    // str
    PlateVest,
    Chestplate,
    CopperPlate,
    WarPlate,
    FullPlate,
    ArenaPlate,
    LordlyPlate,
    BronzePlate,
    BattlePlate,
    SunPlate,
    ColosseumPlate,
    MajesticPlace,
    GoldenPlate,
    CrusaderPlate,
    AstralPlate,
    GladiatorPlate,
    GloriousPlate,

    // dex
    ShabbyJerkin,
    StrappedLeather,
    BuckskinTunic,
    WildLeather,
    FullLeather,
    SunLeather,
    ThiefsGarb,
    EelskinTunic,
    FrontierLeather,
    GloriousLeather,
    CoronalLeather,
    CutthroatsGarb,
    SharkskinTunic,
    DestinyLeather,
    ExquisiteLeather,
    ZodiacLeather,
    AssassinsGarb,

    // int
    SimpleRobe,
    SilkenVest,
    ScholarsRobe,
    SilkenGarb,
    MageVestmest,
    SilkRobe,
    CabalistRegalia,
    SagesRobe,
    SilkenWrap,
    ConjurerVestment,
    SpidersilkRobbe,
    DestroyerRegalia,
    SavantsRobe,
    NecromancerSilks,
    OccultistsVestment,
    WidowsilkRobe,
    VaalRegalia,

    // str/dex
    ScaleVest,
    LightBrigandine,
    ScaleDoublet,
    InfantryBrigandine,
    FullScaleArmour,
    SoldiersBrigandine,
    FieldLamellar,
    WyrmscaleDoublet,
    HussarBrigandine,
    FullWyrmscale,
    CommandersBrigandine,
    BattleLamellar,
    DragonscaleDoublet,
    DesertBrigandine,
    FullDragonscale,
    GeneralsBrigandine,
    TriumphantLamellar,

    // str/int
    ChainmailVest,
    ChainmailTunic,
    RingmailCoat,
    ChainmailDoublet,
    FullRingmail,
    FullChainmail,
    HolyChainmail,
    LatticedRingmail,
    CrusaderChainmail,
    OrnateRingmail,
    ChainHauberk,
    DevoutChainmail,
    LoricatedRingmail,
    ConquestChainmail,
    ElegantRingmail,
    SaintsHauberk,
    SaintlyChainmail,

    // int/dex
    PaddedVest,
    OiledVest,
    PaddedJacket,
    OiledCoat,
    ScarletRaiment,
    WaxedGarb,
    BoneArmour,
    QuiltedJacket,
    SleekCoat,
    CrimsonRaiment,
    LacqueredGarb,
    CryptArmour,
    SentinetJacket,
    VarnishedCoat,
    BloodRaiment,
    SadistGarb,
    CarnalArmour,

    // str/dex/int
    SacrificialGarb,
}

#[derive(Clone, Debug)]
pub enum League {
    Standard,
    Hardcore,
    TempStandard,
    TempHardcore,
}

impl Default for League {
    fn default() -> Self {
        League::Standard
    }
}

#[derive(Clone, Debug)]
pub enum ItemLvl {
    No,
    Yes(i32),
}

impl From<i32> for ItemLvl {
    fn from(value: i32) -> Self {
        ItemLvl::Yes(value)
    }
}

impl Default for ItemLvl {
    fn default() -> Self {
        ItemLvl::No
    }
}

#[derive(Clone, Debug)]
pub enum Influence {
    Shaper,
    Elder,
    Warlord,
    Hunter,
    Redeemer,
    Crusader,
}

#[derive(Clone, Debug)]
pub enum Subcategory {
    Smth(String),
}

impl Default for Subcategory {
    fn default() -> Self {
        Subcategory::Smth("".into())
    }
}

#[derive(Clone, Debug)]
pub enum ModType {
    Utility = 0,
    Implicit = 1,
    Explicit = 2,
    Crafted = 3,
    Enchant = 4,
    Fractured = 5,
    Cosmetic = 6,
    Veiled = 7,
    ExplicitHybrid = 8,
}

#[derive(Clone, Debug)]
pub struct Mod {
    pub text: String,
    pub type_: ModType,
}

impl Mod {
    pub fn from_str(value: &str, type_: ModType) -> Self {
        Mod {
            text: value.to_owned(),
            type_,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Hybrid {
    pub is_vaal_gem: bool,
    pub base_type_name: String,
    pub sec_descr_text: Option<String>,
}

impl Default for Hybrid {
    fn default() -> Self {
        Hybrid {
            is_vaal_gem: false,
            base_type_name: String::new(),
            sec_descr_text: None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Item {
    pub id: String,
    pub league: League,

    pub item_lvl: ItemLvl,
    pub identified: bool,
    pub rarity: Rarity,
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
}

pub struct SimilarityScore(i64);

impl Deref for SimilarityScore {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Item {
    pub fn calculate_similarity_score(&self, item: &Item) -> SimilarityScore {
        use itertools::Itertools;
        use std::collections::HashMap;
        use strsim::levenshtein;

        let mods_scores = self
            .mods
            .iter()
            .cartesian_product(item.mods.iter())
            .group_by(|(el, _)| el.text.clone())
            .into_iter()
            .map(|(k, grp)| {
                (
                    k,
                    grp.map(|(o, d)| levenshtein(&d.text, &o.text) as i64)
                        .min()
                        .unwrap_or(0i64),
                )
            })
            .collect::<HashMap<String, i64>>();

        let score = mods_scores.values().sum();

        SimilarityScore(score)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_similarity() -> anyhow::Result<()> {
        let item1 = Item {
            mods: vec![
                Mod {
                    text: "51% increased Energy Shield".to_owned(),
                    type_: ModType::Explicit,
                },
                Mod {
                    text: "+20 to maximum Life".to_owned(),
                    type_: ModType::Explicit,
                },
            ],
            ..Item::default()
        };

        let score = item1.calculate_similarity_score(&item1);
        assert_eq!(*score, 0);

        Ok(())
    }
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
