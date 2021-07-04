use crate::domain::item::Item as DomainItem;
use crate::ports::outbound::public_stash_retriever::{Item, ItemProperty as ItemPropertyJson};
use crate::ports::outbound::repository::RepositoryError;
use crate::schema::{build_info, builds_match};
use serde_json::json;
use std::convert::{From, TryFrom};
use tracing::{event, Level};
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "build_info"]
pub struct NewBuild<'a> {
    pub id: String,
    pub pob_url: &'a str,
    pub itemset: &'a str,
}

#[derive(Queryable, Identifiable, AsChangeset, Clone, Debug)]
#[table_name = "build_info"]
pub struct PobBuild {
    pub id: String,
    pub pob_url: String,
    pub itemset: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "builds_match"]
pub struct NewBuildMatch {
    pub id: String,
    pub idx: i32,
    pub score: i32,
    pub item_id: String,
}

pub struct SplittedItem {
    pub item: NewItem,
    pub mods: Option<Vec<NewMod>>,
    pub subcategories: Option<Vec<NewSubcategory>>,
    pub props: Option<Vec<Property>>,
    pub sockets: Option<Vec<NewSocket>>,
    pub ultimatum: Option<Vec<NewUltimatumMod>>,
    pub incubated: Option<NewIncubatedItem>,
    pub hybrid: Option<HybridMod>,
    pub extended: Option<NewExtended>,
    pub influence: Option<NewInfluence>,
}

