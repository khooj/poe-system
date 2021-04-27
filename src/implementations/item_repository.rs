use crate::domain::item::Item as DomainItem;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::Queryable;
use dotenv::dotenv;
use std::env;
use thiserror::Error;

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
    item_lvl: i32,
    identified: bool,
    inventory_id: Option<String>,
    type_line: String,
    abyss_jewel: Option<bool>,
    corrupted: Option<bool>,
    duplicated: Option<bool>,
    elder: Option<bool>,
    frame_type: i32,
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
    verified: Option<bool>,
    icon: Option<String>,
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

#[repr(C)]
pub enum PropertyType {
    Properties = 0,
    Requirements,
    AdditionalProperties,
    NextLevelRequirements,
    NotableProperties,
    Hybrid,
}

#[repr(C)]
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

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("orm error")]
    OrmError(#[from] diesel::result::ConnectionError),
    #[error("query error")]
    QueryError(#[from] diesel::result::Error),
    #[error("t")]
    Ttt,
}

use crate::schema::items;

#[derive(Insertable)]
#[table_name = "items"]
pub struct NewItem<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

pub struct DieselItemRepository {
    db_url: String,
    conn: SqliteConnection,
}

use crate::schema::items::dsl::*;
impl DieselItemRepository {
    pub fn new() -> Result<DieselItemRepository, RepositoryError> {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = SqliteConnection::establish(&db_url)?;
        Ok(DieselItemRepository { db_url, conn })
    }

    pub fn get_item(&self, search_name: &str) -> Result<DomainItem, RepositoryError> {
        let result = items
            .filter(name.eq(&search_name))
            .limit(5)
            .load::<RawItem>(&self.conn)?;

        Ok(DomainItem::empty())
    }

    pub fn insert_item(&self, item: &DomainItem) -> Result<(), RepositoryError> {
        let new_item = NewItem {
            id: &item.id,
            name: &item.name,
        };

        diesel::insert_into(items::table)
            .values(&new_item)
            .execute(&self.conn)
            .expect("not ok");

        Ok(())
    }
}
