use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use strum::EnumString;
use thiserror::Error;

use crate::STATS_CUTTED;

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("can't parse rarity: {0}")]
    RarityParse(String),
}

#[allow(non_camel_case_types)]
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

#[derive(Deserialize, Serialize, Clone, Debug, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Category {
    Flasks,
    Accessories,
    Armour,
    Jewels,
    Weapons,
    Maps,
    Currency,
    Logbook,
    Heistmission,
    Heistequipment,
    Cards,
    Monsters,
    Gems,
    Leaguestones,
}

impl Default for Category {
    fn default() -> Self {
        Category::Accessories
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, EnumString)]
#[strum(ascii_case_insensitive)]
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
    Gloves,
    Boots,
    BodyArmour,
    Helmet,
    Shield,
    StackableCurrency,
    Sceptre,
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
    // TODO: do i need this?
    SmallClusterJewel,
    MediumClusterJewel,
    LargeClusterJewel,
}

#[derive(Error, Debug)]
pub enum ClassError {
    #[error("parse error: {0}")]
    ParseError(#[from] strum::ParseError),
}

impl Default for Class {
    fn default() -> Self {
        Class::LifeFlask
    }
}

impl Class {
    pub fn from_itemclass(itemclass: &str) -> Result<Class, ClassError> {
        use std::str::FromStr;

        let mut s = itemclass.to_string();
        s.retain(|c| !c.is_whitespace());
        Ok(Class::from_str(&s)?)
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum League {
    Standard,
    SSFStandard,
    Hardcore,
    SSFHardcore,
    TempStandard,
    TempHardcore,
    Private(String),
}

impl Default for League {
    fn default() -> Self {
        League::Standard
    }
}

impl From<String> for League {
    fn from(t: String) -> League {
        match t.as_ref() {
            "Hardcore" => League::Hardcore,
            "Standard" => League::Standard,
            "SSF Hardcore" => League::SSFHardcore,
            "SSF Standard" => League::SSFStandard,
            x if !x.contains("(PL") => {
                if x.contains("HC") {
                    League::TempHardcore
                } else {
                    League::TempStandard
                }
            }
            _ => League::Private(t),
        }
    }
}

impl From<Option<String>> for League {
    fn from(t: Option<String>) -> League {
        match t {
            Some(k) => League::from(k),
            None => League::Standard,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum ItemLvl {
    No,
    Yes(i32),
}

impl From<i32> for ItemLvl {
    fn from(value: i32) -> Self {
        ItemLvl::Yes(value)
    }
}

impl From<Option<i32>> for ItemLvl {
    fn from(v: Option<i32>) -> Self {
        match v {
            Some(k) => ItemLvl::Yes(k),
            None => ItemLvl::No,
        }
    }
}

impl Default for ItemLvl {
    fn default() -> Self {
        ItemLvl::No
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Influence {
    Shaper,
    Elder,
    Warlord,
    Hunter,
    Redeemer,
    Crusader,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Subcategory {
    Empty,
}

impl Default for Subcategory {
    fn default() -> Self {
        Subcategory::Empty
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Copy, PartialEq)]
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
    Scourge = 9,
    Invalid = 100,
}

#[derive(Error, Debug)]
pub enum ModError {
    #[error("can't find mod by stat: {0}")]
    StatError(String),
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Mod {
    pub text: String,
    pub type_: ModType,
    pub stat_translation: String,
    pub stat_id: String,
    #[serde(skip_serializing, skip_deserializing)]
    _internal: crate::private::Private,
}

impl Mod {
    pub fn by_stat(value: &str, typ: ModType) -> Result<Self, ModError> {
        let v = crate::cut_numbers(&value);
        if let Some(idx) = STATS_CUTTED.get(&v) {
            return Ok(Mod {
                stat_translation: STATS_CUTTED::get_original_stat(*idx),
                type_: typ,
                text: value.to_string(),
                stat_id: STATS_CUTTED::get_stat_id(*idx),
                ..Default::default()
            });
        }

        return Err(ModError::StatError(value.to_string()));
    }

    pub fn by_stat_or_invalid(value: &str, typ: ModType) -> Self {
        Mod::by_stat(value, typ).unwrap_or_default()
    }

    pub fn invalid() -> Self {
        Mod {
            stat_translation: String::new(),
            type_: ModType::Invalid,
            text: String::new(),
            stat_id: String::new(),
            _internal: crate::private::Private,
        }
    }

    pub fn many_by_stat_or_invalid(values: &[&(&str, ModType)]) -> Vec<Self> {
        let values2 = values
            .iter()
            .enumerate()
            .map(|(idx, e)| (idx, crate::cut_numbers(e.0), e.1))
            .collect::<Vec<(usize, String, ModType)>>();

        let mut result = vec![Mod::invalid(); values.len()];
        for val in values2 {
            let stat_idx = if let Some(val) = STATS_CUTTED.get(&val.1) {
                *val
            } else {
                continue;
            };
            result[val.0] = Mod {
                stat_translation: STATS_CUTTED::get_original_stat(stat_idx),
                type_: val.2,
                text: values[val.0].0.to_string(),
                stat_id: STATS_CUTTED::get_stat_id(stat_idx),
                ..Default::default()
            }
        }
        result
    }
}

impl Default for Mod {
    fn default() -> Self {
        Mod::invalid()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
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

#[allow(unused)]
#[derive(Deserialize, Serialize, Clone, Debug)]
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
    type Error = TypeError;

    fn try_from(v: String) -> Result<Rarity, Self::Error> {
        match v.to_lowercase().as_str() {
            "magic" => Ok(Rarity::Normal),
            "rare" => Ok(Rarity::Rare),
            "unique" => Ok(Rarity::Unique),
            "normal" => Ok(Rarity::Normal),
            _ => Err(TypeError::RarityParse(v)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Mod, ModType};

    #[test]
    fn mod_parse() {
        let mods1 = vec![&("75% increased Spell Damage", ModType::Explicit)];
        let mods2 = Mod::many_by_stat_or_invalid(&mods1);
        assert_eq!(mods2.len(), 1);
        assert_eq!(
            mods2[0],
            Mod {
                stat_translation: "{0}% increased Spell Damage".to_string(),
                text: "75% increased Spell Damage".to_string(),
                type_: ModType::Explicit,
                ..Default::default()
            }
        )
    }
}