impl TryFrom<Item> for SplittedItem {
    type Error = RepositoryError;
    fn try_from(mut item: Item) -> Result<Self, Self::Error> {
        if item.id.is_none() {
            return Err(RepositoryError::Skipped);
        }

        let raw = NewItem {
            account_id: String::new(),
            account_name: String::new(),
            stash_id: String::new(),
            verified: item.verified,
            w: item.w,
            h: item.h,
            icon: item.icon.clone(),
            support: item.support,
            stack_size: item.stack_size,
            max_stack_size: item.max_stack_size,
            league: item.league.clone(),
            id: item.id.as_ref().unwrap().clone(),
            elder: item.elder,
            shaper: item.shaper,
            abyss_jewel: item.abyss_jewel,
            delve: item.delve,
            fractured: item.fractured,
            synthesised: item.synthesised,
            name: item.name.clone(),
            type_line: item.type_line.clone(),
            base_type: item.base_type.clone(),
            identified: item.identified,
            item_lvl: item.item_level,
            note: item.note.take(),
            duplicated: item.duplicated,
            split: item.split,
            corrupted: item.corrupted,
            talisman_tier: item.talisman_tier,
            sec_descr_text: item.sec_descr_text.take(),
            veiled: item.veiled,
            descr_text: item.descr_text.take(),
            prophecy_text: item.prophecy_text.take(),
            is_relic: item.is_relic,
            replica: item.replica,
            frame_type: item.frame_type,
            x_coordinate: item.x,
            y_coordinate: item.y,
            inventory_id: item.inventory_id.take(),
            socket: item.socket,
            colour: item.colour.take(),
        };

        let mut mods = append_mods(
            vec![
                (item.utility_mods.take(), ModType::Utility),
                (item.implicit_mods.take(), ModType::Implicit),
                (item.explicit_mods.take(), ModType::Explicit),
                (item.crafted_mods.take(), ModType::Crafted),
                (item.enchant_mods.take(), ModType::Enchant),
                (item.fractured_mods.take(), ModType::Fractured),
                (item.cosmetic_mods.take(), ModType::Cosmetic),
                (item.veiled_mods.take(), ModType::Veiled),
            ],
            item.id.as_deref().unwrap(),
        );
        let subcategories: Vec<NewSubcategory> = item
            .extended
            .take()
            .and_then(|el| el.subcategories)
            .map_or(vec![], |v| v)
            .into_iter()
            .map(|el| NewSubcategory {
                id: Uuid::new_v4().to_hyphenated().to_string(),
                item_id: item.id.as_ref().unwrap().clone(),
                subcategory: el,
            })
            .collect();
        let subcategories = if subcategories.len() > 0 {
            Some(subcategories)
        } else {
            None
        };

        let mut props = append_properties(
            vec![
                (item.properties.take(), PropertyType::Properties),
                (
                    item.notable_properties.take(),
                    PropertyType::NotableProperties,
                ),
                (item.requirements.take(), PropertyType::Requirements),
                (
                    item.additional_properties.take(),
                    PropertyType::AdditionalProperties,
                ),
                (
                    item.next_item_requirements.take(),
                    PropertyType::NextLevelRequirements,
                ),
            ],
            item.id.as_deref().unwrap(),
        );

        let sockets: Vec<NewSocket> = item
            .sockets
            .take()
            .map_or(vec![], |v| v)
            .into_iter()
            .map(|el| NewSocket {
                id: Uuid::new_v4().to_hyphenated().to_string(),
                item_id: item.id.as_ref().unwrap().clone(),
                attr: el.attr,
                s_colour: el.s_colour,
                s_group: el.group,
            })
            .collect();
        let sockets = if sockets.len() > 0 {
            Some(sockets)
        } else {
            None
        };

        let ultimatum: Vec<NewUltimatumMod> = item
            .ultimatum_mods
            .take()
            .map_or(vec![], |v| v)
            .into_iter()
            .map(|el| NewUltimatumMod {
                item_id: item.id.as_ref().unwrap().clone(),
                tier: el.tier,
                type_: el.mod_type,
            })
            .collect();
        let ultimatum = if ultimatum.len() > 0 {
            Some(ultimatum)
        } else {
            None
        };

        let incubated = if let Some(el) = item.incubated_item {
            Some(NewIncubatedItem {
                item_id: item.id.as_ref().unwrap().clone(),
                level: el.level,
                name: el.name,
                progress: el.progress,
                total: el.total,
            })
        } else {
            None
        };

        let hybrid = if let Some(mut el) = item.hybrid {
            event!(Level::DEBUG, "hybrid: {:?}", el);
            let mut hybrid_mods = append_mods(
                vec![(el.explicit_mods.take(), ModType::ExplicitHybrid)],
                item.id.as_deref().unwrap(),
            );
            if mods.is_none() {
                mods = hybrid_mods;
            } else {
                if hybrid_mods.is_some() {
                    mods.as_mut().unwrap().append(hybrid_mods.as_mut().unwrap());
                }
            }

            let mut hybrid_props = append_properties(
                vec![(el.properties.take(), PropertyType::Hybrid)],
                item.id.as_deref().unwrap(),
            );

            if props.is_none() {
                props = hybrid_props;
            } else {
                if hybrid_props.is_some() {
                    props
                        .as_mut()
                        .unwrap()
                        .append(hybrid_props.as_mut().unwrap());
                }
            }

            Some(HybridMod {
                item_id: item.id.as_ref().unwrap().clone(),
                base_type_name: el.base_type_name,
                is_vaal_gem: el.is_vaal_gem.unwrap_or(false),
                sec_descr_text: el.sec_descr_text,
            })
        } else {
            None
        };

        let extended = if let Some(el) = item.extended {
            Some(NewExtended {
                item_id: item.id.as_ref().unwrap().clone(),
                category: el.category,
                prefixes: el.prefixes,
                suffixes: el.suffixes,
            })
        } else {
            None
        };

        let influence = if let Some(el) = item.influences {
            Some(NewInfluence {
                item_id: item.id.as_ref().unwrap().clone(),
                crusader: el.crusader.unwrap_or(false),
                hunter: el.hunter.unwrap_or(false),
                redeemer: el.redeemer.unwrap_or(false),
                warlord: el.warlord.unwrap_or(false),
                shaper: el.shaper.unwrap_or(false),
                elder: el.elder.unwrap_or(false),
            })
        } else {
            None
        };

        Ok(SplittedItem {
            item: raw,
            mods,
            subcategories,
            props,
            sockets,
            ultimatum,
            incubated,
            hybrid,
            extended,
            influence,
        })
    }
}

