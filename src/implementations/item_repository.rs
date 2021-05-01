use crate::domain::item::Item as DomainItem;
use crate::ports::outbound::public_stash_retriever::{Extended, Item, PublicStashData};
use crate::ports::outbound::repository::{ItemRepository, LatestStashId, RepositoryError};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::Queryable;
use dotenv::dotenv;
use log::{debug, info, warn};
use std::convert::TryFrom;
use std::env;
use thiserror::Error;
use uuid::Uuid;

struct SplittedItem(
    NewItem,
    // Vec<Mod>,
    // Vec<Subcategory>,
    // Vec<Property>,
    // Vec<Socket>,
    // Vec<UltimatumMod>,
    // Option<IncubatedItem>,
    // Option<Hybrid>,
);

impl TryFrom<Item> for SplittedItem {
    type Error = RepositoryError;
    fn try_from(item: Item) -> Result<Self, Self::Error> {
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
            icon: item.icon,
            support: item.support,
            stack_size: item.stack_size,
            max_stack_size: item.max_stack_size,
            league: item.league,
            id: item.id.unwrap(),
            elder: item.elder,
            shaper: item.shaper,
            abyss_jewel: item.abyss_jewel,
            delve: item.delve,
            fractured: item.fractured,
            synthesised: item.synthesised,
            name: item.name,
            type_line: item.type_line,
            base_type: item.base_type,
            identified: item.identified,
            item_lvl: item.item_level,
            note: item.note,
            duplicated: item.duplicated,
            split: item.split,
            corrupted: item.corrupted,
            talisman_tier: item.talisman_tier,
            sec_descr_text: item.sec_descr_text,
            veiled: item.veiled,
            descr_text: item.descr_text,
            prophecy_text: item.prophecy_text,
            is_relic: item.is_relic,
            replica: item.replica,
            frame_type: item.frame_type,
            x_coordinate: item.x,
            y_coordinate: item.y,
            inventory_id: item.inventory_id,
            socket: item.socket,
            colour: item.colour,
        };
        Ok(SplittedItem(raw))
    }
}
/*
#[derive(Queryable)]
pub struct RawItem {
    id: String,
    base_type: String,
    category: Option<String>,
    prefixes: Option<i32>,
    suffixes: Option<i32>,
    account_id: String,
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
    x: Option<i32>,
    y: Option<i32>,
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
    crusader: Option<bool>,
    hunter: Option<bool>,
    warlord: Option<bool>,
    redeemer: Option<bool>,
    // mods: Vec<Mod>,
    // subcategories: Vec<Subcategory>,
    // properties: Vec<Property>,
    // socketed_items: Vec<RawItem>,
    // sockets: Vec<Socket>,
    // ultimatum_mods: Vec<UltimatumMod>,
    // incubated_items: Vec<IncubatedItem>,
    // hybrids: Vec<Hybrid>,
}

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

use crate::schema::{items, latest_stash_id};

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
#[table_name = "latest_stash_id"]
struct NewLatestStash {
    id: String,
}

#[derive(Identifiable)]
#[table_name = "items"]
#[primary_key(account_id, stash_id)]
struct RemoveItems<'a> {
    account_id: &'a String,
    stash_id: &'a String,
}

pub struct DieselItemRepository {
    conn: SqliteConnection,
}

use crate::schema::items::dsl as items_dsl;
use crate::schema::latest_stash_id::dsl as stash_dsl;

impl DieselItemRepository {
    pub fn new(connection: SqliteConnection) -> Result<DieselItemRepository, RepositoryError> {
        Ok(DieselItemRepository { conn: connection })
    }
}

impl ItemRepository for DieselItemRepository {
    fn get_stash_id(&self) -> Result<LatestStashId, RepositoryError> {
        let v = stash_dsl::latest_stash_id.load::<LatestStashId>(&self.conn)?;
        Ok(v.into_iter()
            .nth(0)
            .or(Some(LatestStashId::default()))
            .unwrap())
    }

    fn insert_raw_item(&self, public_data: PublicStashData) -> Result<(), RepositoryError> {
        self.conn.transaction::<_, RepositoryError, _>(|| {
            let new_item_info: Vec<SplittedItem> = public_data
                .stashes
                .iter()
                .map(|v| {
                    v.items
                        .iter()
                        .map(|i| match SplittedItem::try_from(i.clone()) {
                            Ok(mut item) => {
                                item.0.account_id = v.id.clone();
                                item.0.account_name = v.account_name.as_ref().cloned().unwrap();
                                item.0.stash_id = v.stash.as_ref().cloned().unwrap();
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
                .collect();

            // TODO: implement stash unlist
            let insert_items: Vec<&NewItem> = new_item_info.iter().map(|v| &v.0).collect();
            let delete_items: Vec<RemoveItems> = new_item_info
                .iter()
                .map(|v| RemoveItems {
                    account_id: &v.0.account_id,
                    stash_id: &v.0.stash_id,
                })
                .collect();

            let latest_stash = NewLatestStash {
                id: public_data.next_change_id,
            };

            // workaround for upsert functionality for sqlite https://github.com/diesel-rs/diesel/issues/1854
            let vals = stash_dsl::latest_stash_id.load::<LatestStashId>(&self.conn)?;
            if vals.len() == 0 {
                diesel::insert_into(latest_stash_id::table)
                    .values(&latest_stash)
                    .execute(&self.conn)?;
            } else {
                diesel::update(latest_stash_id::table)
                    .set(stash_dsl::id.eq(&latest_stash.id))
                    .execute(&self.conn)?;
            }

            // TODO: somehow use Identifiable or smth else to simplify delete query
            for i in &delete_items {
                diesel::delete(
                    items_dsl::items.filter(
                        items_dsl::account_id
                            .eq(i.account_id)
                            .and(items_dsl::stash_id.eq(i.stash_id)),
                    ),
                )
                .execute(&self.conn)?;
            }

            diesel::insert_into(items::table)
                .values(insert_items)
                .execute(&self.conn)?;

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
}
