use crate::implementations::models::{NewBuild, PobBuild};
use diesel::prelude::*;
use thiserror::Error;
use uuid::Uuid;

use super::models::NewBuildMatch;

#[derive(Error, Debug)]
pub enum BuildsRepositoryError {
    #[error("cant load from db")]
    Repository(#[from] diesel::result::Error),
    #[error("empty response")]
    Empty,
}

pub struct DieselBuildsRepository {
    pub conn: SqliteConnection,
}

impl DieselBuildsRepository {
    pub fn save_new_build(
        &self,
        pob: &str,
        item_set: &str,
    ) -> Result<String, BuildsRepositoryError> {
        use crate::schema::build_info::dsl::*;

        let builds = build_info
            .select(id)
            .filter(pob_url.eq(pob).and(itemset.eq(item_set)))
            .load::<String>(&self.conn)?;

        if builds.len() > 0 {
            return Ok(builds[0].clone());
        }

        let new_build = NewBuild {
            // TODO: change to_string()
            id: Uuid::new_v4().to_hyphenated().to_string(),
            pob_url: pob,
            itemset: item_set,
        };

        diesel::insert_into(build_info)
            .values(&new_build)
            .execute(&self.conn)?;

        Ok(new_build.id.to_owned())
    }

    pub fn get_build(&self, id_: &str) -> Result<PobBuild, BuildsRepositoryError> {
        use crate::schema::build_info::dsl::*;

        let builds = build_info
            .filter(id.eq(id_))
            .limit(1)
            .load::<PobBuild>(&self.conn)?;

        if builds.len() > 0 {
            Ok(builds[0].clone())
        } else {
            Err(BuildsRepositoryError::Empty)
        }
    }

    pub fn update_build(&self, build: &PobBuild) -> Result<(), BuildsRepositoryError> {
        use crate::schema::build_info::dsl::*;

        diesel::update(build_info).set(build).execute(&self.conn)?;

        Ok(())
    }

    pub fn new_build_match(&self, mtch: &NewBuildMatch) -> Result<(), BuildsRepositoryError> {
        use crate::schema::builds_match::dsl::*;

        diesel::insert_into(builds_match)
            .values(mtch)
            .execute(&self.conn)?;

        Ok(())
    }
}