fn append_properties(
    to_insert: Vec<(Option<Vec<ItemPropertyJson>>, PropertyType)>,
    item_id: &str,
) -> Option<Vec<Property>> {
    let mut vals = None;
    for (ins, t) in to_insert {
        // debug!("prop: {:?}", ins);
        vals = append_if_not_empty2(
            vals,
            ins.map_or(vec![], |v| v)
                .into_iter()
                .map(|el| Property {
                    item_id: String::from(item_id),
                    name: el.name,
                    progress: el.progress.map_or(None, |v| Some(v as f32)),
                    property_type: t as i32,
                    suffix: el.suffix,
                    type_: el.item_type,
                    value: el.values.get(0).map_or(&vec![json!("")], |el| el)[0]
                        .as_str()
                        .unwrap()
                        .to_owned(),
                    value_type: el.values.get(0).map_or(&vec![json!(""), json!(0)], |el| el)[1]
                        .as_i64()
                        .unwrap() as i32,
                })
                .collect(),
        );
    }
    vals
}

fn append_mods(
    to_insert: Vec<(Option<Vec<String>>, ModType)>,
    item_id: &str,
) -> Option<Vec<NewMod>> {
    let mut vals = None;
    for (ins, t) in to_insert {
        vals = append_mod(vals, ins, item_id, t);
    }
    vals
}

fn append_mod(
    vals: Option<Vec<NewMod>>,
    to_insert: Option<Vec<String>>,
    item_id: &str,
    type_: ModType,
) -> Option<Vec<NewMod>> {
    append_if_not_empty2(
        vals,
        to_insert
            .map_or(vec![], |v| v)
            .into_iter()
            .map(|el| NewMod {
                id: Uuid::new_v4().to_hyphenated().to_string(),
                item_id: String::from(item_id),
                type_: type_ as i32,
                mod_: el,
            })
            .collect(),
    )
}

fn append_if_not_empty2<T>(mut vals: Option<Vec<T>>, mut to_insert: Vec<T>) -> Option<Vec<T>> {
    if to_insert.len() == 0 {
        return vals;
    }

    if to_insert.len() > 0 && vals.is_none() {
        vals = Some(vec![]);
    }

    vals.as_mut().unwrap().append(&mut to_insert);

    vals
}

