use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, str::FromStr};
use strum::EnumString;
use thiserror::Error;

use crate::{MODS, STATS_CUTTED};

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("can't parse rarity: {0}")]
    RarityParse(String),
    #[error("unknown category: {0}")]
    UnknownCategory(String),
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

#[derive(Deserialize, Serialize, Clone, Debug, EnumString, Default, PartialEq)]
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

impl Category {
    pub fn parse_from_basetype<T: AsRef<str>>(basetype: T) -> Result<Category, TypeError> {
        let basetype = basetype.as_ref();
        if let Ok(_) = BootsBase::from_str(basetype) {
            return Ok(Category::Armour);
        }
        Err(TypeError::UnknownCategory(basetype.to_string()))
    }
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

#[derive(Debug, EnumString, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum BootsBase {
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

#[derive(Deserialize, Serialize, Clone, Debug, Default, EnumString, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum Subcategory {
    #[default]
    Empty,
    Helmets,
    Boots,
}

#[derive(Deserialize, Serialize, Clone, Debug, Copy, PartialEq, Default)]
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
    #[default]
    Invalid = 100,
}

#[derive(Error, Debug)]
pub enum ModError {
    #[error("can't find mod by stat: {0}")]
    StatError(String),
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Default)]
pub enum ModValue {
    #[default]
    Nothing,
    Exact(i32),
    MinMax(i32, i32),
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Default)]
pub struct Mod {
    pub text: String,
    pub type_: ModType,
    // pub stat_translation: String,
    pub stat_id: String,
    pub numeric_value: ModValue,
    #[serde(skip_serializing, skip_deserializing)]
    _internal: crate::private::Private,
}

impl Mod {
    pub fn by_stat(value: &str, typ: ModType) -> Result<Self, ModError> {
        let ret = Mod::by_stat_or_invalid(value, typ);
        if ret.type_ == ModType::Invalid {
            Err(ModError::StatError(value.to_string()))
        } else {
            Ok(ret)
        }
    }

    pub fn by_stat_or_invalid(value: &str, typ: ModType) -> Self {
        if let Some(mod_value) = MODS::get_mod_data(value) {
            return Mod {
                text: value.to_string(),
                type_: typ,
                stat_id: mod_value.id.clone(),
                numeric_value: match (mod_value.min, mod_value.max) {
                    (Some(m1), Some(m2)) if m1 == m2 => ModValue::Exact(m1),
                    (Some(m1), Some(m2)) => ModValue::MinMax(m1, m2),
                    _ => ModValue::Nothing,
                },
                ..Default::default()
            };
        }

        Mod::default()
    }

    pub fn many_by_stat_or_invalid(values: &[(&str, ModType)]) -> Vec<Self> {
        values
            .iter()
            .map(|(v, m)| Mod::by_stat_or_invalid(v, *m))
            .collect()
    }

    pub fn many_by_stat(values: &[(&str, ModType)]) -> Vec<Self> {
        Mod::many_by_stat_or_invalid(values)
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

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Property {
    pub name: String,
    pub value: Option<String>,
    pub augmented: bool,
}

#[cfg(test)]
mod tests {
    use crate::ModValue;

    use super::{Mod, ModType};

    #[test]
    fn mod_parse() {
        let mods1 = vec![("75% increased Spell Damage", ModType::Explicit)];
        let mods2 = Mod::many_by_stat_or_invalid(&mods1);
        assert_eq!(mods2.len(), 1);
        assert_eq!(
            mods2[0],
            Mod {
                text: "75% increased Spell Damage".to_string(),
                type_: ModType::Explicit,
                numeric_value: ModValue::MinMax(60, 80),
                stat_id: "spell_damage_+%".to_string(),
                ..Default::default()
            }
        )
    }

    #[test]
    fn mod_parse_cluster() -> Result<(), anyhow::Error> {
        let _ = Mod::by_stat(
            "Added Small Passive Skills also grant: 3% increased Projectile Speed",
            ModType::Implicit,
        )?;
        Ok(())
    }
}
