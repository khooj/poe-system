use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::TryFrom,
    ops::{Deref, RangeInclusive, Sub},
    str::FromStr,
};
use strum::{AsRefStr, EnumString};
use thiserror::Error;
use ts_rs::TS;

use crate::data::{BaseItems, ModValue as DataModValue, MODS};

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

#[derive(Deserialize, Serialize, Clone, Debug, EnumString, Default, PartialEq, AsRefStr, TS)]
#[strum(ascii_case_insensitive)]
#[ts(export)]
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

lazy_static::lazy_static! {
    static ref CATEGORY_MAPPING: HashMap<String, Category> = {
        let mut hm = HashMap::new();
        hm.insert("weapon".into(), Category::Weapons);
        hm.insert("armour".into(), Category::Armour);
        hm.insert("jewel".into(), Category::Jewels);
        hm.insert("abyss_jewel".into(), Category::Jewels);
        hm.insert("quiver".into(), Category::Weapons);
        hm.insert("ring".into(), Category::Accessories);
        hm.insert("amulet".into(), Category::Accessories);
        hm.insert("flask".into(), Category::Flasks);
        hm.insert("belt".into(), Category::Accessories);
        hm.insert("gem".into(), Category::Gems);
        hm
    };
}

impl Category {
    pub fn get_from_basetype<T: AsRef<str>>(basetype: T) -> Result<Category, TypeError> {
        let baseinfo = BaseItems::get_by_name(basetype.as_ref())
            .ok_or(TypeError::UnknownCategory(basetype.as_ref().to_string()))?;
        for (k, v) in CATEGORY_MAPPING.deref().iter() {
            if baseinfo.tags.contains(k) {
                return Ok(v.clone());
            }
        }
        Err(TypeError::UnknownCategory(basetype.as_ref().to_string()))
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
pub enum Jewels {
    SmallClusterJewel,
    MediumClusterJewel,
    LargeClusterJewel,
    ViridianJewel,
}

#[derive(Debug, EnumString, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum Weapons {
    OrnateQuiver,
    DeathBow,
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

lazy_static::lazy_static! {
    static ref SUBCATEGORY_MAPPING: HashMap<String, Subcategory> = {
        let mut hm = HashMap::new();
        hm.insert("Boots".to_string(), Subcategory::Boots);
        hm.insert("Helmet".to_string(), Subcategory::Helmets);
        hm.insert("AbyssJewel".to_string(), Subcategory::Jewel);
        hm.insert("Active Skill Gem".to_string(), Subcategory::Gem);
        hm.insert("Amulet".to_string(), Subcategory::Amulet);
        hm.insert("Belt".to_string(), Subcategory::Belt);
        hm.insert("Body Armour".to_string(), Subcategory::BodyArmour);
        hm.insert("Bow".to_string(), Subcategory::Weapon);
        hm.insert("Claw".to_string(), Subcategory::Weapon);
        hm.insert("Dagger".to_string(), Subcategory::Weapon);
        hm.insert("FishingRod".to_string(), Subcategory::Weapon);
        hm.insert("Gloves".to_string(), Subcategory::Gloves);
        hm.insert("HybridFlask".to_string(), Subcategory::HybridFlask);
        hm.insert("Jewel".to_string(), Subcategory::Jewel);
        hm.insert("LifeFlask".to_string(), Subcategory::LifeFlask);
        hm.insert("ManaFlask".to_string(), Subcategory::ManaFlask);
        hm.insert("One Hand Axe".to_string(), Subcategory::Weapon);
        hm.insert("One Hand Mace".to_string(), Subcategory::Weapon);
        hm.insert("One Hand Sword".to_string(), Subcategory::Weapon);
        hm.insert("Quiver".to_string(), Subcategory::Quiver);
        hm.insert("Ring".to_string(), Subcategory::Ring);
        hm.insert("Rune Dagger".to_string(), Subcategory::Weapon);
        hm.insert("Sceptre".to_string(), Subcategory::Weapon);
        hm.insert("Shield".to_string(), Subcategory::Shield);
        hm.insert("Staff".to_string(), Subcategory::Weapon);
        hm.insert("Support Skill Gem".to_string(), Subcategory::Gem);
        hm.insert("Thrusting One Hand Sword".to_string(), Subcategory::Weapon);
        hm.insert("Two Hand Axe".to_string(), Subcategory::Weapon);
        hm.insert("Two Hand Mace".to_string(), Subcategory::Weapon);
        hm.insert("Two Hand Sword".to_string(), Subcategory::Weapon);
        hm.insert("UtilityFlask".to_string(), Subcategory::UtilityFlask);
        hm.insert("Wand".to_string(), Subcategory::Weapon);
        hm.insert("Warstaff".to_string(), Subcategory::Weapon);
        hm
    };
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, EnumString, PartialEq, AsRefStr, TS)]
#[strum(ascii_case_insensitive)]
#[ts(export)]
pub enum Subcategory {
    #[default]
    Empty,
    Helmets,
    Boots,
    Gem,
    Jewel,
    Amulet,
    Belt,
    BodyArmour,
    Gloves,
    Weapon,
    LifeFlask,
    ManaFlask,
    HybridFlask,
    UtilityFlask,
    Shield,
    Quiver,
    Ring,
}

#[derive(Error, Debug)]
pub enum SubcategoryError {
    #[error("unknown subcategory: {0}")]
    UnknownSubcategory(String),
}

impl Subcategory {
    pub fn get_from_basetype<T: AsRef<str>>(basetype: T) -> Result<Subcategory, SubcategoryError> {
        let baseinfo = BaseItems::get_by_name(basetype.as_ref()).ok_or(
            SubcategoryError::UnknownSubcategory(basetype.as_ref().to_string()),
        )?;

        Ok(SUBCATEGORY_MAPPING
            .get(&baseinfo.item_class)
            .ok_or(SubcategoryError::UnknownSubcategory(
                basetype.as_ref().to_string(),
            ))?
            .clone())
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Copy, PartialEq, Default, TS)]
#[ts(export)]
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
    Invalid = 99,
}

