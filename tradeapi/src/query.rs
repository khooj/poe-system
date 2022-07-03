use derivative::Derivative;
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
struct StatQuery {
    disabled: bool,
    typ: StatQueryType,
    filters: Vec<StatQueryFilter>,
}

#[derive(Serialize, Derivative)]
#[serde(rename_all = "lowercase")]
#[derivative(Default)]
enum StatQueryType {
    #[derivative(Default)]
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

#[derive(Serialize, Derivative)]
#[serde(rename_all = "lowercase")]
#[derivative(Default)]
enum StatusOption {
    #[derivative(Default)]
    Online,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "lowercase")]
struct SortOptions {
    price: Sort,
}

#[derive(Serialize, Derivative)]
#[serde(rename_all = "lowercase")]
#[derivative(Default)]
enum Sort {
    #[derivative(Default)]
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

    pub fn build(self) -> Result<String, BuilderError> {
        Ok(serde_json::to_string(&self)?)
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

    pub fn try_add_raw_mod(
        mut self,
        text: &str,
        min: Option<i32>,
        max: Option<i32>,
    ) -> Result<Self, BuilderError> {
        Err(BuilderError::UnknownMod(text.to_owned()))
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
            .try_add_raw_mod("Energy Shield", None, None)
            .unwrap()
            .try_add_raw_mod("Life", Some(10), Some(45))
            .unwrap()
            .end()
            .new_stat_group()
            .try_add_raw_mod("Attack Speed", None, None)
            .unwrap()
            .end()
            .build();
    }
}