#[derive(Queryable, Identifiable, Debug)]
#[table_name = "items"]
pub struct RawItem {
    id: String,
    base_type: String,
    account_id: String,
    account_name: String,
    stash_id: String,
    league: Option<String>,
    name: String,
    item_lvl: Option<i32>,
    identified: bool,
    inventory_id: Option<String>,
    type_line: String,
    abyss_jewel: Option<bool>,
    corrupted: Option<bool>,
    duplicated: Option<bool>,
    elder: Option<bool>,
    frame_type: Option<i32>,
    h: i32,
    w: i32,
    x_coordinate: Option<i32>,
    y_coordinaate: Option<i32>,
    is_relic: Option<bool>,
    note: Option<String>,
    shaper: Option<bool>,
    stack_size: Option<i32>,
    max_stack_size: Option<i32>,
    support: Option<bool>,
    talisman_tier: Option<i32>,
    verified: bool,
    icon: String,
    delve: Option<bool>,
    fractured: Option<bool>,
    synthesised: Option<bool>,
    split: Option<bool>,
    sec_descr_text: Option<String>,
    veiled: Option<bool>,
    descr_text: Option<String>,
    prophecy_text: Option<String>,
    replica: Option<bool>,
    socket: Option<i32>,
    colour: Option<String>,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub enum PropertyType {
    Properties = 0,
    Requirements,
    AdditionalProperties,
    NextLevelRequirements,
    NotableProperties,
    Hybrid,
}

pub enum ValueType {
    WhitePhysical = 0,
    BlueModified = 1,
    Fire = 4,
    Cold = 5,
    Lightning = 6,
    Chaos = 7,
}

use crate::schema::{
    extended, hybrid_mods, hybrids, incubated_item, influences, items, latest_stash_id, mods,
    properties, property_types, socketed_items, sockets, subcategories, ultimatum_mods,
};

#[derive(Insertable, Debug)]
#[table_name = "items"]
pub struct NewItem {
    pub id: String,
    pub base_type: String,
    pub account_id: String,
    pub account_name: String,
    pub stash_id: String,
    pub league: Option<String>,
    pub name: String,
    pub item_lvl: Option<i32>,
    pub identified: bool,
    pub inventory_id: Option<String>,
    pub type_line: String,
    pub abyss_jewel: Option<bool>,
    pub corrupted: Option<bool>,
    pub duplicated: Option<bool>,
    pub elder: Option<bool>,
    pub frame_type: Option<i32>,
    pub h: i32,
    pub w: i32,
    pub x_coordinate: Option<i32>,
    pub y_coordinate: Option<i32>,
    pub is_relic: Option<bool>,
    pub note: Option<String>,
    pub shaper: Option<bool>,
    pub stack_size: Option<i32>,
    pub max_stack_size: Option<i32>,
    pub support: Option<bool>,
    pub talisman_tier: Option<i32>,
    pub verified: bool,
    pub icon: String,
    pub delve: Option<bool>,
    pub fractured: Option<bool>,
    pub synthesised: Option<bool>,
    pub split: Option<bool>,
    pub sec_descr_text: Option<String>,
    pub veiled: Option<bool>,
    pub descr_text: Option<String>,
    pub prophecy_text: Option<String>,
    pub replica: Option<bool>,
    pub socket: Option<i32>,
    pub colour: Option<String>,
}

#[derive(Insertable)]
#[table_name = "mods"]
pub struct NewMod {
    pub id: String,
    pub item_id: String,
    pub type_: i32,
    pub mod_: String,
}

#[derive(Insertable)]
#[table_name = "subcategories"]
pub struct NewSubcategory {
    pub id: String,
    pub item_id: String,
    pub subcategory: String,
}

#[derive(Clone, Debug)]
pub struct Property {
    pub item_id: String,
    pub property_type: i32,
    pub name: String,
    pub value_type: i32,
    pub value: String,
    pub type_: Option<i32>,
    pub progress: Option<f32>,
    pub suffix: Option<String>,
}

#[derive(Insertable)]
#[table_name = "property_types"]
pub struct NewPropertyType {
    pub id: String,
    pub property_type: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "properties"]
pub struct NewProperty {
    pub property_id: String,
    pub item_id: String,
    pub value_type: i32,
    pub value: String,
    pub type_: Option<i32>,
    pub progress: Option<f32>,
    pub suffix: Option<String>,
}

#[derive(Insertable)]
#[table_name = "socketed_items"]
pub struct NewSocketedItem {
    pub item_id: String,
    pub socketed_item_id: String,
}

#[derive(Insertable)]
#[table_name = "sockets"]
pub struct NewSocket {
    pub id: String,
    pub item_id: String,
    pub s_group: i32,
    pub attr: Option<String>,
    pub s_colour: Option<String>,
}

#[derive(Insertable)]
#[table_name = "ultimatum_mods"]
pub struct NewUltimatumMod {
    pub item_id: String,
    pub type_: String,
    pub tier: i32,
}

#[derive(Insertable)]
#[table_name = "incubated_item"]
pub struct NewIncubatedItem {
    pub item_id: String,
    pub name: String,
    pub level: i32,
    pub progress: i32,
    pub total: i32,
}

#[derive(Debug)]
pub struct HybridMod {
    pub item_id: String,
    pub is_vaal_gem: bool,
    pub base_type_name: String,
    pub sec_descr_text: Option<String>,
}

#[derive(Insertable)]
#[table_name = "hybrids"]
pub struct NewHybrid {
    pub hybrid_id: String,
    pub item_id: String,
}

#[derive(Insertable)]
#[table_name = "hybrid_mods"]
pub struct NewHybridMod {
    pub id: String,
    pub is_vaal_gem: bool,
    pub base_type_name: String,
    pub sec_descr_text: Option<String>,
}

#[derive(Insertable)]
#[table_name = "extended"]
pub struct NewExtended {
    pub item_id: String,
    pub category: String,
    pub prefixes: Option<i32>,
    pub suffixes: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "influences"]
pub struct NewInfluence {
    pub item_id: String,
    pub warlord: bool,
    pub crusader: bool,
    pub redeemer: bool,
    pub hunter: bool,
    pub shaper: bool,
    pub elder: bool,
}

#[derive(Insertable)]
#[table_name = "latest_stash_id"]
pub struct NewLatestStash {
    pub id: String,
}

#[derive(Identifiable)]
#[table_name = "items"]
#[primary_key(account_name, stash_id)]
pub struct RemoveItems<'a> {
    pub account_name: &'a String,
    pub stash_id: &'a String,
}

