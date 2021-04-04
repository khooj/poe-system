enum Rarity {
    Normal,
    Magic,
    Rare,
    Unique,
}

enum Category {
    Flasks,
    Jewellery,
    OneHandedWeapon,
    TwoHandedWeapon,
    Gems,
    Offhand,
    Armor,
    Other,
}

enum Class {
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

enum Base {
    // Boots
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

struct Item {
    rarity: Rarity,
    name: String,
    category: Category,
    class: Class,
    base_type: Base,
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
