use macros::{gen_min_max_method, gen_option_method};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use thiserror::Error;

use crate::dist::{STATS_IDS, STAT_TO_ID};

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub struct StatQuery {
    disabled: bool,
    #[serde(rename = "type")]
    typ: StatQueryType,
    filters: Vec<StatQueryFilter>,
}

impl StatQuery {
    pub fn new() -> Self {
        StatQuery::default()
    }

    pub fn set_type(mut self, typ: StatQueryType) -> Self {
        self.typ = typ;
        self
    }

    pub fn try_add_mod(
        self,
        text: &str,
        min: Option<i32>,
        max: Option<i32>,
        option: Option<i32>,
    ) -> Result<Self, BuilderError> {
        if !STAT_TO_ID.contains_key(text) {
            Err(BuilderError::UnknownMod(text.to_string()))
        } else {
            let id = STAT_TO_ID.get(text).unwrap().to_owned();
            let s = self.try_add_mod_id(id, min, max, option)?;
            Ok(s)
        }
    }

    pub fn try_add_mod_id(
        mut self,
        text: &str,
        min: Option<i32>,
        max: Option<i32>,
        option: Option<i32>,
    ) -> Result<Self, BuilderError> {
        if !STATS_IDS.contains(&text) {
            Err(BuilderError::UnknownMod(text.to_string()))
        } else {
            self.filters.push(StatQueryFilter {
                id: text.to_string(),
                disabled: false,
                value: StatQueryValues { max, min, option },
            });
            Ok(self)
        }
    }
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum StatQueryType {
    #[default]
    And,
    Count(i32),
    Not,
    If,
    WeightedSum,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
struct StatQueryFilter {
    disabled: bool,
    id: String,
    value: StatQueryValues,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
struct StatQueryValues {
    min: Option<i32>,
    max: Option<i32>,
    option: Option<i32>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
struct Filters {
    #[serde(skip_serializing_if = "Option::is_none")]
    type_filters: Option<TypeFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weapon_filters: Option<WeaponFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    socket_filters: Option<SocketFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    misc_filters: Option<MiscFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    armour_filters: Option<ArmourFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    req_filters: Option<ReqFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trade_filters: Option<TradeFilters>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub struct TypeFilters {
    filters: HashMap<String, Value>,
}

impl TypeFilters {
    pub fn set_category(mut self, s: &str) -> Self {
        let v = json!({
            "option": s,
        });
        let m = self.filters.entry("category".to_string()).or_default();
        *m = v;
        self
    }

    pub fn set_rarity(mut self, s: &str) -> Self {
        let v = json!({
            "option": s,
        });
        let m = self.filters.entry("rarity".to_string()).or_default();
        *m = v;
        self
    }
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub struct WeaponFilters {
    filters: HashMap<String, Value>,
    disabled: bool,
}

impl WeaponFilters {
    gen_min_max_method!(damage);
    gen_min_max_method!(crit);
    gen_min_max_method!(pdps);
    gen_min_max_method!(aps);
    gen_min_max_method!(dps);
    gen_min_max_method!(edps);
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub struct SocketFilters {
    disabled: bool,
    filters: HashMap<String, Value>,
}

impl SocketFilters {
    pub fn set_sockets(
        mut self,
        min: Option<&usize>,
        max: Option<&usize>,
        r: Option<&usize>,
        g: Option<&usize>,
        b: Option<&usize>,
        w: Option<&usize>,
    ) -> Self {
        let v = json!({
            "min": min,
            "max": max,
            "r": r,
            "g": g,
            "b": b,
            "w": w,
        });
        let m = self.filters.entry("sockets".to_string()).or_default();
        *m = v;
        self
    }

    pub fn set_links(
        mut self,
        min: Option<usize>,
        max: Option<usize>,
        r: Option<usize>,
        g: Option<usize>,
        b: Option<usize>,
        w: Option<usize>,
    ) -> Self {
        let v = json!({
            "min": min,
            "max": max,
            "r": r,
            "g": g,
            "b": b,
            "w": w,
        });
        let m = self.filters.entry("links".to_string()).or_default();
        *m = v;
        self
    }
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub struct MiscFilters {
    filters: HashMap<String, Value>,
}

impl MiscFilters {
    gen_option_method!(corrupted);
    gen_min_max_method!(quality);
    gen_min_max_method!(gem_level);
    gen_min_max_method!(ilvl);
    gen_min_max_method!(gem_level_progress);
    gen_min_max_method!(gem_alternative_quality);
    gen_option_method!(fractured_item);
    gen_option_method!(searing_item);
    gen_option_method!(split);
    gen_option_method!(veiled);
    gen_min_max_method!(scourge_tier);
    gen_min_max_method!(stored_experience);
    gen_option_method!(synthesised_item);
    gen_option_method!(tangled_item);
    gen_option_method!(identified);
    gen_option_method!(mirrored);
    gen_option_method!(crafted);
    gen_option_method!(enchanted);
    gen_min_max_method!(talisman_tier);
    gen_min_max_method!(stack_size);
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub struct ArmourFilters {
    disabled: bool,
    filters: HashMap<String, Value>,
}

impl ArmourFilters {
    gen_min_max_method!(ar);
    gen_min_max_method!(es);
    gen_min_max_method!(block);
    gen_min_max_method!(ev);
    gen_min_max_method!(ward);
    gen_min_max_method!(base_defence_percentile);
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub struct ReqFilters {
    disabled: bool,
    filters: HashMap<String, Value>,
}

impl ReqFilters {
    gen_min_max_method!(lvl);
    gen_min_max_method!(dex);
    gen_min_max_method!(str);
    gen_min_max_method!(int);

    fn set_class(mut self, class: &str) -> Self {
        let v = json!({
            "option": class,
        });
        let m = self.filters.entry(class.to_string()).or_default();
        *m = v;
        self
    }
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub struct TradeFilters {
    disabled: bool,
    filters: HashMap<String, Value>,
}

impl TradeFilters {
    pub fn set_account(mut self, acc: &str) -> Self {
        let v = json!({
            "input": acc,
        });
        self.filters.insert("account".to_string(), v);
        self
    }

    pub fn set_sale_type(mut self, opt: &str) -> Self {
        let v = json!({
            "option": opt,
        });
        self.filters.insert("sale_type".to_string(), v);
        self
    }
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
struct Query {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    stats: Vec<StatQuery>,
    filters: Filters,
    status: Status,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    typ: Option<String>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
struct Status {
    option: StatusOption,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum StatusOption {
    #[default]
    Online,
    Offline,
    Any,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
struct SortOptions {
    price: Sort,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
enum Sort {
    #[default]
    Asc,
    Desc,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub struct Builder {
    query: Query,
    sort: SortOptions,
}

#[derive(Error, Debug)]
pub enum BuilderError {
    #[error("unknown mod: {0}")]
    UnknownMod(String),
    #[error("build error: {0}")]
    BuildError(#[from] serde_json::Error),
}

impl Builder {
    pub fn new() -> Builder {
        Builder::default()
    }

    pub fn set_type_filters(&mut self, filters: TypeFilters) {
        self.query.filters.type_filters = Some(filters);
    }

    pub fn set_weapon_filters(&mut self, filters: WeaponFilters) {
        self.query.filters.weapon_filters = Some(filters);
    }

    pub fn set_socket_filters(&mut self, filters: SocketFilters) {
        self.query.filters.socket_filters = Some(filters);
    }

    pub fn set_misc_filters(&mut self, filters: MiscFilters) {
        self.query.filters.misc_filters = Some(filters);
    }

    pub fn set_armour_filters(&mut self, filters: ArmourFilters) {
        self.query.filters.armour_filters = Some(filters);
    }

    pub fn set_req_filters(&mut self, filters: ReqFilters) {
        self.query.filters.req_filters = Some(filters);
    }

    pub fn add_stat_group(&mut self, group: StatQuery) {
        self.query.stats.push(group);
    }

    pub fn set_type(&mut self, s: &str) {
        self.query.typ = Some(s.to_string());
    }

    pub fn set_status(&mut self, status: StatusOption) {
        self.query.status.option = status;
    }

    pub fn set_trade_filters(&mut self, filters: TradeFilters) {
        self.query.filters.trade_filters = Some(filters);
    }
}

/*
{
    "query": {
        "status": {
            "option": "online"
        },
        "stats": [
            {
                "type": "and",
                "filters": []
            }
        ],
        "filters": {
            "type_filters": {
                "filters": {
                    "category": {
                        "option": "weapon.one"
                    },
                    "rarity": {
                        "option": "magic"
                    }
                }
            },
            "weapon_filters": {
                "disabled": false,
                "filters": {
                    "damage": {
                        "min": 5
                    },
                    "crit": {
                        "min": 5
                    },
                    "pdps": {
                        "min": 5
                    },
                    "aps": {
                        "min": 5
                    },
                    "dps": {
                        "min": 5
                    },
                    "edps": {
                        "min": 5
                    }
                }
            },
            "socket_filters": {
                "disabled": false,
                "filters": {
                    "sockets": {
                        "r": 1,
                        "g": 1,
                        "b": 1,
                        "w": 1,
                        "min": 1,
                        "max": 2
                    },
                    "links": {
                        "min": 1,
                        "r": 1,
                        "g": 1,
                        "b": 1,
                        "w": 1,
                        "max": 2
                    }
                }
            },
            "misc_filters": {
                "filters": {
                    "corrupted": {
                        "option": "true"
                    },
                    "quality": {
                        "min": 1
                    },
                    "gem_level": {
                        "min": 1
                    },
                    "ilvl": {
                        "min": 1
                    },
                    "gem_level_progress": {
                        "min": 1
                    },
                    "gem_alternate_quality": {
                        "option": "1"
                    },
                    "fractured_item": {
                        "option": "true"
                    },
                    "searing_item": {
                        "option": "true"
                    },
                    "split": {
                        "option": "true"
                    },
                    "veiled": {
                        "option": "true"
                    },
                    "scourge_tier": {
                        "min": 1,
                        "max": 2
                    },
                    "stored_experience": {
                        "min": 1,
                        "max": 2
                    },
                    "synthesised_item": {
                        "option": "true"
                    },
                    "tangled_item": {
                        "option": "true"
                    },
                    "identified": {
                        "option": "true"
                    },
                    "mirrored": {
                        "option": "true"
                    },
                    "crafted": {
                        "option": "true"
                    },
                    "enchanted": {
                        "option": "true"
                    },
                    "talisman_tier": {
                        "min": 1,
                        "max": 2
                    },
                    "stack_size": {
                        "min": 1,
                        "max": 2
                    }
                }
            },
            "armour_filters": {
                "disabled": false,
                "filters": {
                    "ar": {
                        "min": 1
                    },
                    "es": {
                        "min": 1
                    },
                    "block": {
                        "min": 1
                    },
                    "ev": {
                        "min": 1
                    },
                    "ward": {
                        "min": 1
                    },
                    "base_defence_percentile": {
                        "min": 1
                    }
                }
            },
            "req_filters": {
                "disabled": false,
                "filters": {
                    "lvl": {
                        "min": 1
                    },
                    "dex": {
                        "min": 1
                    },
                    "class": {
                        "option": "scion"
                    },
                    "str": {
                        "min": 1
                    },
                    "int": {
                        "min": 1
                    }
                }
            }
        }
    },
    "sort": {
        "price": "asc"
    }
}
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usage() {
        let mut query = Builder::new();
        query.add_stat_group(
            StatQuery::new()
                .set_type(StatQueryType::Count(2))
                .try_add_mod("Cannot Leech Energy Shield", None, None, None)
                .expect("err")
                .try_add_mod_id("ultimatum.umod_7052", None, None, None)
                .expect("err2"),
        );
        query.set_socket_filters(SocketFilters::default().set_links(
            Some(1),
            None,
            None,
            None,
            None,
            None,
        ));
    }
}
