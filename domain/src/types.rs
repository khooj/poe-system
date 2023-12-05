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

#[derive(Deserialize, Serialize, Clone, Debug, EnumString, Default)]
#[strum(ascii_case_insensitive)]
pub enum Category {
    Flasks,
    #[default]
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

#[derive(Deserialize, Serialize, Clone, Debug, EnumString, Default)]
#[strum(ascii_case_insensitive)]
pub enum Class {
    #[default]
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

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub enum League {
    #[default]
    Standard,
    SSFStandard,
    Hardcore,
    SSFHardcore,
    TempStandard,
    TempHardcore,
    Private(String),
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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Default)]
pub enum ItemLvl {
    #[default]
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Influence {
    Shaper,
    Elder,
    Warlord,
    Hunter,
    Redeemer,
    Crusader,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub enum Subcategory {
    #[default]
    Empty,
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
    pub numeric_value: Option<i32>,
    #[serde(skip_serializing, skip_deserializing)]
    _internal: crate::private::Private,
}

impl Mod {
    pub fn by_stat(value: &str, typ: ModType) -> Result<Self, ModError> {
        let v = crate::cut_numbers(value);
        if let Some(idx) = STATS_CUTTED.get(&v) {
            let text = value.to_string();
            let stat_translation = STATS_CUTTED::get_original_stat(*idx);
            let numeric_value = Self::cut_numeric_values(&text, &stat_translation);
            return Ok(Mod {
                text,
                stat_translation,
                numeric_value,
                type_: typ,
                stat_id: STATS_CUTTED::get_stat_id(*idx),
                ..Default::default()
            });
        }

        Err(ModError::StatError(value.to_string()))
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
            numeric_value: None,
            _internal: crate::private::Private,
        }
    }

    fn cut_numeric_values(s: &str, template: &str) -> Option<i32> {
        let idx = template.find('{')?;

        let mut still_numeric = true;
        let num = s.chars().skip(idx).fold(String::new(), |mut acc, x| {
            if x.is_numeric() && still_numeric {
                acc.push(x);
            } else {
                still_numeric = false;
            }
            acc
        });

        num.parse().ok()
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
            let stat_translation = STATS_CUTTED::get_original_stat(stat_idx);
            let text = values[val.0].0.to_string();
            let numeric_value = Self::cut_numeric_values(&text, &stat_translation);
            result[val.0] = Mod {
                stat_translation,
                type_: val.2,
                text,
                stat_id: STATS_CUTTED::get_stat_id(stat_idx),
                numeric_value,
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

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Hybrid {
    pub is_vaal_gem: bool,
    pub base_type_name: String,
    pub sec_descr_text: Option<String>,
}

#[allow(unused)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub enum Rarity {
    #[default]
    Normal,
    Magic,
    Rare,
    Unique,
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
                numeric_value: Some(75),
                stat_id: "spell_damage_+%".to_string(),
                ..Default::default()
            }
        )
    }
}
