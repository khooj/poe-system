use crate::domain::item::Item as DomainItem;
use crate::ports::outbound::public_stash_retriever::{Item, ItemProperty, PublicStashData};
use crate::ports::outbound::repository::{LatestStashId, RepositoryError};
use diesel::BelongingToDsl;
use diesel::Queryable;
use diesel::{backend::UsesAnsiSavepointSyntax, sqlite::Sqlite, connection::AnsiTransactionManager, prelude::*};
use itertools::Itertools;
use log::warn;
use serde_json::json;
use std::{
    collections::HashMap,
    convert::{From, TryFrom},
};
use uuid::Uuid;

struct SplittedItem {
    item: NewItem,
    mods: Option<Vec<NewMod>>,
    subcategories: Option<Vec<NewSubcategory>>,
    props: Option<Vec<NewProperty>>,
    sockets: Option<Vec<NewSocket>>,
    ultimatum: Option<Vec<NewUltimatumMod>>,
    incubated: Option<NewIncubatedItem>,
    hybrid: Option<NewHybrid>,
    extended: Option<NewExtended>,
    influence: Option<NewInfluence>,
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

        let mods = append_mods(
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

        let props = append_properties(
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

        let hybrid = if let Some(el) = item.hybrid {
            Some(NewHybrid {
                id: Some(Uuid::new_v4().to_hyphenated().to_string()),
                item_id: item.id.as_ref().unwrap().clone(),
                base_type_name: el.base_type_name,
                is_vaal_gem: el.is_vaal_gem,
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
                crusader: el.crusader,
                hunter: el.hunter,
                redeemer: el.redeemer,
                warlord: el.warlord,
                shaper: el.shaper,
                elder: el.elder,
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
    to_insert: Vec<(Option<Vec<ItemProperty>>, PropertyType)>,
    item_id: &str,
) -> Option<Vec<NewProperty>> {
    let mut vals = None;
    for (ins, t) in to_insert {
        // debug!("prop: {:?}", ins);
        vals = append_if_not_empty2(
            vals,
            ins.map_or(vec![], |v| v)
                .into_iter()
                .map(|el| NewProperty {
                    id: Uuid::new_v4().to_hyphenated().to_string(),
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

fn _append_if_not_empty<T, K>(
    mut vals: Option<Vec<T>>,
    to_insert: Option<Vec<K>>,
) -> Result<Option<Vec<T>>, RepositoryError>
where
    K: std::convert::TryInto<T>,
    RepositoryError: From<<K as std::convert::TryInto<T>>::Error>,
{
    if to_insert.is_none() || to_insert.as_ref().unwrap().len() == 0 {
        return Ok(vals);
    }

    let mut ins: Vec<T> = vec![];
    if to_insert.is_some() {
        for el in to_insert.unwrap() {
            let el_ = el.try_into()?;
            ins.push(el_);
        }
    }

    if ins.len() > 0 && vals.is_none() {
        vals = Some(vec![]);
    }

    vals.as_mut().unwrap().append(&mut ins);

    Ok(vals)
}

#[allow(dead_code)]
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

// pub struct Mod {
//     item_id: String,
//     r#type: ModType,
//     r#mod: String,
// }

// pub struct Subcategory {
//     item_id: String,
//     subcategory: String,
// }

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
/*

pub struct Property {
    item_id: String,
    property_type: PropertyType,
    name: String,
    value_type: ValueType,
    value: i32,
    r#type: Option<i32>,
    progress: Option<f64>,
    suffix: Option<String>,
}

pub struct Socket {
    item_id: String,
    s_group: i32,
    attr: Option<String>,
    s_colour: Option<String>,
}

pub struct UltimatumMod {
    item_id: String,
    r#type: String,
    tier: i32,
}

pub struct IncubatedItem {
    item_id: String,
    name: String,
    level: i32,
    progress: i32,
    total: i32,
}

pub struct Hybrid {
    id: Option<String>,
    item_id: String,
    is_vaal_gem: Option<bool>,
    base_type_name: String,
    sec_descr_text: Option<String>,
}
*/

use crate::schema::{
    extended, hybrids, incubated_item, influences, items, latest_stash_id, mods, properties,
    socketed_items, sockets, subcategories, ultimatum_mods,
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
struct NewMod {
    id: String,
    item_id: String,
    type_: i32,
    mod_: String,
}

#[derive(Insertable)]
#[table_name = "subcategories"]
struct NewSubcategory {
    id: String,
    item_id: String,
    subcategory: String,
}

#[derive(Insertable)]
#[table_name = "properties"]
struct NewProperty {
    id: String,
    item_id: String,
    property_type: i32,
    name: String,
    value_type: i32,
    value: String,
    type_: Option<i32>,
    progress: Option<f32>,
    suffix: Option<String>,
}

#[derive(Insertable)]
#[table_name = "socketed_items"]
struct NewSocketedItem {
    item_id: String,
    socketed_item_id: String,
}

#[derive(Insertable)]
#[table_name = "sockets"]
struct NewSocket {
    id: String,
    item_id: String,
    s_group: i32,
    attr: Option<String>,
    s_colour: Option<String>,
}

#[derive(Insertable)]
#[table_name = "ultimatum_mods"]
struct NewUltimatumMod {
    item_id: String,
    type_: String,
    tier: i32,
}

#[derive(Insertable)]
#[table_name = "incubated_item"]
struct NewIncubatedItem {
    item_id: String,
    name: String,
    level: i32,
    progress: i32,
    total: i32,
}

#[derive(Insertable)]
#[table_name = "hybrids"]
struct NewHybrid {
    id: Option<String>,
    item_id: String,
    is_vaal_gem: Option<bool>,
    base_type_name: String,
    sec_descr_text: Option<String>,
}

#[derive(Insertable)]
#[table_name = "extended"]
struct NewExtended {
    item_id: String,
    category: String,
    prefixes: Option<i32>,
    suffixes: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "influences"]
struct NewInfluence {
    item_id: String,
    warlord: Option<bool>,
    crusader: Option<bool>,
    redeemer: Option<bool>,
    hunter: Option<bool>,
    shaper: Option<bool>,
    elder: Option<bool>,
}

#[derive(Insertable)]
#[table_name = "latest_stash_id"]
struct NewLatestStash {
    id: String,
}

#[derive(Identifiable)]
#[table_name = "items"]
#[primary_key(account_name, stash_id)]
struct RemoveItems<'a> {
    account_name: &'a String,
    stash_id: &'a String,
}

#[derive(Identifiable, Queryable, Associations, Debug, Default, Clone)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "influences"]
#[primary_key(item_id)]
struct Influence {
    item_id: String,
    warlord: Option<bool>,
    crusader: Option<bool>,
    redeemer: Option<bool>,
    hunter: Option<bool>,
    shaper: Option<bool>,
    elder: Option<bool>,
}

#[derive(Identifiable, Queryable, Associations, Debug, Default, Clone)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "extended"]
#[primary_key(item_id)]
struct Extended {
    item_id: String,
    category: String,
    prefixes: Option<i32>,
    suffixes: Option<i32>,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "hybrids"]
#[primary_key(id, item_id)]
struct Hybrid {
    id: String,
    item_id: String,
    is_vaal_gem: Option<bool>,
    base_type_name: String,
    sec_descr_text: Option<String>,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "incubated_item"]
#[primary_key(item_id, name)]
struct IncubatedItem {
    item_id: String,
    name: String,
    level: i32,
    progress: i32,
    total: i32,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "ultimatum_mods"]
#[primary_key(item_id, type_)]
struct UltimatumMod {
    item_id: String,
    type_: String,
    tier: i32,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "sockets"]
struct Socket {
    id: String,
    item_id: String,
    s_group: i32,
    attr: Option<String>,
    s_colour: Option<String>,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "socketed_items"]
#[primary_key(item_id, socketed_item_id)]
struct SocketedItem {
    item_id: String,
    socketed_item_id: String,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "properties"]
struct Property {
    id: String,
    item_id: String,
    property_type: i32,
    name: String,
    value_type: i32,
    value: String,
    type_: Option<i32>,
    progress: Option<f32>,
    suffix: Option<String>,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "subcategories"]
struct Subcategory {
    id: String,
    item_id: String,
    subcategory: String,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(RawItem, foreign_key = "item_id")]
#[table_name = "mods"]
struct Mod {
    id: String,
    item_id: String,
    type_: i32,
    mod_: String,
}

type DomainItemFrom = (
    RawItem,
    Vec<Influence>,
    Vec<Mod>,
    Vec<Extended>,
    Vec<Hybrid>,
    Vec<IncubatedItem>,
    Vec<UltimatumMod>,
    Vec<Socket>,
    Vec<SocketedItem>,
    Vec<Property>,
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
            if opt.is_some() {
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

impl From<DomainItemFrom> for DomainItem {
    fn from(val: DomainItemFrom) -> Self {
        let extended = val.3.first().map_or(Extended::default(), |e| (*e).clone());
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
            mods: val.2.into_iter().map(|e| e.into()).collect(),
            ..Default::default()
        }
    }
}

macro_rules! collect_val {
    ($v:expr, $field:tt) => {
        $v.iter()
            .map(|el| &el.$field)
            .filter_map(|el| el.as_ref())
            .flatten()
    };
}

macro_rules! collect_val2 {
    ($v:expr, $field:tt) => {
        $v.iter().map(|el| &el.$field).filter_map(|el| el.as_ref())
    };
}

#[derive(Clone)]
pub struct DieselItemRepository<T>
where
    T: Connection + Send + 'static,
{
    conn: T,
}

use crate::schema::{items::dsl as items_dsl, latest_stash_id::dsl as stash_dsl};

impl<T> DieselItemRepository<T>
where
    // TODO: investigate why compiler complains about `Backend = Sqlite` requirement
    T: Connection<TransactionManager = AnsiTransactionManager, Backend = Sqlite> + Send + 'static,
    T::Backend: UsesAnsiSavepointSyntax,
{
    pub fn new(conn: T) -> Result<DieselItemRepository<T>, RepositoryError> {
        Ok(DieselItemRepository { conn })
    }

    pub fn get_items_by_basetype(
        &self,
        base_type: &str,
    ) -> Result<Vec<DomainItem>, RepositoryError> {
        use itertools::izip;

        let items = items_dsl::items
            .filter(items_dsl::base_type.eq(base_type))
            .load::<RawItem>(&self.conn)?;

        let influences = Influence::belonging_to(&items)
            .load::<Influence>(&self.conn)?
            .grouped_by(&items);
        let mods = Mod::belonging_to(&items)
            .load::<Mod>(&self.conn)?
            .grouped_by(&items);
        let extended = Extended::belonging_to(&items)
            .load::<Extended>(&self.conn)?
            .grouped_by(&items);
        let hybrid = Hybrid::belonging_to(&items)
            .load::<Hybrid>(&self.conn)?
            .grouped_by(&items);
        let incubated = IncubatedItem::belonging_to(&items)
            .load::<IncubatedItem>(&self.conn)?
            .grouped_by(&items);
        let ultimatum = UltimatumMod::belonging_to(&items)
            .load::<UltimatumMod>(&self.conn)?
            .grouped_by(&items);
        let socket = Socket::belonging_to(&items)
            .load::<Socket>(&self.conn)?
            .grouped_by(&items);
        let socketed = SocketedItem::belonging_to(&items)
            .load::<SocketedItem>(&self.conn)?
            .grouped_by(&items);
        let properties = Property::belonging_to(&items)
            .load::<Property>(&self.conn)?
            .grouped_by(&items);
        let subcategories = Subcategory::belonging_to(&items)
            .load::<Subcategory>(&self.conn)?
            .grouped_by(&items);

        let data = izip!(
            items,
            influences,
            mods,
            extended,
            hybrid,
            incubated,
            ultimatum,
            socket,
            socketed,
            properties,
            subcategories
        )
        .map(|v| DomainItem::from(v))
        .collect::<Vec<_>>();

        Ok(data)
    }

    fn get_raw_items(
        &self,
        account_name: &str,
        stash_id: &str,
    ) -> Result<Vec<RawItem>, RepositoryError> {
        Ok(items_dsl::items
            .filter(
                items_dsl::account_name
                    .eq(account_name)
                    .and(items_dsl::stash_id.eq(stash_id)),
            )
            .load::<RawItem>(&self.conn)?)
    }

    pub fn get_stash_id(&self) -> Result<LatestStashId, RepositoryError> {
        let v = stash_dsl::latest_stash_id.load::<LatestStashId>(&self.conn)?;
        Ok(v.into_iter()
            .nth(0)
            .or(Some(LatestStashId::default()))
            .unwrap())
    }

    pub fn set_stash_id(&self, id: &str) -> Result<(), RepositoryError> {
        // workaround for upsert functionality for sqlite https://github.com/diesel-rs/diesel/issues/1854
        let vals = stash_dsl::latest_stash_id.load::<LatestStashId>(&self.conn)?;
        if vals.len() == 0 {
            let latest_stash = NewLatestStash { id: id.to_owned() };
            diesel::insert_into(latest_stash_id::table)
                .values(&latest_stash)
                .execute(&self.conn)?;
        } else {
            diesel::update(latest_stash_id::table)
                .set(stash_dsl::id.eq(id))
                .execute(&self.conn)?;
        }
        Ok(())
    }

    pub fn insert_raw_item(&self, public_data: &PublicStashData) -> Result<(), RepositoryError> {
        use itertools::izip;

        self.conn.transaction::<_, RepositoryError, _>(|| {
            let new_item_info: HashMap<String, Vec<SplittedItem>> = public_data
                .stashes
                .iter()
                .map(|v| {
                    v.items
                        .iter()
                        .map(|i| match SplittedItem::try_from(i.clone()) {
                            Ok(mut item) => {
                                item.item.account_id = v.id.clone();
                                item.item.account_name = v.account_name.as_ref().cloned().unwrap();
                                item.item.stash_id = v.stash.as_ref().cloned().unwrap();
                                Some(item)
                            }
                            Err(_) => {
                                warn!("skipping {:?} item because cant generate entity", i);
                                None
                            }
                        })
                        .filter_map(|i| i)
                        .collect::<Vec<SplittedItem>>()
                })
                .flatten()
                .into_group_map_by(|el| el.item.account_id.clone());

            self.set_stash_id(&public_data.next_change_id)?;

            for (k, v) in &new_item_info {
                if v.len() == 0 {
                    diesel::delete(items_dsl::items.filter(items_dsl::account_id.eq(&k)))
                        .execute(&self.conn)?;
                } else {
                    let insert_items: Vec<&NewItem> = v.iter().map(|v| &v.item).collect();
                    let delete_items: Vec<RemoveItems> = v
                        .iter()
                        .map(|v| RemoveItems {
                            account_name: &v.item.account_name,
                            stash_id: &v.item.stash_id,
                        })
                        .collect();

                    // TODO: somehow use Identifiable or smth else to simplify delete query
                    for i in &delete_items {
                        diesel::delete(
                            items_dsl::items.filter(
                                items_dsl::account_name
                                    .eq(i.account_name)
                                    .and(items_dsl::stash_id.eq(i.stash_id)),
                            ),
                        )
                        .execute(&self.conn)?;
                    }

                    for i in insert_items {
                        diesel::insert_into(items::table)
                            .values(i)
                            .execute(&self.conn)?;
                    }

                    for mods in izip!(
                        collect_val!(v, mods),
                        collect_val!(v, subcategories),
                        collect_val!(v, props),
                        collect_val!(v, sockets),
                        collect_val!(v, ultimatum),
                    ) {
                        diesel::insert_into(mods::table)
                            .values(mods.0)
                            .execute(&self.conn)?;

                        diesel::insert_into(subcategories::table)
                            .values(mods.1)
                            .execute(&self.conn)?;

                        diesel::insert_into(properties::table)
                            .values(mods.2)
                            .execute(&self.conn)?;

                        diesel::insert_into(sockets::table)
                            .values(mods.3)
                            .execute(&self.conn)?;

                        diesel::insert_into(ultimatum_mods::table)
                            .values(mods.4)
                            .execute(&self.conn)?;
                    }

                    for mods in izip!(
                        collect_val2!(v, incubated),
                        collect_val2!(v, hybrid),
                        collect_val2!(v, extended),
                        collect_val2!(v, influence),
                    ) {
                        diesel::insert_into(incubated_item::table)
                            .values(mods.0)
                            .execute(&self.conn)?;

                        diesel::insert_into(hybrids::table)
                            .values(mods.1)
                            .execute(&self.conn)?;

                        diesel::insert_into(extended::table)
                            .values(mods.2)
                            .execute(&self.conn)?;

                        diesel::insert_into(influences::table)
                            .values(mods.3)
                            .execute(&self.conn)?;
                    }
                }
            }

            Ok(())
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::DieselItemRepository;
    use crate::ports::outbound::public_stash_retriever::PublicStashData;
    use diesel::prelude::*;

    const PUBLIC_STASH_DATA: &str = include_str!("public-stash-tabs.json");

    #[test]
    fn insert_item() -> Result<(), anyhow::Error> {
        let conn = SqliteConnection::establish(":memory:")?;

        let repo = DieselItemRepository::new(conn)?;
        let stash: PublicStashData = serde_json::from_str(&PUBLIC_STASH_DATA)?;

        let _ = repo.insert_raw_item(&stash)?;

        let latest_stash_id = repo.get_stash_id()?;
        assert_eq!(
            latest_stash_id.latest_stash_id.unwrap(),
            "2949-5227-4536-5447-1849"
        );
        Ok(())
    }

    #[test]
    fn get_items() -> Result<(), anyhow::Error> {
        let conn = SqliteConnection::establish(":memory:")?;

        let repo = DieselItemRepository::new(conn)?;
        let stash: PublicStashData = serde_json::from_str(&PUBLIC_STASH_DATA)?;

        let _ = repo.insert_raw_item(&stash)?;
        let items = repo.get_items_by_basetype("Recurve Bow")?;

        for i in items {
            println!("{:?}", i);
        }

        Ok(())
    }

    #[test]
    fn insert_remove_stash() -> Result<(), anyhow::Error> {
        let conn = SqliteConnection::establish(":memory:")?;

        let repo = DieselItemRepository::new(conn)?;
        let mut stash: PublicStashData = serde_json::from_str(&PUBLIC_STASH_DATA)?;

        let _ = repo.insert_raw_item(&stash.clone())?;

        stash.stashes = vec![stash
            .stashes
            .into_iter()
            .filter(|v| v.account_name.is_some())
            .nth(0)
            .unwrap()];
        stash.stashes.get_mut(0).unwrap().items.truncate(3);

        let _ = repo.insert_raw_item(&stash)?;

        let items = repo.get_raw_items(
            stash.stashes[0].account_name.as_ref().unwrap(),
            stash.stashes[0].stash.as_ref().unwrap(),
        )?;
        assert_eq!(items.len(), 3);
        Ok(())
    }
}
