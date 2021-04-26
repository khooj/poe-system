use thiserror::Error;
use sqlx::query_as;
use crate::domain::item::{Item as DomainItem};

pub struct RawItem {
    id: String,
    base_type: String,
    category: Option<String>,
    prefixes: Option<i64>,
    suffixes: Option<i64>,
    account_id: String,
    stash_id: String,
    league: Option<String>,
    name: String,
    item_lvl: i64,
    identified: bool,
    inventory_id: Option<String>,
    type_line: String,
    abyss_jewel: Option<bool>,
    corrupted: Option<bool>,
    duplicated: Option<bool>,
    elder: Option<bool>,
    frame_type: i64,
    h: i64,
    w: i64,
    x: Option<i64>,
    y: Option<i64>,
    is_relic: Option<bool>,
    note: Option<String>,
    shaper: Option<bool>,
    stack_size: Option<i64>,
    max_stack_size: Option<i64>,
    support: Option<bool>,
    talisman_tier: Option<i64>,
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
    socket: Option<i64>,
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

#[derive(sqlx::Type)]
#[repr(i64)]
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
    #[error("sqlx error")]
    SqlxError(#[from] sqlx::Error),
    #[error("t")]
    Ttt,
}

pub struct SqlxItemRepository {

}

impl SqlxItemRepository {
    pub async fn get_item(name: &str) -> Result<DomainItem, RepositoryError> {
        let mut pool = sqlx::SqlitePool::connect_lazy("sqlite:main.db")?;
        let rawItem = query_as!(
            RawItem,
            r#"
            SELECT * FROM items WHERE name = ?
            "#,
            name
        ).fetch_one(&pool)
        .await?;
        let mods = query_as!(
            Mod,
            r#"
            SELECT item_id, type as "type:_", mod FROM mods WHERE item_id = ?
            "#,
            rawItem.id)
        .fetch_all(&pool)
        .await?;
        let props = query_as!(
            Property,
            r#"
            SELECT * FROM properties WHERE item_id = ?"#,
            rawItem.id)
        .fetch_all(&pool)
        .await?;

        Err(RepositoryError::Ttt)
    }
}