use serde::Serialize;
use thiserror::Error;

use super::dist::{STATS_IDS, STAT_TO_ID};

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
struct StatQuery {
    disabled: bool,
    typ: StatQueryType,
    filters: Vec<StatQueryFilter>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
enum StatQueryType {
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
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
struct Query {
    stats: Vec<StatQuery>,
    status: Status,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
struct Status {
    option: StatusOption,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
enum StatusOption {
    #[default]
    Online,
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

    pub fn new_stat_group(self) -> StatGroupBuilder {
        StatGroupBuilder {
            builder: self,
            query: StatQuery::default(),
        }
    }
}

pub struct StatGroupBuilder {
    builder: Builder,
    query: StatQuery,
}

impl StatGroupBuilder {
    pub fn count(mut self, v: i32) -> Self {
        self.query.typ = StatQueryType::Count(v);
        self
    }

    pub fn and(mut self) -> Self {
        self.query.typ = StatQueryType::And;
        self
    }

    pub fn end(mut self) -> Builder {
        self.builder.query.stats.push(self.query);
        self.builder
    }

    pub fn try_add_mod(
        self,
        text: &str,
        min: Option<i32>,
        max: Option<i32>,
    ) -> Result<Self, BuilderError> {
        if !STAT_TO_ID.contains_key(text) {
            Err(BuilderError::UnknownMod(text.to_string()))
        } else {
            let id = STAT_TO_ID.get(text).unwrap().clone();
            let s = self
                .try_add_mod_id(&id, min, max)
                .expect("should work after check");
            Ok(s)
        }
    }

    pub fn try_add_mod_id(
        mut self,
        text: &str,
        min: Option<i32>,
        max: Option<i32>,
    ) -> Result<Self, BuilderError> {
        if !STATS_IDS.contains(&text) {
            Err(BuilderError::UnknownMod(text.to_string()))
        } else {
            self.query.filters.push(StatQueryFilter {
                id: text.to_string(),
                disabled: false,
                value: StatQueryValues { max, min },
            });
            Ok(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Builder;

    #[test]
    fn usage() {
        let query = Builder::new()
            .new_stat_group()
            .count(2)
            .try_add_mod("Cannot Leech Energy Shield", None, None)
            .unwrap()
            .try_add_mod_id("ultimatum.umod_7052", None, None)
            .unwrap()
            .try_add_mod(
                "Grants # to Maximum Life per 2% Quality",
                Some(10),
                Some(45),
            )
            .unwrap()
            .end()
            .new_stat_group()
            .try_add_mod("#% increased Attack Speed per 8% Quality", None, None)
            .unwrap()
            .end();
    }
}