#[derive(Identifiable, Queryable, Associations, Debug, Default, Clone)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "influences"]
#[primary_key(item_id)]
pub struct Influence {
    pub item_id: String,
    pub warlord: bool,
    pub crusader: bool,
    pub redeemer: bool,
    pub hunter: bool,
    pub shaper: bool,
    pub elder: bool,
}

#[derive(Identifiable, Queryable, Associations, Debug, Default, Clone)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "extended"]
#[primary_key(item_id)]
pub struct Extended {
    pub item_id: String,
    pub category: String,
    pub prefixes: Option<i32>,
    pub suffixes: Option<i32>,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "hybrids"]
#[primary_key(hybrid_id, item_id)]
pub struct Hybrid {
    pub hybrid_id: String,
    pub item_id: String,
}

#[derive(Identifiable, Queryable, Debug, Clone)]
#[table_name = "hybrid_mods"]
pub struct HybridModDb {
    pub id: String,
    pub is_vaal_gem: bool,
    pub base_type_name: String,
    pub sec_descr_text: Option<String>,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "incubated_item"]
#[primary_key(item_id, name)]
pub struct IncubatedItem {
    pub item_id: String,
    pub name: String,
    pub level: i32,
    pub progress: i32,
    pub total: i32,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "ultimatum_mods"]
#[primary_key(item_id, type_)]
pub struct UltimatumMod {
    pub item_id: String,
    pub type_: String,
    pub tier: i32,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "sockets"]
pub struct Socket {
    pub id: String,
    pub item_id: String,
    pub s_group: i32,
    pub attr: Option<String>,
    pub s_colour: Option<String>,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "socketed_items"]
