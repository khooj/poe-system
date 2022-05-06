use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use strum::EnumString;

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

impl Class {
    pub fn from_itemclass(itemclass: &str) -> Result<Class> {
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
            x => League::Private(t),
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

#[derive(Deserialize, Serialize, Clone, Debug)]
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

#[derive(Deserialize, Serialize, Clone, Debug, Copy)]
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
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Mod {
    pub text: String,
    pub type_: ModType,
}

impl Mod {
    pub fn from_str_type(value: &str, type_: ModType) -> Self {
        Mod {
            text: value.to_owned(),
            type_,
        }
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