#[derive(Error, Debug)]
pub enum ModError {
    #[error("can't find mod by stat: {0}")]
    StatError(String),
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Default, TS)]
#[ts(export)]
pub enum ModValue {
    #[default]
    Nothing,
    Exact(DataModValue),
    DoubleExact {
        from: DataModValue,
        to: DataModValue,
    },
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Default, TS)]
#[ts(export)]
pub struct Mod {
    pub text: String,
    pub type_: ModType,
    // pub stat_translation: String,
    pub stat_id: String,
    pub numeric_value: ModValue,
    #[serde(skip_serializing, skip_deserializing)]
    #[ts(skip)]
    _internal: crate::private::Private,
}

impl Mod {
    pub fn try_by_stat(value: &str, typ: ModType) -> Result<Self, ModError> {
        if let Some(ext) = MODS::get_mod_data(value) {
            return Ok(Mod {
                text: value.to_string(),
                type_: typ,
                stat_id: ext.mod_type().get_id(),
                numeric_value: match ext.extract_values(value) {
                    (Some(num), Some(num2)) => ModValue::DoubleExact {
                        from: num,
                        to: num2,
                    },
                    (Some(num), None) => ModValue::Exact(num),
                    _ => ModValue::Nothing,
                },
                ..Default::default()
            });
        }
        Err(ModError::StatError(value.to_string()))
    }

    pub fn try_by_range_stat(value: &str, range: f32, typ: ModType) -> Result<Self, ModError> {
        if let Some(ext) = MODS::get_mod_data(value) {
            return Ok(Mod {
                text: value.to_string(),
                type_: typ,
                stat_id: ext.mod_type().get_id(),
                numeric_value: match ext.extract_by_range(value, range) {
                    (Some(num), Some(num2)) => ModValue::DoubleExact {
                        from: num,
                        to: num2,
                    },
                    (Some(num), None) => ModValue::Exact(num),
                    _ => ModValue::Nothing,
                },
                ..Default::default()
            });
        }
        Err(ModError::StatError(value.to_string()))
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

#[derive(Deserialize, Serialize, Clone, Debug, Default, TS)]
#[ts(export)]
pub struct Property {
    pub name: String,
    pub value: Option<String>,
    pub augmented: bool,
}

#[cfg(test)]
mod tests {
    use super::{Mod, ModType, ModValue};

    #[test]
    fn mod_parse() {
        let mod1 = "75% increased Spell Damage";
        let mod2 = Mod::try_by_stat(mod1, ModType::Explicit).unwrap();
        assert_eq!(
            mod2,
            Mod {
                text: "75% increased Spell Damage".to_string(),
                type_: ModType::Explicit,
                numeric_value: ModValue::Exact(crate::data::ModValue::Int(75)),
                stat_id: "spell_damage_+%".to_string(),
                ..Default::default()
            }
        )
    }

    #[test]
    fn mod_parse_cluster() -> Result<(), anyhow::Error> {
        let _ = Mod::try_by_stat(
            "Added Small Passive Skills also grant: 3% increased Projectile Speed",
            ModType::Implicit,
        )?;
        Ok(())
    }
}