#[primary_key(item_id, socketed_item_id)]
pub struct SocketedItem {
    pub item_id: String,
    pub socketed_item_id: String,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[belongs_to(PropertyTypeDb, foreign_key = "property_id")]
#[table_name = "properties"]
#[primary_key(property_id, item_id)]
pub struct ItemProperty {
    pub property_id: String,
    pub item_id: String,
    pub value_type: i32,
    pub value: String,
    pub type_: Option<i32>,
    pub progress: Option<f32>,
    pub suffix: Option<String>,
}

#[derive(Identifiable, Queryable, Debug, Clone)]
#[table_name = "property_types"]
pub struct PropertyTypeDb {
    pub id: String,
    pub property_type: i32,
    pub name: String,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "subcategories"]
pub struct Subcategory {
    pub id: String,
    pub item_id: String,
    pub subcategory: String,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "mods"]
pub struct Mod {
    pub id: String,
    pub item_id: String,
    pub type_: i32,
    pub mod_: String,
}

type DomainItemFrom = (
    RawItem,
    Vec<Influence>,
    Vec<Mod>,
    Vec<Extended>,
    Option<HybridModDb>,
    Vec<IncubatedItem>,
    Vec<UltimatumMod>,
    Vec<Socket>,
    Vec<SocketedItem>,
    Vec<PropertyTypeDb>,
    Vec<Subcategory>,
);

use crate::domain::item as domain_item;

impl Into<domain_item::League> for Option<String> {
    fn into(self) -> domain_item::League {
        use domain_item::League;
        if let Some(l) = self {
            match l.as_ref() {
                "Standard" => League::Standard,
                "Hardcore" => League::Hardcore,
                "Ultimatum" => League::TempStandard,
                "HC Ultimatum" => League::TempHardcore,
                _ => League::Standard,
            }
        } else {
            League::Standard
        }
    }
}

impl Into<domain_item::ItemLvl> for Option<i32> {
    fn into(self) -> domain_item::ItemLvl {
        if let Some(i) = self {
            domain_item::ItemLvl::Yes(i)
        } else {
            domain_item::ItemLvl::No
        }
    }
}

impl Into<domain_item::Rarity> for Option<i32> {
    fn into(self) -> domain_item::Rarity {
        use domain_item::Rarity;
        if let Some(i) = self {
            match i {
                0 => Rarity::Normal,
                1 => Rarity::Magic,
                2 => Rarity::Rare,
                3 => Rarity::Unique,
                _ => Rarity::Normal,
            }
        } else {
            Rarity::Normal
        }
    }
}

impl Into<domain_item::Category> for String {
    fn into(self) -> domain_item::Category {
        use domain_item::Category;

        match self.as_ref() {
            "accessories" => Category::Accessories,
            "armour" => Category::Armour,
            "jewels" => Category::Jewels,
            "weapons" => Category::Weapons,
            _ => Category::Accessories,
        }
    }
}

impl Into<domain_item::Subcategory> for Subcategory {
    fn into(self) -> domain_item::Subcategory {
        domain_item::Subcategory::Smth(self.subcategory)
    }
}

impl Into<Vec<domain_item::Influence>> for Influence {
    fn into(self) -> Vec<domain_item::Influence> {
        use domain_item::Influence as Inf;
        let mut v = vec![];
        let mapping = vec![
            (Inf::Hunter, self.hunter),
            (Inf::Shaper, self.shaper),
            (Inf::Elder, self.elder),
            (Inf::Warlord, self.warlord),
            (Inf::Crusader, self.crusader),
            (Inf::Redeemer, self.redeemer),
        ];
        for (type_, opt) in mapping {
            if opt {
                v.push(type_);
            }
        }

        v
    }
}

impl Into<domain_item::Mod> for Mod {
    fn into(self) -> domain_item::Mod {
        use domain_item::ModType;

        domain_item::Mod {
            text: self.mod_,
            type_: match self.type_ {
                1 => ModType::Implicit,
                2 => ModType::Explicit,
                3 => ModType::Crafted,
                4 => ModType::Enchant,
                5 => ModType::Fractured,
                6 => ModType::Cosmetic,
                7 => ModType::Veiled,
                8 => ModType::ExplicitHybrid,
                _ => ModType::Utility,
            },
        }
    }
}

impl Into<domain_item::Hybrid> for HybridModDb {
    fn into(self) -> domain_item::Hybrid {
        domain_item::Hybrid {
            is_vaal_gem: self.is_vaal_gem,
            base_type_name: self.base_type_name,
            sec_descr_text: self.sec_descr_text,
        }
    }
}

impl From<DomainItemFrom> for DomainItem {
    fn from(val: DomainItemFrom) -> Self {
        let extended = val.3.first().map_or(Extended::default(), |e| (*e).clone());
        let mods: Vec<domain_item::Mod> = val.2.into_iter().map(|e| e.into()).collect();

        DomainItem {
            id: val.0.id,
            league: val.0.league.into(),
            item_lvl: val.0.item_lvl.into(),
            identified: val.0.identified,
            rarity: val.0.frame_type.into(),
            name: val.0.name,
            category: extended.category.into(),
            subcategories: val.10.into_iter().map(|e| e.into()).collect(),
            base_type: val.0.base_type,
            type_line: val.0.type_line,
            corrupted: val.0.corrupted.unwrap_or(false),
            influences: val
                .1
                .first()
                .map_or(Influence::default(), |e| (*e).clone())
                .into(),
            fractured: val.0.fractured.unwrap_or(false),
            synthesised: val.0.synthesised.unwrap_or(false),
            mods,
            hybrid: if let Some(k) = val.4 {
                k.into()
            } else {
                domain_item::Hybrid::default()
            },
            ..Default::default()
        }
    }
}
