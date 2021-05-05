use crate::ports::outbound::public_stash_retriever::{
    Extended, Item, ItemProperty, ItemSocket, PublicStashData,
};
use crate::ports::outbound::repository::{ItemRepository, LatestStashId, RepositoryError};
use crate::{domain::item::Item as DomainItem, ports::outbound::repository};
use diesel::{
    backend::UsesAnsiSavepointSyntax,
    connection::{AnsiTransactionManager, TransactionManager},
    prelude::*,
    sqlite::Sqlite,
};
use diesel::{
    backend::{Backend, SupportsDefaultKeyword},
    Queryable,
};
use diesel::{query_dsl::LoadQuery, sqlite::SqliteConnection};
use dotenv::dotenv;
use itertools::Itertools;
use log::{debug, info, warn};
use serde_json::json;
use std::convert::TryInto;
use std::env;
use std::ops::Deref;
use std::str::FromStr;
use std::{collections::HashMap, convert::TryFrom};
use thiserror::Error;
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
    mut to_insert: Vec<(Option<Vec<ItemProperty>>, PropertyType)>,
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
    mut vals: Option<Vec<NewMod>>,
    mut to_insert: Option<Vec<String>>,
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

fn append_if_not_empty<T, K>(
    mut vals: Option<Vec<T>>,
    mut to_insert: Option<Vec<K>>,
) -> Result<Option<Vec<T>>, RepositoryError>
where
    K: TryInto<T>,
    RepositoryError: From<<K as TryInto<T>>::Error>,
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

#[derive(Queryable)]
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

pub struct Mod {
    item_id: String,
    r#type: ModType,
    r#mod: String,
}

pub struct Subcategory {
    item_id: String,
    subcategory: String,
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

pub struct DieselItemRepository {
    conn: SqliteConnection,
}

use crate::schema::{items::dsl as items_dsl, latest_stash_id::dsl as stash_dsl};

impl DieselItemRepository {
    pub fn new(connection: SqliteConnection) -> Result<DieselItemRepository, RepositoryError> {
        Ok(DieselItemRepository { conn: connection })
    }

    fn get_items(
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
            let latest_stash = NewLatestStash {
                id: id.to_owned(),
            };
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

    pub fn insert_raw_item(&self, public_data: PublicStashData) -> Result<(), RepositoryError> {
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

                    diesel::insert_into(items::table)
                        .values(insert_items)
                        .execute(&self.conn)?;

                    // TODO: write func to insert these values?
                    let mods = v
                        .iter()
                        .map(|el| &el.mods)
                        .filter_map(|el| el.as_ref())
                        .flatten()
                        .collect::<Vec<&NewMod>>();
                    diesel::insert_into(mods::table)
                        .values(mods)
                        .execute(&self.conn)?;

                    let subcategories = v
                        .iter()
                        .map(|el| &el.subcategories)
                        .filter_map(|el| el.as_deref())
                        .flatten()
                        .collect::<Vec<&NewSubcategory>>();
                    diesel::insert_into(subcategories::table)
                        .values(subcategories)
                        .execute(&self.conn)?;

                    let props = v
                        .iter()
                        .map(|el| &el.props)
                        .filter_map(|el| el.as_deref())
                        .flatten()
                        .collect::<Vec<&NewProperty>>();
                    diesel::insert_into(properties::table)
                        .values(props)
                        .execute(&self.conn)?;

                    let sockets = v
                        .iter()
                        .map(|el| &el.sockets)
                        .filter_map(|el| el.as_deref())
                        .flatten()
                        .collect::<Vec<&NewSocket>>();
                    diesel::insert_into(sockets::table)
                        .values(sockets)
                        .execute(&self.conn)?;

                    let ultimatum = v
                        .iter()
                        .map(|el| &el.ultimatum)
                        .filter_map(|el| el.as_deref())
                        .flatten()
                        .collect::<Vec<&NewUltimatumMod>>();
                    diesel::insert_into(ultimatum_mods::table)
                        .values(ultimatum)
                        .execute(&self.conn)?;

                    let incubated = v
                        .iter()
                        .map(|el| &el.incubated)
                        .filter_map(|el| el.as_ref())
                        .collect::<Vec<&NewIncubatedItem>>();
                    diesel::insert_into(incubated_item::table)
                        .values(incubated)
                        .execute(&self.conn)?;

                    let hybrid = v
                        .iter()
                        .map(|el| &el.hybrid)
                        .filter_map(|el| el.as_ref())
                        .collect::<Vec<&NewHybrid>>();
                    diesel::insert_into(hybrids::table)
                        .values(hybrid)
                        .execute(&self.conn)?;

                    let extended = v
                        .iter()
                        .map(|el| &el.extended)
                        .filter_map(|el| el.as_ref())
                        .collect::<Vec<&NewExtended>>();
                    diesel::insert_into(extended::table)
                        .values(extended)
                        .execute(&self.conn)?;

                    let influences = v
                        .iter()
                        .map(|el| &el.influence)
                        .filter_map(|el| el.as_ref())
                        .collect::<Vec<&NewInfluence>>();
                    diesel::insert_into(influences::table)
                        .values(influences)
                        .execute(&self.conn)?;
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
    use crate::ports::outbound::public_stash_retriever::{Item, PublicStashData};
    use crate::ports::outbound::repository::ItemRepository;
    use diesel::prelude::*;
    use diesel_logger::LoggingConnection;
    use log::debug;
    use std::env;

    const PUBLIC_STASH_DATA: &str = include_str!("public-stash-tabs.json");

    embed_migrations!("migrations");

    #[test]
    fn insert_item() -> Result<(), anyhow::Error> {
        let conn = SqliteConnection::establish(":memory:")?;
        embedded_migrations::run(&conn)?;

        let mut repo = DieselItemRepository::new(conn)?;
        let stash: PublicStashData = serde_json::from_str(&PUBLIC_STASH_DATA)?;

        let _ = repo.insert_raw_item(stash)?;

        let latest_stash_id = repo.get_stash_id()?;
        assert_eq!(
            latest_stash_id.latest_stash_id.unwrap(),
            "2949-5227-4536-5447-1849"
        );
        Ok(())
    }

    #[test]
    fn insert_remove_stash() -> Result<(), anyhow::Error> {
        let conn = SqliteConnection::establish(":memory:")?;
        embedded_migrations::run(&conn)?;

        let mut repo = DieselItemRepository::new(conn)?;
        let mut stash: PublicStashData = serde_json::from_str(&PUBLIC_STASH_DATA)?;

        let _ = repo.insert_raw_item(stash.clone())?;

        stash.stashes = vec![stash
            .stashes
            .into_iter()
            .filter(|v| v.account_name.is_some())
            .nth(0)
            .unwrap()];
        stash.stashes.get_mut(0).unwrap().items.truncate(3);

        let _ = repo.insert_raw_item(stash.clone())?;

        let items = repo.get_items(
            stash.stashes[0].account_name.as_ref().unwrap(),
            stash.stashes[0].stash.as_ref().unwrap(),
        )?;
        assert_eq!(items.len(), 3);
        Ok(())
    }
}
